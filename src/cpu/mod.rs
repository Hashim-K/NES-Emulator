use registers::{CpuRegister, ProgramCounter, StatusRegister};

use crate::MainError;
use crate::memory::Memory;
mod registers;

enum AddressingMode {
    Accumulator,        // No operand,          instruction size is 1 byte
    Absolute,           // Operand is 2 bytes,  instruction size is 3 bytes
    AbsoluteX,          // Operand is 2 bytes,  instruction size is 3 bytes
    AbsoluteY,          // Operand is 2 bytes,  instruction size is 3 bytes
    Immediate,          // Operand is 1 byte,   instruction size is 2 bytes
    Implied,            // No operand,          instruction size is 1 byte
    Indirect,           // Operand is 2 bytes,  instruction size is 3 bytes
    IndirectX,          // Operand is 1 byte,   instruction size is 2 bytes
    IndirectY,          // Operand is 1 byte,   instruction size is 2 bytes
    Relative,           // Operand is 1 byte,   instruction size is 2 bytes
    ZeroPage,           // Operand is 1 byte,   instruction size is 2 bytes
    ZeroPageX,          // Operand is 1 byte,   instruction size is 2 bytes
    ZeroPageY,          // Operand is 1 byte,   instruction size is 2 bytes
}

impl AddressingMode {
    fn length(&self) -> u8 {
        match self {
            AddressingMode::Accumulator => 1,
            AddressingMode::Absolute => 3,
            AddressingMode::AbsoluteX => 3,
            AddressingMode::AbsoluteY => 3,
            AddressingMode::Immediate => 2,
            AddressingMode::Implied => 1,
            AddressingMode::Indirect => 3,
            AddressingMode::IndirectX => 2,
            AddressingMode::IndirectY => 2,
            AddressingMode::Relative => 2,
            AddressingMode::ZeroPage => 2,
            AddressingMode::ZeroPageX => 2,
            AddressingMode::ZeroPageY => 2,
        }
    }
}

enum Instruction{

    //888      8888888888  .d8888b.         d8888 888      
    //888      888        d88P  Y88b       d88888 888      
    //888      888        888    888      d88P888 888      
    //888      8888888    888            d88P 888 888      
    //888      888        888  88888    d88P  888 888      
    //888      888        888    888   d88P   888 888      
    //888      888        Y88b  d88P  d8888888888 888      
    //88888888 8888888888  "Y8888P88 d88P     888 88888888 


    //Transfer Instructions
    LDA(AddressingMode),    // Load Accumulator
    LDX(AddressingMode),    // Load X Register
    LDY(AddressingMode),    // Load Y Register
    STA(AddressingMode),    // Store Accumulator
    STX(AddressingMode),    // Store X Register
    STY(AddressingMode),    // Store Y Register
    TAX(AddressingMode),    // Transfer Accumulator to X
    TAY(AddressingMode),    // Transfer Accumulator to Y
    TSX(AddressingMode),    // Transfer Stack Pointer to X
    TXA(AddressingMode),    // Transfer X to Accumulator
    TXS(AddressingMode),    // Transfer X to Stack Pointer
    TYA(AddressingMode),    // Transfer Y to Accumulator

    //Stack Instructions
    PHA(AddressingMode),    // Push Accumulator
    PHP(AddressingMode),    // Push Processor Status
    PLA(AddressingMode),    // Pull Accumulator
    PLP(AddressingMode),    // Pull Processor Status

    //Decrements & Increments
    DEC(AddressingMode),    // Decrement Memory
    DEX(AddressingMode),    // Decrement X Register
    DEY(AddressingMode),    // Decrement Y Register
    INC(AddressingMode),    // Increment Memory
    INX(AddressingMode),    // Increment X Register
    INY(AddressingMode),    // Increment Y Register

    //Arithmetic Instructions
    ADC(AddressingMode),    // Add with Carry (prepare by CLC)
    SBC(AddressingMode),    // Subtract with Carry (prepare by SEC)

    //Logical Instructions
    AND(AddressingMode),    // AND Memory with Accumulator
    EOR(AddressingMode),    // Exclusive OR Memory with Accumulator
    ORA(AddressingMode),    // OR Memory with Accumulator

    //Shift & Rotate Instructions
    ASL(AddressingMode),    // Arithmetic Shift Left (shifts in a zero bit on the right)
    LSR(AddressingMode),    // Logical Shift Right (shifts in a zero bit on the left)
    ROL(AddressingMode),    // Rotate Left (shifts in the carry bit on the right)
    ROR(AddressingMode),    // Rotate Right (shifts in the carry bit on the left)

