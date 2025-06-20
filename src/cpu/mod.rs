use crate::cpu::instructions::{AddressingMode, Instruction, InstructionType};
use crate::error::{MemoryError, MyGetCpuError, MyTickError};
use crate::memory::Memory;
use crate::MainError;
use debug::DebugMode;
use interrupt_handler::InterruptState;
use log::warn;
use registers::{CpuRegister, ProgramCounter, StatusRegister, StatusRegisterBit};
use tudelft_nes_ppu::{Cpu as CpuTemplate, Ppu};
use tudelft_nes_test::TestableCpu;
pub(crate) mod debug;
mod instructions;
mod interrupt_handler;
mod registers;

struct OperandValue {
    value: Option<u8>,
    address: Option<u16>,
}

#[derive(Debug)]
pub struct Cpu {
    accumulator: CpuRegister,
    x_register: CpuRegister,
    y_register: CpuRegister,
    stack_pointer: CpuRegister,
    program_counter: ProgramCounter,
    status_register: StatusRegister,
    current_instruction: Instruction,
    current_cycle: u8,
    instruction_cycle_count: u8,
    interrupt_polling_cycle: u8,
    interrupt_state: InterruptState,
    nmi_line_prev: bool,
    nmi_line_current: bool,
    nmi_line_triggered: bool,
    irq_line_triggered: bool,
    branch_success: bool,
    page_crossing: bool,
    memory: Memory,
    total_cycles: u64,
    instructions_executed: u64,
    debug: DebugMode,
}

/// Trait for making the CPU testable in automated tests
impl TestableCpu for Cpu {
    type GetCpuError = MyGetCpuError;

    fn get_cpu(_rom: &[u8]) -> Result<Self, MyGetCpuError> {
        Ok(Cpu {
            accumulator: CpuRegister::default(),
            x_register: CpuRegister::default(),
            y_register: CpuRegister::default(),
            stack_pointer: CpuRegister::default(),
            program_counter: ProgramCounter::new(),
            status_register: StatusRegister::default(),
            current_instruction: Instruction {
                instruction_type: InstructionType::NOP,
                addressing_mode: AddressingMode::Absolute,
            },
            current_cycle: 1,
            instruction_cycle_count: 0,
            interrupt_polling_cycle: 0,
            interrupt_state: InterruptState::Uninitialized,
            nmi_line_prev: false,
            nmi_line_current: false,
            nmi_line_triggered: false,
            irq_line_triggered: false,
            branch_success: false,
            page_crossing: false,
            total_cycles: 0,
            instructions_executed: 0,
            debug: DebugMode::No,
            memory: Memory::new(_rom)?,
        })
    }

    fn set_program_counter(&mut self, _value: u16) {
        self.program_counter.set(_value);
    }

    fn memory_read(&self, _address: u16) -> u8 {
        self.memory
            .read_cpu_mem(_address)
            .expect("Could not read from memory")
    }
}

/// Trait for the CPU to be used by the PPU.
impl CpuTemplate for Cpu {
    type TickError = MyTickError;

    // Only needed when the specific mapper you implement has character RAM, writable memory on the
    // cartridge. Most games don’t require this. If you just don’t implement this method it will
    // default to ignoring all writes (as if there was only character ROM, not RAM)
    fn ppu_memory_write(&mut self, _address: u16, _value: u8) {
        if let Err(x) = self.memory.write_ppu_byte(_address, _value) {
            warn!("Ppu write error: {}", x);
        }
    }

