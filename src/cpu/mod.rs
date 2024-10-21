use std::any::Any;
use std::sync::mpsc::RecvTimeoutError;

use crate::cpu::instructions::{AddressingMode, Instruction, InstructionType};
use crate::error::{MemoryError, MyTickError};
use crate::memory::Memory;
use crate::MainError;
use interrupt_handler::InterruptState;
use registers::{CpuRegister, ProgramCounter, StatusRegister, StatusRegisterBit};
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
    initialized: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
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
            interrupt_state: InterruptState::NormalOperation,
            nmi_line_prev: false,
            nmi_line_current: false,
            nmi_line_triggered: false,
            irq_line_triggered: true,
            initialized: false,
        }
    }

    fn get_operand_value(
        &mut self,
        addressing_mode: &AddressingMode,
        memory: &mut Memory,
    ) -> Result<OperandValue, MainError> {
        let mut hh: u8 = 0;
        let mut ll: u8 = 0;

        match addressing_mode.length() {
            1 => (),
            2 => ll = self.read_next_value(memory)?,
            3 => {
                ll = self.read_next_value(memory)?;
                hh = self.read_next_value(memory)?;
            }
            _ => panic!("Unknown addressing mode"),
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
                Ok(OperandValue {
                    address: Some(address),
                    value: Some(self.memory_read(address, memory)?),
                })
            }

            // abs,X	    absolute, X-indexed	    OPC $LLHH,X	    operand is address; effective address is address incremented by X with carry **
            AddressingMode::AbsoluteX => {
                let address: u16 = (hh as u16) << 8 | ll as u16;
                Ok(OperandValue {
                    address: Some(address + self.x_register.get() as u16),
                    value: Some(self.memory_read(address + self.y_register.get() as u16, memory)?),
                })
            }

            // abs,Y	    absolute, Y-indexed	    OPC $LLHH,Y	    operand is address; effective address is address incremented by Y with carry **
            AddressingMode::AbsoluteY => {
                let address: u16 = (hh as u16) << 8 | ll as u16;
                Ok(OperandValue {
                    address: Some(address + self.y_register.get() as u16),
                    value: Some(self.memory_read(address + self.y_register.get() as u16, memory)?),
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
                let memory_ll: u8 = self.memory_read(address, memory)?;
                let memory_hh: u8 = self.memory_read(address + 1, memory)?;
                let memory_address: u16 = (memory_hh as u16) << 8 | memory_ll as u16;
                Ok(OperandValue {
                    address: Some(memory_address),
                    value: Some(self.memory_read(memory_address, memory)?),
                })
            }

            // X,ind	    X-indexed, indirect	    OPC ($LL,X)	    operand is zeropage address; effective address is word in (LL + X, LL + X + 1), inc. without carry: C.w($00LL + X)
            AddressingMode::IndirectX => {
                let address: u16 = ll.saturating_add(self.x_register.get()) as u16;
                let memory_ll: u8 = self.memory_read(address, memory)?;
                let memory_hh: u8 = self.memory_read(address + 1, memory)?;
                let memory_address: u16 = (memory_hh as u16) << 8 | memory_ll as u16;
                Ok(OperandValue {
                    address: Some(memory_address),
                    value: Some(self.memory_read(memory_address, memory)?),
                })
            }

            // ind,Y	    indirect, Y-indexed	    OPC ($LL),Y	    operand is zeropage address; effective address is word in (LL, LL + 1) incremented by Y with carry: C.w($00LL) + Y
            AddressingMode::IndirectY => {
                let address: u16 = ll as u16;
                let memory_ll: u8 = self.memory_read(address, memory)? + self.y_register.get();
                let memory_hh: u8 = self.memory_read(address + 1, memory)?;
                let memory_address: u16 = (memory_hh as u16) << 8 | memory_ll as u16;
                Ok(OperandValue {
                    address: Some(memory_address),
                    value: Some(self.memory_read(memory_address, memory)?),
                })
            }

            // rel	        relative	            OPC $BB	        branch target is PC + signed offset BB ***
            AddressingMode::Relative => {
                let offset: i8 = ll as i8;
                Ok(OperandValue {
                    value: None,
                    address: Some((self.program_counter.get() as i16 + offset as i16) as u16),
                })
            }

            // zpg	        zeropage	            OPC $LL	        operand is zeropage address (hi-byte is zero, address = $00LL)
            AddressingMode::ZeroPage => {
                let address: u16 = (0 as u16) << 8 | ll as u16;
                Ok(OperandValue {
                    address: Some(address),
                    value: Some(self.memory_read(address, memory)?),
                })
            }

            // zpg,X	    zeropage, X-indexed	    OPC $LL,X	    operand is zeropage address; effective address is address incremented by X without carry **
            AddressingMode::ZeroPageX => {
                let address: u16 = ll.saturating_add(self.x_register.get()) as u16;
                Ok(OperandValue {
                    address: Some(address),
                    value: Some(self.memory_read(address, memory)?),
                })
            }

            // zpg,Y	    zeropage, Y-indexed	    OPC $LL,Y	    operand is zeropage address; effective address is address incremented by Y without carry **
            AddressingMode::ZeroPageY => {
                let address: u16 = ll.saturating_add(self.x_register.get()) as u16;
                Ok(OperandValue {
                    address: Some(address),
                    value: Some(self.memory_read(address, memory)?),
                })
            }
        }
    }

    fn read_next_value(&mut self, memory: &mut Memory) -> Result<u8, MainError> {
        if self.program_counter.get() == 0xFFFC {
            self.read_reset_vector(memory)?;
        }
        let value = memory.read(self.program_counter.get())?;
        self.program_counter.increment();
        Ok(value)
    }

    // fn read_reset_vector(&mut self, memory: &mut Memory) -> Result<(), MainError> {
    //     let lower_reset_byte = memory.read(0xfffc)?;
    //     let upper_reset_byte = memory.read(0xfffd)?;
    //     let reset_vector: u16 = ((upper_reset_byte as u16) << 8 | lower_reset_byte as u16);
    //     self.program_counter.set(reset_vector);
    //     Ok(())
    // }

    fn memory_read(&self, address: u16, memory: &mut Memory) -> Result<u8, MainError> {
        let memory_value = memory.read(address)?;
        Ok(memory_value)
    }

    fn memory_write(
        &mut self,
        address: u16,
        value: u8,
        memory: &mut Memory,
    ) -> Result<(), MainError> {
        memory.write(address, value)?;
        Ok(())
    }

    pub fn tick(&mut self, memory: &mut Memory) -> Result<(), MyTickError> {
        // set the cpu to the startup state fi
        // println!("the clock is ticking");
        // print!("cpu: {self:?}");
        if !self.initialized {
            self.initialize_cpu(memory)?;
        }
        if self.current_cycle == self.interrupt_polling_cycle {
            // this line is for interrupt hijacking to be working later
            let current_interrupt = self.poll_interrupts(memory, false);
            if current_interrupt == InterruptState::IRQ
                && self
                    .status_register
                    .get_bit(StatusRegisterBit::InterruptBit)
            {
                self.interrupt_state = InterruptState::NormalOperation;
            } else {
                self.interrupt_state = current_interrupt;
            }
        }
        // println!("Polled for interrupts, checking ");
        // execute interrupt or opcode
        if self.current_cycle > self.instruction_cycle_count {
            self.current_cycle = 1;

            match self.interrupt_state {
                InterruptState::NMI => {
                    println!("Executing NMI");
                    self.push_pc_and_status_on_stack(memory)?;
                    let nmi_lobyte = memory.read(0xFFFA)?;
                    let nmi_hibyte = memory.read(0xFFFB)?;
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
                    // TODO: interface for irq
                    ();
                }
                InterruptState::NormalOperation => {
                    let opcode = self.read_next_value(memory)?;
                    println!("Reading opcode {:?}", opcode);

                    let instruction: Instruction =
                        Instruction::decode(opcode).expect("Failed decoding opcode");
                    println!("Executing instruction {:?}", instruction);
                    instruction.execute(self, memory)?;

                    self.instruction_cycle_count =
                        self.current_instruction.addressing_mode.length();

                    self.interrupt_polling_cycle = self.instruction_cycle_count - 1;

                    if self.current_instruction.is_rmw() {
                        self.instruction_cycle_count -= 1;
                    }
                    self.current_instruction = instruction;
                }
            }
        }

        // interrupt hijacking, if an interrupt arrives in the first four cycles of a BRK
        if self.current_instruction.instruction_type == InstructionType::BRK
            && self.current_cycle < 4
        {
            let current_interrupt = self.poll_interrupts(memory, false);
            match current_interrupt {
                InterruptState::NMI => {
                    let nmi_lobyte = memory.read(0xFFFA)?;
                    let nmi_hibyte = memory.read(0xFFFB)?;
                    self.program_counter.set_lobyte(nmi_lobyte);
                    self.program_counter.set_hibyte(nmi_hibyte);

                    self.instruction_cycle_count = 7;
                    self.interrupt_state = InterruptState::NormalOperation;
                    self.interrupt_polling_cycle = 0;
                }
                InterruptState::IRQ => {
                    if !self
                        .status_register
                        .get_bit(StatusRegisterBit::InterruptBit)
                    {
                        let nmi_lobyte = memory.read(0xFFFE)?;
                        let nmi_hibyte = memory.read(0xFFFF)?;
                        self.program_counter.set_lobyte(nmi_lobyte);
                        self.program_counter.set_hibyte(nmi_hibyte);

                        self.instruction_cycle_count = 7;
                        self.interrupt_state = InterruptState::NormalOperation;
                        self.interrupt_polling_cycle = 0;
                    }
                }
                InterruptState::NormalOperation => (),
            }
        }

        if self.nmi_line_current && !self.nmi_line_prev {
            self.nmi_line_triggered = true;
        }
        self.current_cycle += 1;
        self.nmi_line_prev = self.nmi_line_prev;
        self.nmi_line_current = false;
        Ok(())
    }

    pub fn on_non_maskable_interrupt(&mut self) {
        self.nmi_line_current = true;
    }

    fn push_pc_and_status_on_stack(&mut self, memory: &mut Memory) -> Result<(), MemoryError> {
        memory.write(
            self.stack_pointer.get() as u16 + 0x0100,
            self.program_counter.get_hibyte(),
        )?;
        self.stack_pointer.decrement();
        memory.write(
            self.stack_pointer.get() as u16 + 0x0100,
            self.program_counter.get_lobyte(),
        )?;
        self.stack_pointer.decrement();
        memory.write(
            self.stack_pointer.get() as u16 + 0x0100,
            self.status_register.get_byte(),
        )?;
        self.stack_pointer.decrement();
        Ok(())
    }

    fn poll_interrupts(&mut self, memory: &mut Memory, reset_lines: bool) -> InterruptState {
        let return_value: InterruptState;
        if self.nmi_line_triggered {
            return_value = InterruptState::NMI;
            println!("Interrupt state NMI polled");
        } else if self.irq_line_triggered {
            return_value = InterruptState::IRQ;
            println!("Interrupt state IRQ polled");
        } else {
            return_value = InterruptState::NormalOperation;
            // println!("Interrupt state NormalOperation polled");
        }
        self.irq_line_triggered = false;
        self.nmi_line_triggered = false;
        return_value
    }

    fn initialize_cpu(&mut self, memory: &mut Memory) -> Result<(), MemoryError> {
        println!("intializing cpu");
        let lobyte = memory.read(0xFFFC)?;
        let hibyte = memory.read(0xFFFD)?;
        self.program_counter.set_lobyte(lobyte);
        self.program_counter.set_hibyte(hibyte);
        // println!("program counter set to {}", self.program_counter.get());
        self.stack_pointer.set(0xFF);

        self.instruction_cycle_count = 7;
        self.interrupt_state = InterruptState::NormalOperation;
        self.interrupt_polling_cycle = 0;
        self.initialized = true;

        Ok(())
    }
}