    //Flag Instructions
    CLC(AddressingMode),    // Clear Carry Flag
    CLD(AddressingMode),    // Clear Decimal Mode Flag (BCD arithmetic disabled)
    CLI(AddressingMode),    // Clear Interrupt Disable Flag
    CLV(AddressingMode),    // Clear Overflow Flag
    SEC(AddressingMode),    // Set Carry Flag
    SED(AddressingMode),    // Set Decimal Mode Flag (BCD arithmetic enabled)
    SEI(AddressingMode),    // Set Interrupt Disable Flag

    //Comparison Instructions
    CMP(AddressingMode),    // Compare Memory and Accumulator
    CPX(AddressingMode),    // Compare Memory and X Register
    CPY(AddressingMode),    // Compare Memory and Y Register

    //Conditional Branch Instructions
    BCC(AddressingMode),    // Branch on Carry Clear
    BCS(AddressingMode),    // Branch on Carry Set
    BEQ(AddressingMode),    // Branch on Equal (zero set)
    BMI(AddressingMode),    // Branch on Minus (negative set)
    BNE(AddressingMode),    // Branch on Not Equal (zero clear)

    //Jump & Subroutine Instructions
    JMP(AddressingMode),    // Jump
    JSR(AddressingMode),    // Jump to Subroutine
    RTS(AddressingMode),    // Return from Subroutine

    //Interrupt Instructions
    BRK(AddressingMode),    // Force Break
    RTI(AddressingMode),    // Return from Interrupt

    //Miscellaneous Instructions
    BIT(AddressingMode),    // Bit Test
    NOP(AddressingMode),    // No Operation

    // 
} 

impl Instruction {
    fn decode(opcode: u8) -> Result<Instruction, MainError> {
        match opcode {
            0xA9 => todo!(),
            _ => panic!("Unknown opcode: {:#X}", opcode),
        }
        
    }

    fn execute(&self, cpu: &mut Cpu) -> Result<(), MainError> {
        let operand_value =  cpu.get_operand_value(self.get_addressing_mode())?;
        match self {
            Instruction::LDA(_) => {
                cpu.accumulator.set(operand_value as u8);
                Ok(())
            }
            _ => panic!("Unknown instruction"),
        }
    }

    fn get_addressing_mode(&self) -> &AddressingMode {
        match self {
            Instruction::LDA(mode)
            | Instruction::LDX(mode)
            | Instruction::LDY(mode)
            | Instruction::STA(mode)
            | Instruction::STX(mode)
            | Instruction::STY(mode)
            | Instruction::TAX(mode)
            | Instruction::TAY(mode)
            | Instruction::TSX(mode)
            | Instruction::TXA(mode)
            | Instruction::TXS(mode)
            | Instruction::TYA(mode)
            | Instruction::PHA(mode)
            | Instruction::PHP(mode)
            | Instruction::PLA(mode)
            | Instruction::PLP(mode)
            | Instruction::DEC(mode)
            | Instruction::DEX(mode)
            | Instruction::DEY(mode)
            | Instruction::INC(mode)
            | Instruction::INX(mode)
            | Instruction::INY(mode)
            | Instruction::ADC(mode)
            | Instruction::SBC(mode)
            | Instruction::AND(mode)
            | Instruction::EOR(mode)
            | Instruction::ORA(mode)
            | Instruction::ASL(mode)
            | Instruction::LSR(mode)
            | Instruction::ROL(mode)
            | Instruction::ROR(mode)
            | Instruction::CLC(mode)
            | Instruction::CLD(mode)
            | Instruction::CLI(mode)
            | Instruction::CLV(mode)
            | Instruction::SEC(mode)
            | Instruction::SED(mode)
            | Instruction::SEI(mode)
            | Instruction::CMP(mode)
            | Instruction::CPX(mode)
            | Instruction::CPY(mode)
            | Instruction::BCC(mode)
            | Instruction::BCS(mode)
            | Instruction::BEQ(mode)
            | Instruction::BMI(mode)
            | Instruction::BNE(mode)
            | Instruction::JMP(mode)
            | Instruction::JSR(mode)
            | Instruction::RTS(mode)
            | Instruction::BRK(mode)
            | Instruction::RTI(mode)
            | Instruction::BIT(mode)
            | Instruction::NOP(mode) => mode,
        }
    }
}

// let current_instruction = Instruction::decode(0xA9);

pub struct Cpu {
    memory: Memory,
    accumulator: CpuRegister,
    x_register: CpuRegister,
    y_register: CpuRegister,
    stack_pointer: CpuRegister,
    program_counter: ProgramCounter,
    status_register: StatusRegister,
    current_instruction: Instruction,

}