    // Called every cpu cycle. Note that some instructions take multiple cycles, which is important
    // for some games to work properly. That means that it won’t work to execute an entire instruction
    // every time tick is called. It should take multiple calls to tick to execute one instruction.
    fn tick(&mut self, ppu: &mut Ppu) -> Result<(), MyTickError> {
        // set the cpu to the startup state fi
        if self.current_cycle == self.interrupt_polling_cycle {
            // this line is for interrupt hijacking to be working later
            let current_interrupt = self.poll_interrupts();
            if current_interrupt == InterruptState::IRQ
                && self.status_register.get_bit(StatusRegisterBit::Interrupt)
            {
                self.interrupt_state = InterruptState::NormalOperation;
            } else {
                self.interrupt_state = current_interrupt;
            }
        }
        // execute interrupt or opcode
        if self.current_cycle > self.instruction_cycle_count {
            self.current_cycle = 1;

            match self.interrupt_state {
                InterruptState::Uninitialized => {
                    log::debug!("Initializing CPU");
                    self.initialize_cpu(ppu)?;
                    log::debug!("CPU initialized\n\n");
                    self.print_cpu_state_header();
                    self.interrupt_state = InterruptState::Booting;
                }
                InterruptState::Booting => {
                    if self.total_cycles == 6 {
                        self.interrupt_state = InterruptState::NormalOperation;
                    }
                }
                InterruptState::NMI => {
                    log::debug!("Executing NMI");
                    self.push_pc_and_status_on_stack(ppu)?;
                    let nmi_lobyte = self.memory.read(0xFFFA, self, ppu)?;
                    let nmi_hibyte = self.memory.read(0xFFFB, self, ppu)?;
                    self.program_counter.set_lobyte(nmi_lobyte);
                    self.program_counter.set_hibyte(nmi_hibyte);

                    self.instruction_cycle_count = 7;
                    self.interrupt_state = InterruptState::NormalOperation;
                    self.interrupt_polling_cycle = 0;
                    // TODO: there is conflicting info on masswerk and nesdev whether this line should happen
                    // self.status_register
                    //     .set_bit(StatusRegisterBit::InterruptBit, true);
                }
                InterruptState::IRQ => {
                    warn!("Add interface for IRQ")
                }
                InterruptState::NormalOperation => {
                    log::debug!("\n\n---------------");
                    self.debug(self.memory.read(self.program_counter.get(), self, ppu)?);
                    let opcode = self.read_next_value(ppu)?;
                    log::debug!("Opcode: {:02X}", opcode);
                    let instruction: Instruction =
                        Instruction::decode(opcode).expect("Failed decoding opcode");
                    instruction.execute(self, ppu)?;

                    self.instruction_cycle_count = Instruction::get_instruction_duration(opcode)?;
                    log::debug!(
                        "Instruction cycle count set to {}",
                        self.instruction_cycle_count,
                    );

                    if !instruction.is_rmw() && self.page_crossing {
                        self.instruction_cycle_count += 1;
                    }

                    // make sure the interrupts are polled before the second cycle of the conditional branch operations
                    // it could still be wrong, i dont understand this part on nesdev
                    self.interrupt_polling_cycle = self.instruction_cycle_count - 1;

                    if self.branch_success {
                        self.instruction_cycle_count += 1;
                        self.branch_success = false;
                    }

                    self.page_crossing = false;
                    self.current_instruction = instruction;
                    self.instructions_executed += 1;
                    self.print_cpu_state_header();
                }
            }
        }

        // interrupt hijacking, if an interrupt arrives in the first four cycles of a BRK
        if self.current_instruction.instruction_type == InstructionType::BRK
            && self.current_cycle < 4
        {
            let current_interrupt = self.poll_interrupts();
            match current_interrupt {
                InterruptState::NMI => {
                    let nmi_lobyte = self.memory.read(0xFFFA, self, ppu)?;
                    let nmi_hibyte = self.memory.read(0xFFFB, self, ppu)?;
                    self.program_counter.set_lobyte(nmi_lobyte);
                    self.program_counter.set_hibyte(nmi_hibyte);

                    self.instruction_cycle_count = 7;
                    self.interrupt_state = InterruptState::NormalOperation;
                    self.interrupt_polling_cycle = 0;
                }
                InterruptState::IRQ => {
                    if !self.status_register.get_bit(StatusRegisterBit::Interrupt) {
                        let nmi_lobyte = self.memory.read(0xFFFE, self, ppu)?;
                        let nmi_hibyte = self.memory.read(0xFFFF, self, ppu)?;
                        self.program_counter.set_lobyte(nmi_lobyte);
                        self.program_counter.set_hibyte(nmi_hibyte);

                        self.instruction_cycle_count = 7;
                        self.interrupt_state = InterruptState::NormalOperation;
                        self.interrupt_polling_cycle = 0;
                    }
                }
                _ => (),
            }
        }

        if self.nmi_line_current && !self.nmi_line_prev {
            self.nmi_line_triggered = true;
        }
        self.print_cpu_state();
        self.current_cycle += 1;
        self.total_cycles += 1;
        self.nmi_line_prev = self.nmi_line_current;
        self.nmi_line_current = false;

        Ok(())
    }

