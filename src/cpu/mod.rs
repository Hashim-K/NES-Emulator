use crate::cpu::instructions::{AddressingMode, Instruction, InstructionType};
use crate::error::MyTickError;
use registers::{CpuRegister, ProgramCounter, StatusRegister, StatusRegisterBit};

use crate::memory::Memory;
use crate::MainError;
mod instructions;
mod registers;

struct OperandValue {
    value: Option<u8>,
    address: Option<u16>,
}

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
    irq: bool,
    nmi: bool,
    res: bool,
    in_interrupt: bool,
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
            current_cycle: 0,
            instruction_cycle_count: 0,
            irq: false,
            nmi: false,
            res: true,
            in_interrupt: false,
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

    fn read_reset_vector(&mut self, memory: &mut Memory) -> Result<(), MainError> {
        let lower_reset_byte = memory.read(0xfffc)?;
        let upper_reset_byte = memory.read(0xfffd)?;
        let reset_vector: u16 = ((upper_reset_byte as u16) << 8 | lower_reset_byte as u16);
        self.program_counter.set(reset_vector);
        Ok(())
    }

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
        memory.write_memory_byte(address, value)?;
        Ok(())
    }

    pub fn tick(&mut self, memory: &mut Memory) -> Result<(), MyTickError> {
        if self.current_cycle > self.instruction_cycle_count {
            self.current_cycle = 1;

            let opcode = self.read_next_value(memory)?;
            println!("Reading opcode {:?}", opcode);

            let instruction: Instruction =
                Instruction::decode(opcode).expect("Failed decoding opcode");
            println!("Executing instruction {:?}", instruction);
            instruction.execute(self, memory)?;

            self.instruction_cycle_count = self.current_instruction.addressing_mode.length();
            if self.current_instruction.is_rmw() {
                self.instruction_cycle_count -= 1;
            }
            self.current_instruction = instruction;
        }

        self.current_cycle += 1;

        Ok(())
    }
}