impl Cpu{
    fn get_operand_value(&mut self, addressing_mode: &AddressingMode) -> Result<usize, MainError> {
        let mut hh: u8 = 0;
        let mut ll: u8 = 0;

        match addressing_mode.length() {
            1 => (),
            2 => {
                ll = self.read_next_value()?
            },
            3 => {
                    ll = self.read_next_value()?;
                    hh = self.read_next_value()?;
                },
            _ => panic!("Unknown addressing mode"),
        }
        match addressing_mode {
            // A	        Accumulator	            OPC A	        operand is AC (implied single byte instruction)
            AddressingMode::Accumulator => Ok(0),
            
            // abs	        absolute	            OPC $LLHH	    operand is address $HHLL *
            AddressingMode::Absolute =>{
                let address: u16 = (hh as u16) << 8 | ll as u16;
                Ok(address as usize)
            }

            // abs,X	    absolute, X-indexed	    OPC $LLHH,X	    operand is address; effective address is address incremented by X with carry **
            AddressingMode::AbsoluteX =>{
                let address: u16 = (hh as u16) << 8 | ll as u16;
                Ok((address + self.x_register.get() as u16) as usize)
            }
            
            // abs,Y	    absolute, Y-indexed	    OPC $LLHH,Y	    operand is address; effective address is address incremented by Y with carry **
            AddressingMode::AbsoluteY =>{
                let address: u16 = (hh as u16) << 8 | ll as u16;
                Ok((address + self.y_register.get() as u16) as usize)
            }
            
            // #	        immediate	            OPC #$BB	    operand is byte BB
            AddressingMode::Immediate =>{
                Ok(ll as usize)
            }
            
            // impl	        implied	                OPC	            operand implied
            AddressingMode::Implied =>{
                Ok(0)
            }
            
            // ind	        indirect	            OPC ($LLHH)	    operand is address; effective address is contents of word at address: C.w($HHLL)
            AddressingMode::Indirect =>{
                let address: u16 = (hh as u16) << 8 | ll as u16;
                let memory_ll: u8 = self.memory_read(address)?;
                let memory_hh: u8 = self.memory_read(address + 1)?;
                let memory_address: u16 = (memory_hh as u16) << 8 | memory_ll as u16;
                Ok(memory_address as usize)
            }
            
            // X,ind	    X-indexed, indirect	    OPC ($LL,X)	    operand is zeropage address; effective address is word in (LL + X, LL + X + 1), inc. without carry: C.w($00LL + X)
            AddressingMode::IndirectX =>{
                let address: u16 = ll.saturating_add(self.x_register.get()) as u16;
                let memory_ll: u8 = self.memory_read(address)?;
                let memory_hh: u8 = self.memory_read(address + 1)?;
                let memory_address: u16 = (memory_hh as u16) << 8 | memory_ll as u16;
                Ok(memory_address as usize)
            }
            
            // ind,Y	    indirect, Y-indexed	    OPC ($LL),Y	    operand is zeropage address; effective address is word in (LL, LL + 1) incremented by Y with carry: C.w($00LL) + Y
            AddressingMode::IndirectY =>{
                let address: u16 = ll as u16;
                let memory_ll: u8 = self.memory_read(address)? + self.y_register.get();
                let memory_hh: u8 = self.memory_read(address + 1)?;
                let memory_address: u16 = (memory_hh as u16) << 8 | memory_ll as u16;
                Ok(memory_address as usize)
            }
            
            // rel	        relative	            OPC $BB	        branch target is PC + signed offset BB ***
            AddressingMode::Relative =>{
                let offset: i8 = ll as i8;
                Ok((self.program_counter.get() as i16 + offset as i16) as usize)
            },
            
            // zpg	        zeropage	            OPC $LL	        operand is zeropage address (hi-byte is zero, address = $00LL)
            AddressingMode::ZeroPage =>{
                let address: u16 = (0 as u16) << 8 | ll as u16;
                Ok(address as usize)
            }
            
            // zpg,X	    zeropage, X-indexed	    OPC $LL,X	    operand is zeropage address; effective address is address incremented by X without carry **
            AddressingMode::ZeroPageX =>{
                let address: u16 = ll.saturating_add(self.x_register.get()) as u16;
                Ok(address  as usize)
            }
            
            // zpg,Y	    zeropage, Y-indexed	    OPC $LL,Y	    operand is zeropage address; effective address is address incremented by Y without carry **
            AddressingMode::ZeroPageY =>{
                let address: u16 = ll.saturating_add(self.x_register.get()) as u16;
                Ok(address  as usize)
            }
        }
    }

    fn read_next_value(&mut self) -> Result<u8,MainError> {
        let value = self.memory.get_memory_byte(self.program_counter.get())?;
        self.program_counter.increment();
        Ok(value)
    }

    fn memory_read(&self, address: u16) -> Result<u8, MainError> {
        let memory_value = self.memory.get_memory_byte(address)?;
        Ok(memory_value)
    }
}