    // This method is called when the PPU (implemented by us) wants to read a byte from memory.
    // The byte that is actually read, may depend on the current mapper state. Since you implement
    // the mapper, you should make sure the correct byte is returned here.
    fn ppu_read_chr_rom(&self, _offset: u16) -> u8 {
        self.memory
            .read_ppu_byte(_offset)
            .expect("Failed reading character ROM")
    }

    // Sometimes the PPU needs to give a non-maskable interrupt to the cpu. If it does, this method
    // is called by the PPU.
    fn non_maskable_interrupt(&mut self) {
        self.on_non_maskable_interrupt();
    }
}

impl Cpu {
    // Get instruction length of an addressing mode
    fn addressing_mode_get_bytes(&self, addressing_mode: &AddressingMode) -> Vec<u8> {
        let length = addressing_mode.length() as u16;
        (0..length)
            .map(|n| self.memory_read(self.program_counter.get() + n))
            .collect::<Vec<_>>()
    }

    // Print debug information in a format that can be compared with other
    // emulators
    fn debug(&self, opcode: u8) {
        if self.debug == DebugMode::Emu {
            if let Ok(instruction) = Instruction::decode(opcode) {
                let raw_bytes = self.addressing_mode_get_bytes(&instruction.addressing_mode);
                let bytes = raw_bytes
                    .iter()
                    .map(|arg| format!("{:02X}", arg))
                    .collect::<Vec<_>>()
                    .join(" ");

                println!(
                    "{:04X}  {:8}  {:32?} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC:{}",
                    self.program_counter.get(),
                    bytes,
                    instruction.instruction_type,
                    self.accumulator.get(),
                    self.x_register.get(),
                    self.y_register.get(),
                    self.status_register.get() & !(1 << 4),
                    self.stack_pointer.get(),
                    self.total_cycles,
                );
            }
        }
    }

    fn print_cpu_state_header(&self) {
        log::debug!("A |X |Y |SP |PC   |T/MT |NV-BDIZC |Instr# |CYCLE");
        log::debug!("----------------------------------------");
    }

    fn print_cpu_state(&self) {
        log::debug!(
            "{:02X}|{:02X}|{:02X}|{:02X} |{:04X} |{}/{}  |{:08b} |{}      |{}",
            self.accumulator.get(),
            self.x_register.get(),
            self.y_register.get(),
            self.stack_pointer.get(),
            self.program_counter.get(),
            self.current_cycle,
            self.instruction_cycle_count,
            self.status_register.get(),
            self.instructions_executed,
            self.total_cycles,
        );
    }

    // Get operand value and address of an instruction.
    //
    // If an instruction is write only it will not read the value.
    fn get_operand_value(
        &mut self,
        addressing_mode: &AddressingMode,
        ppu: &mut Ppu,
        write_only: bool,
    ) -> Result<OperandValue, MainError> {
        let mut hh: u8 = 0;
        let mut ll: u8 = 0;

        match addressing_mode.length() {
            1 => (),
            2 => {
                ll = self.read_next_value(ppu)?;
                log::debug!("ll: {:02X}", ll);
            }
            3 => {
                ll = self.read_next_value(ppu)?;
                hh = self.read_next_value(ppu)?;
                log::debug!("ll: {:02X} hh: {:02X}", ll, hh);
            }
            _ => warn!("Unknown addressing mode"),
        }
        match addressing_mode {
            // A	        Accumulator	            OPC A	        operand is AC (implied single byte instruction)
            AddressingMode::Accumulator => Ok(OperandValue {
                value: Some(self.accumulator.get()),
                address: None,
            }),

            // abs	        absolute	            OPC $LLHH	    operand is address $HHLL *
            AddressingMode::Absolute => {
                let address: u16 = (hh as u16) << 8 | ll as u16;
                match write_only {
                    true => Ok(OperandValue {
                        address: Some(address),
                        value: None,
                    }),
                    false => Ok(OperandValue {
                        address: Some(address),
                        value: Some(self.memory.read(address, self, ppu)?),
                    }),
                }
            }

            // abs,X	    absolute, X-indexed	    OPC $LLHH,X	    operand is address; effective address is address incremented by X with carry **
            AddressingMode::AbsoluteX => {
                let address: u16 = (hh as u16) << 8 | ll as u16;
                let new_address = address.wrapping_add(self.x_register.get() as u16);
                if ((new_address & 0x0100) ^ (address & 0x0100)) == 0x0100 {
                    self.page_crossing = true;
                }
                Ok(OperandValue {
                    address: Some(new_address),
                    value: Some(self.memory.read(new_address, self, ppu)?),
                })
            }

            // abs,Y	    absolute, Y-indexed	    OPC $LLHH,Y	    operand is address; effective address is address incremented by Y with carry **
            AddressingMode::AbsoluteY => {
                let address: u16 = (hh as u16) << 8 | ll as u16;
                let new_address = address.wrapping_add(self.y_register.get() as u16);
                if ((new_address & 0x0100) ^ (address & 0x0100)) == 0x0100 {
                    self.page_crossing = true;
                }
                Ok(OperandValue {
                    address: Some(new_address),
                    value: Some(self.memory.read(new_address, self, ppu)?),
                })
            }

            // #	        immediate	            OPC #$BB	    operand is byte BB
            AddressingMode::Immediate => Ok(OperandValue {
                value: Some(ll),
                address: None,
            }),

            // impl	        implied	                OPC	            operand implied
            AddressingMode::Implied => Ok(OperandValue {
                value: None,
                address: None,
            }),

            // ind	        indirect	            OPC ($LLHH)	    operand is address; effective address is contents of word at address: C.w($HHLL)
            AddressingMode::Indirect => {
                let address: u16 = (hh as u16) << 8 | ll as u16;
                let address_plus_one = (hh as u16) << 8 | ll.wrapping_add(1) as u16;
                let memory_ll: u8 = self.memory.read(address, self, ppu)?;
                let memory_hh: u8 = self.memory.read(address_plus_one, self, ppu)?;
                let memory_address: u16 = (memory_hh as u16) << 8 | memory_ll as u16;
                Ok(OperandValue {
                    address: Some(memory_address),
                    value: Some(self.memory.read(memory_address, self, ppu)?),
                })
            }

            // X,ind	    X-indexed, indirect	    OPC ($LL,X)	    operand is zeropage address; effective address is word in (LL + X, LL + X + 1), inc. without carry: C.w($00LL + X)
            AddressingMode::IndirectX => {
                let address: u16 = ll.wrapping_add(self.x_register.get()) as u16;
                let address_plus_one =
                    ll.wrapping_add(self.x_register.get()).wrapping_add(1) as u16;
                let memory_ll: u8 = self.memory.read(address, self, ppu)?;
                let memory_hh: u8 = self.memory.read(address_plus_one, self, ppu)?;
                let memory_address: u16 = (memory_hh as u16) << 8 | memory_ll as u16;
                Ok(OperandValue {
                    address: Some(memory_address),
                    value: Some(self.memory.read(memory_address, self, ppu)?),
                })
            }

            // ind,Y	    indirect, Y-indexed	    OPC ($LL),Y	    operand is zeropage address; effective address is word in (LL, LL + 1) incremented by Y with carry: C.w($00LL) + Y
            AddressingMode::IndirectY => {
                let address: u16 = ll as u16;
                let address_plus_one = ll.wrapping_add(1) as u16;
                let memory_ll: u8 = self.memory.read(address, self, ppu)?;
                let memory_hh: u8 = self.memory.read(address_plus_one, self, ppu)?;
                let address_non_incremented = (memory_hh as u16) << 8 | memory_ll as u16;
                let address_incremented: u16 =
                    address_non_incremented.wrapping_add(self.y_register.get().into());
                if ((address_incremented & 0x0100) ^ (address_non_incremented & 0x0100)) == 0x0100 {
                    self.page_crossing = true;
                }
                Ok(OperandValue {
                    address: Some(address_incremented),
                    value: Some(self.memory.read(address_incremented, self, ppu)?),
                })
            }

            // rel	        relative	            OPC $BB	        branch target is PC + signed offset BB ***
            AddressingMode::Relative => {
                // Add u8 as twos complement i8 to u16
                let new_pc = self
                    .program_counter
                    .get()
                    .wrapping_add((ll & 0b0111_1111) as u16)
                    .wrapping_sub((ll & 0b1000_0000) as u16);
                log::debug!(
                    "Old PC: {:04X} Offset:{} New PC: {:04X}",
                    self.program_counter.get(),
                    ll as i8,
                    new_pc,
                );
                if ((new_pc & 0x0100) ^ (self.program_counter.get() & 0x0100)) == 0x0100 {
                    self.page_crossing = true;
                }
                Ok(OperandValue {
                    value: None,
                    address: Some(new_pc),
                })
            }

            // zpg	        zeropage	            OPC $LL	        operand is zeropage address (hi-byte is zero, address = $00LL)
            AddressingMode::ZeroPage => {
                let address: u16 = ll as u16;
                Ok(OperandValue {
                    address: Some(address),
                    value: Some(self.memory.read(address, self, ppu)?),
                })
            }

            // zpg,X	    zeropage, X-indexed	    OPC $LL,X	    operand is zeropage address; effective address is address incremented by X without carry **
            AddressingMode::ZeroPageX => {
                let address: u16 = ll.wrapping_add(self.x_register.get()) as u16;
                Ok(OperandValue {
                    address: Some(address),
                    value: Some(self.memory.read(address, self, ppu)?),
                })
            }

            // zpg,Y	    zeropage, Y-indexed	    OPC $LL,Y	    operand is zeropage address; effective address is address incremented by Y without carry **
            AddressingMode::ZeroPageY => {
                let address: u16 = ll.wrapping_add(self.y_register.get()) as u16;
                Ok(OperandValue {
                    address: Some(address),
                    value: Some(self.memory.read(address, self, ppu)?),
                })
            }
        }
    }

    // Read the next value at the instruction pointer and increment it
    fn read_next_value(&mut self, ppu: &mut Ppu) -> Result<u8, MainError> {
        let value = self.memory.read(self.program_counter.get(), self, ppu)?;
        log::debug!(
            "PC: {:04X} Value: {:02X} Next PC: {:04X}",
            self.program_counter.get(),
            value,
            self.program_counter.get() + 1
        );
        self.program_counter.increment();
        Ok(value)
    }

    // Run this function when a NMI occurs
    pub fn on_non_maskable_interrupt(&mut self) {
        self.nmi_line_current = true;
    }

    // Push the process counter and stack pointer on the stack
    fn push_pc_and_status_on_stack(&mut self, ppu: &mut Ppu) -> Result<(), MemoryError> {
        self.memory.write(
            self.stack_pointer.get() as u16 + 0x0100,
            self.program_counter.get_hibyte(),
            ppu,
        )?;
        self.stack_pointer.decrement();
        self.memory.write(
            self.stack_pointer.get() as u16 + 0x0100,
            self.program_counter.get_lobyte(),
            ppu,
        )?;
        self.stack_pointer.decrement();
        self.memory.write(
            self.stack_pointer.get() as u16 + 0x0100,
            self.status_register.get() | 0x10,
            ppu,
        )?;
        self.stack_pointer.decrement();
        Ok(())
    }

    // Check if an interrupt has occurred and return the type of interrupt
    //
    // This function will prioritize NMI's over IRQ's
    fn poll_interrupts(&mut self) -> InterruptState {
        let return_value: InterruptState;
        if self.nmi_line_triggered {
            return_value = InterruptState::NMI;
            log::debug!("Interrupt state NMI polled");
        } else if self.irq_line_triggered {
            return_value = InterruptState::IRQ;
            log::debug!("Interrupt state IRQ polled");
        } else {
            return_value = InterruptState::NormalOperation;
        }
        self.irq_line_triggered = false;
        self.nmi_line_triggered = false;
        return_value
    }

    // Perform all the initialization steps of the CPU
    //
    // After the initilization the CPU should wait for 7 cycles
    fn initialize_cpu(&mut self, ppu: &mut Ppu) -> Result<(), MemoryError> {
        let lobyte = self.memory.read(0xFFFC, self, ppu)?;
        log::debug!("lobyte: {:02X}", lobyte);
        let hibyte = self.memory.read(0xFFFD, self, ppu)?;
        log::debug!("hibyte: {:02X}", hibyte);
        self.program_counter.set_lobyte(lobyte);
        self.program_counter.set_hibyte(hibyte);
        self.stack_pointer.set(0xFD);
        self.status_register
            .set_bit(StatusRegisterBit::Interrupt, true);

        self.interrupt_polling_cycle = 0;
        self.total_cycles = 0;
        self.instructions_executed = 0;
        Ok(())
    }
}

#[test]
fn test_address_offset() {
    let offset = 128u8;
    assert_eq!(
        128u16
            .wrapping_add((offset & 0b0111_1111) as u16)
            .wrapping_sub((offset & 0b1000_0000) as u16),
        0
    );
    let offset = 127u8;
    assert_eq!(
        128u16
            .wrapping_add((offset & 0b0111_1111) as u16)
            .wrapping_sub((offset & 0b1000_0000) as u16),
        255
    );
}
