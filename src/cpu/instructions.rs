use crate::cpu::{Cpu, StatusRegisterBit};
use crate::memory::Memory;
use crate::MainError;

#[derive(Debug)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub addressing_mode: AddressingMode,
}

#[derive(Debug)]
pub enum AddressingMode {
    Accumulator, // No operand,          instruction size is 1 byte
    Absolute,    // Operand is 2 bytes,  instruction size is 3 bytes
    AbsoluteX,   // Operand is 2 bytes,  instruction size is 3 bytes
    AbsoluteY,   // Operand is 2 bytes,  instruction size is 3 bytes
    Immediate,   // Operand is 1 byte,   instruction size is 2 bytes
    Implied,     // No operand,          instruction size is 1 byte
    Indirect,    // Operand is 2 bytes,  instruction size is 3 bytes
    IndirectX,   // Operand is 1 byte,   instruction size is 2 bytes
    IndirectY,   // Operand is 1 byte,   instruction size is 2 bytes
    Relative,    // Operand is 1 byte,   instruction size is 2 bytes
    ZeroPage,    // Operand is 1 byte,   instruction size is 2 bytes
    ZeroPageX,   // Operand is 1 byte,   instruction size is 2 bytes
    ZeroPageY,   // Operand is 1 byte,   instruction size is 2 bytes
}

impl AddressingMode {
    pub fn length(&self) -> u8 {
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

#[derive(Debug)]
pub enum InstructionType {
    //888      8888888888  .d8888b.         d8888 888
    //888      888        d88P  Y88b       d88888 888
    //888      888        888    888      d88P888 888
    //888      8888888    888            d88P 888 888
    //888      888        888  88888    d88P  888 888
    //888      888        888    888   d88P   888 888
    //888      888        Y88b  d88P  d8888888888 888
    //88888888 8888888888  "Y8888P88 d88P     888 88888888

    //Transfer Instructions
    LDA, // Load Accumulator
    LDX, // Load X Register
    LDY, // Load Y Register
    STA, // Store Accumulator
    STX, // Store X Register
    STY, // Store Y Register
    TAX, // Transfer Accumulator to X
    TAY, // Transfer Accumulator to Y
    TSX, // Transfer Stack Pointer to X
    TXA, // Transfer X to Accumulator
    TXS, // Transfer X to Stack Pointer
    TYA, // Transfer Y to Accumulator

    //Stack Instructions
    PHA, // Push Accumulator
    PHP, // Push Processor Status
    PLA, // Pull Accumulator
    PLP, // Pull Processor Status

    //Decrements & Increments
    DEC, // Decrement Memory
    DEX, // Decrement X Register
    DEY, // Decrement Y Register
    INC, // Increment Memory
    INX, // Increment X Register
    INY, // Increment Y Register

    //Arithmetic Instructions
    ADC, // Add with Carry (prepare by CLC)
    SBC, // Subtract with Carry (prepare by SEC)

    //Logical Instructions
    AND, // AND Memory with Accumulator
    EOR, // Exclusive OR Memory with Accumulator
    ORA, // OR Memory with Accumulator

    //Shift & Rotate Instructions
    ASL, // Arithmetic Shift Left (shifts in a zero bit on the right)
    LSR, // Logical Shift Right (shifts in a zero bit on the left)
    ROL, // Rotate Left (shifts in the carry bit on the right)
    ROR, // Rotate Right (shifts in the carry bit on the left)

    //Flag Instructions
    CLC, // Clear Carry Flag
    CLD, // Clear Decimal Mode Flag (BCD arithmetic disabled)
    CLI, // Clear Interrupt Disable Flag
    CLV, // Clear Overflow Flag
    SEC, // Set Carry Flag
    SED, // Set Decimal Mode Flag (BCD arithmetic enabled)
    SEI, // Set Interrupt Disable Flag

    //Comparison Instructions
    CMP, // Compare Memory and Accumulator
    CPX, // Compare Memory and X Register
    CPY, // Compare Memory and Y Register

    //Conditional Branch Instructions
    BCC, // Branch on Carry Clear
    BCS, // Branch on Carry Set
    BEQ, // Branch on Equal (zero set)
    BMI, // Branch on Minus (negative set)
    BNE, // Branch on Not Equal (zero clear)
    BPL, // Branch on Plus (negative clear)
    BVC, // Branch on Overflow Clear
    BVS, // Branch on Overflow Set

    //Jump & Subroutine Instructions
    JMP, // Jump
    JSR, // Jump to Subroutine
    RTS, // Return from Subroutine

    //Interrupt Instructions
    BRK, // Force Break
    RTI, // Return from Interrupt

    //Miscellaneous Instructions
    BIT, // Bit Test
    NOP, // No Operation
}

impl Instruction {
    pub fn decode(opcode: u8) -> Result<Instruction, MainError> {
        match opcode {
            //ADC
            0x69 => Ok(Instruction {
                instruction_type: InstructionType::ADC,
                addressing_mode: AddressingMode::Immediate,
            }),
            0x65 => Ok(Instruction {
                instruction_type: InstructionType::ADC,
                addressing_mode: AddressingMode::ZeroPage,
            }),
            0x75 => Ok(Instruction {
                instruction_type: InstructionType::ADC,
                addressing_mode: AddressingMode::ZeroPageX,
            }),
            0x6D => Ok(Instruction {
                instruction_type: InstructionType::ADC,
                addressing_mode: AddressingMode::Absolute,
            }),
            0x7D => Ok(Instruction {
                instruction_type: InstructionType::ADC,
                addressing_mode: AddressingMode::AbsoluteX,
            }),
            0x79 => Ok(Instruction {
                instruction_type: InstructionType::ADC,
                addressing_mode: AddressingMode::AbsoluteY,
            }),
            0x61 => Ok(Instruction {
                instruction_type: InstructionType::ADC,
                addressing_mode: AddressingMode::IndirectX,
            }),
            0x71 => Ok(Instruction {
                instruction_type: InstructionType::ADC,
                addressing_mode: AddressingMode::IndirectY,
            }),

            //AND
            0x29 => Ok(Instruction {
                instruction_type: InstructionType::AND,
                addressing_mode: AddressingMode::Immediate,
            }),
            0x25 => Ok(Instruction {
                instruction_type: InstructionType::AND,
                addressing_mode: AddressingMode::ZeroPage,
            }),
            0x35 => Ok(Instruction {
                instruction_type: InstructionType::AND,
                addressing_mode: AddressingMode::ZeroPageX,
            }),
            0x2D => Ok(Instruction {
                instruction_type: InstructionType::AND,
                addressing_mode: AddressingMode::Absolute,
            }),
            0x3D => Ok(Instruction {
                instruction_type: InstructionType::AND,
                addressing_mode: AddressingMode::AbsoluteX,
            }),
            0x39 => Ok(Instruction {
                instruction_type: InstructionType::AND,
                addressing_mode: AddressingMode::AbsoluteY,
            }),
            0x21 => Ok(Instruction {
                instruction_type: InstructionType::AND,
                addressing_mode: AddressingMode::IndirectX,
            }),
            0x31 => Ok(Instruction {
                instruction_type: InstructionType::AND,
                addressing_mode: AddressingMode::IndirectY,
            }),

            //ASL
            0x0A => Ok(Instruction {
                instruction_type: InstructionType::ASL,
                addressing_mode: AddressingMode::Accumulator,
            }),
            0x06 => Ok(Instruction {
                instruction_type: InstructionType::ASL,
                addressing_mode: AddressingMode::ZeroPage,
            }),
            0x16 => Ok(Instruction {
                instruction_type: InstructionType::ASL,
                addressing_mode: AddressingMode::ZeroPageX,
            }),
            0x0E => Ok(Instruction {
                instruction_type: InstructionType::ASL,
                addressing_mode: AddressingMode::Absolute,
            }),
            0x1E => Ok(Instruction {
                instruction_type: InstructionType::ASL,
                addressing_mode: AddressingMode::AbsoluteX,
            }),

            //BIT
            0x24 => Ok(Instruction {
                instruction_type: InstructionType::BIT,
                addressing_mode: AddressingMode::ZeroPage,
            }),
            0x2C => Ok(Instruction {
                instruction_type: InstructionType::BIT,
                addressing_mode: AddressingMode::Absolute,
            }),

            //Branch
            0x10 => Ok(Instruction {
                instruction_type: InstructionType::BPL,
                addressing_mode: AddressingMode::Relative,
            }),
            0x30 => Ok(Instruction {
                instruction_type: InstructionType::BMI,
                addressing_mode: AddressingMode::Relative,
            }),
            0x50 => Ok(Instruction {
                instruction_type: InstructionType::BVC,
                addressing_mode: AddressingMode::Relative,
            }),
            0x70 => Ok(Instruction {
                instruction_type: InstructionType::BVS,
                addressing_mode: AddressingMode::Relative,
            }),
            0x90 => Ok(Instruction {
                instruction_type: InstructionType::BCC,
                addressing_mode: AddressingMode::Relative,
            }),
            0xB0 => Ok(Instruction {
                instruction_type: InstructionType::BCS,
                addressing_mode: AddressingMode::Relative,
            }),
            0xD0 => Ok(Instruction {
                instruction_type: InstructionType::BNE,
                addressing_mode: AddressingMode::Relative,
            }),
            0xF0 => Ok(Instruction {
                instruction_type: InstructionType::BEQ,
                addressing_mode: AddressingMode::Relative,
            }),

            //BRK
            0x00 => Ok(Instruction {
                instruction_type: InstructionType::BRK,
                addressing_mode: AddressingMode::Implied,
            }),

            //CMP
            0xC9 => Ok(Instruction {
                instruction_type: InstructionType::CMP,
                addressing_mode: AddressingMode::Immediate,
            }),
            0xC5 => Ok(Instruction {
                instruction_type: InstructionType::CMP,
                addressing_mode: AddressingMode::ZeroPage,
            }),
            0xD5 => Ok(Instruction {
                instruction_type: InstructionType::CMP,
                addressing_mode: AddressingMode::ZeroPageX,
            }),
            0xCD => Ok(Instruction {
                instruction_type: InstructionType::CMP,
                addressing_mode: AddressingMode::Absolute,
            }),
            0xDD => Ok(Instruction {
                instruction_type: InstructionType::CMP,
                addressing_mode: AddressingMode::AbsoluteX,
            }),
            0xD9 => Ok(Instruction {
                instruction_type: InstructionType::CMP,
                addressing_mode: AddressingMode::AbsoluteY,
            }),
            0xC1 => Ok(Instruction {
                instruction_type: InstructionType::CMP,
                addressing_mode: AddressingMode::IndirectX,
            }),
            0xD1 => Ok(Instruction {
                instruction_type: InstructionType::CMP,
                addressing_mode: AddressingMode::IndirectY,
            }),

            //CPX
            0xE0 => Ok(Instruction {
                instruction_type: InstructionType::CPX,
                addressing_mode: AddressingMode::Immediate,
            }),
            0xE4 => Ok(Instruction {
                instruction_type: InstructionType::CPX,
                addressing_mode: AddressingMode::ZeroPage,
            }),
            0xEC => Ok(Instruction {
                instruction_type: InstructionType::CPX,
                addressing_mode: AddressingMode::Absolute,
            }),

            //CPY
            0xC0 => Ok(Instruction {
                instruction_type: InstructionType::CPY,
                addressing_mode: AddressingMode::Immediate,
            }),
            0xC4 => Ok(Instruction {
                instruction_type: InstructionType::CPY,
                addressing_mode: AddressingMode::ZeroPage,
            }),
            0xCC => Ok(Instruction {
                instruction_type: InstructionType::CPY,
                addressing_mode: AddressingMode::Absolute,
            }),

            //DEC
            0xC6 => Ok(Instruction {
                instruction_type: InstructionType::DEC,
                addressing_mode: AddressingMode::ZeroPage,
            }),
            0xD6 => Ok(Instruction {
                instruction_type: InstructionType::DEC,
                addressing_mode: AddressingMode::ZeroPageX,
            }),
            0xCE => Ok(Instruction {
                instruction_type: InstructionType::DEC,
                addressing_mode: AddressingMode::Absolute,
            }),
            0xDE => Ok(Instruction {
                instruction_type: InstructionType::DEC,
                addressing_mode: AddressingMode::AbsoluteX,
            }),

            //EOR
            0x49 => Ok(Instruction {
                instruction_type: InstructionType::EOR,
                addressing_mode: AddressingMode::Immediate,
            }),
            0x45 => Ok(Instruction {
                instruction_type: InstructionType::EOR,
                addressing_mode: AddressingMode::ZeroPage,
            }),
            0x55 => Ok(Instruction {
                instruction_type: InstructionType::EOR,
                addressing_mode: AddressingMode::ZeroPageX,
            }),
            0x4D => Ok(Instruction {
                instruction_type: InstructionType::EOR,
                addressing_mode: AddressingMode::Absolute,
            }),
            0x5D => Ok(Instruction {
                instruction_type: InstructionType::EOR,
                addressing_mode: AddressingMode::AbsoluteX,
            }),
            0x59 => Ok(Instruction {
                instruction_type: InstructionType::EOR,
                addressing_mode: AddressingMode::AbsoluteY,
            }),
            0x41 => Ok(Instruction {
                instruction_type: InstructionType::EOR,
                addressing_mode: AddressingMode::IndirectX,
            }),
            0x51 => Ok(Instruction {
                instruction_type: InstructionType::EOR,
                addressing_mode: AddressingMode::IndirectY,
            }),

            //Flag
            0x18 => Ok(Instruction {
                instruction_type: InstructionType::CLC,
                addressing_mode: AddressingMode::Implied,
            }),
            0x38 => Ok(Instruction {
                instruction_type: InstructionType::SEC,
                addressing_mode: AddressingMode::Implied,
            }),
            0x58 => Ok(Instruction {
                instruction_type: InstructionType::CLI,
                addressing_mode: AddressingMode::Implied,
            }),
            0x78 => Ok(Instruction {
                instruction_type: InstructionType::SEI,
                addressing_mode: AddressingMode::Implied,
            }),
            0xB8 => Ok(Instruction {
                instruction_type: InstructionType::CLV,
                addressing_mode: AddressingMode::Implied,
            }),
            0xD8 => Ok(Instruction {
                instruction_type: InstructionType::CLD,
                addressing_mode: AddressingMode::Implied,
            }),
            0xF8 => Ok(Instruction {
                instruction_type: InstructionType::SED,
                addressing_mode: AddressingMode::Implied,
            }),

            //INC
            0xE6 => Ok(Instruction {
                instruction_type: InstructionType::INC,
                addressing_mode: AddressingMode::ZeroPage,
            }),
            0xF6 => Ok(Instruction {
                instruction_type: InstructionType::INC,
                addressing_mode: AddressingMode::ZeroPageX,
            }),
            0xEE => Ok(Instruction {
                instruction_type: InstructionType::INC,
                addressing_mode: AddressingMode::Absolute,
            }),
            0xFE => Ok(Instruction {
                instruction_type: InstructionType::INC,
                addressing_mode: AddressingMode::AbsoluteX,
            }),

            //JMP
            0x4C => Ok(Instruction {
                instruction_type: InstructionType::JMP,
                addressing_mode: AddressingMode::Absolute,
            }),
            0x6C => Ok(Instruction {
                instruction_type: InstructionType::JMP,
                addressing_mode: AddressingMode::Indirect,
            }),

            //JSR
            0x20 => Ok(Instruction {
                instruction_type: InstructionType::JSR,
                addressing_mode: AddressingMode::Absolute,
            }),

            //LDA
            0xA9 => Ok(Instruction {
                instruction_type: InstructionType::LDA,
                addressing_mode: AddressingMode::Immediate,
            }),
            0xA5 => Ok(Instruction {
                instruction_type: InstructionType::LDA,
                addressing_mode: AddressingMode::ZeroPage,
            }),
            0xB5 => Ok(Instruction {
                instruction_type: InstructionType::LDA,
                addressing_mode: AddressingMode::ZeroPageX,
            }),
            0xAD => Ok(Instruction {
                instruction_type: InstructionType::LDA,
                addressing_mode: AddressingMode::Absolute,
            }),
            0xBD => Ok(Instruction {
                instruction_type: InstructionType::LDA,
                addressing_mode: AddressingMode::AbsoluteX,
            }),
            0xB9 => Ok(Instruction {
                instruction_type: InstructionType::LDA,
                addressing_mode: AddressingMode::AbsoluteY,
            }),
            0xA1 => Ok(Instruction {
                instruction_type: InstructionType::LDA,
                addressing_mode: AddressingMode::IndirectX,
            }),
            0xB1 => Ok(Instruction {
                instruction_type: InstructionType::LDA,
                addressing_mode: AddressingMode::IndirectY,
            }),

            //LDX
            0xA2 => Ok(Instruction {
                instruction_type: InstructionType::LDX,
                addressing_mode: AddressingMode::Immediate,
            }),
            0xA6 => Ok(Instruction {
                instruction_type: InstructionType::LDX,
                addressing_mode: AddressingMode::ZeroPage,
            }),
            0xB6 => Ok(Instruction {
                instruction_type: InstructionType::LDX,
                addressing_mode: AddressingMode::ZeroPageY,
            }),
            0xAE => Ok(Instruction {
                instruction_type: InstructionType::LDX,
                addressing_mode: AddressingMode::Absolute,
            }),
            0xBE => Ok(Instruction {
                instruction_type: InstructionType::LDX,
                addressing_mode: AddressingMode::AbsoluteY,
            }),

            //LDY
            0xA0 => Ok(Instruction {
                instruction_type: InstructionType::LDY,
                addressing_mode: AddressingMode::Immediate,
            }),
            0xA4 => Ok(Instruction {
                instruction_type: InstructionType::LDY,
                addressing_mode: AddressingMode::ZeroPage,
            }),
            0xB4 => Ok(Instruction {
                instruction_type: InstructionType::LDY,
                addressing_mode: AddressingMode::ZeroPageX,
            }),
            0xAC => Ok(Instruction {
                instruction_type: InstructionType::LDY,
                addressing_mode: AddressingMode::Absolute,
            }),
            0xBC => Ok(Instruction {
                instruction_type: InstructionType::LDY,
                addressing_mode: AddressingMode::AbsoluteX,
            }),

            //LSR
            0x4A => Ok(Instruction {
                instruction_type: InstructionType::LSR,
                addressing_mode: AddressingMode::Accumulator,
            }),
            0x46 => Ok(Instruction {
                instruction_type: InstructionType::LSR,
                addressing_mode: AddressingMode::ZeroPage,
            }),
            0x56 => Ok(Instruction {
                instruction_type: InstructionType::LSR,
                addressing_mode: AddressingMode::ZeroPageX,
            }),
            0x4E => Ok(Instruction {
                instruction_type: InstructionType::LSR,
                addressing_mode: AddressingMode::Absolute,
            }),
            0x5E => Ok(Instruction {
                instruction_type: InstructionType::LSR,
                addressing_mode: AddressingMode::AbsoluteX,
            }),

            //NOP
            0xEA => Ok(Instruction {
                instruction_type: InstructionType::NOP,
                addressing_mode: AddressingMode::Implied,
            }),

            //ORA
            0x09 => Ok(Instruction {
                instruction_type: InstructionType::ORA,
                addressing_mode: AddressingMode::Immediate,
            }),
            0x05 => Ok(Instruction {
                instruction_type: InstructionType::ORA,
                addressing_mode: AddressingMode::ZeroPage,
            }),
            0x15 => Ok(Instruction {
                instruction_type: InstructionType::ORA,
                addressing_mode: AddressingMode::ZeroPageX,
            }),
            0x0D => Ok(Instruction {
                instruction_type: InstructionType::ORA,
                addressing_mode: AddressingMode::Absolute,
            }),
            0x1D => Ok(Instruction {
                instruction_type: InstructionType::ORA,
                addressing_mode: AddressingMode::AbsoluteX,
            }),
            0x19 => Ok(Instruction {
                instruction_type: InstructionType::ORA,
                addressing_mode: AddressingMode::AbsoluteY,
            }),
            0x01 => Ok(Instruction {
                instruction_type: InstructionType::ORA,
                addressing_mode: AddressingMode::IndirectX,
            }),
            0x11 => Ok(Instruction {
                instruction_type: InstructionType::ORA,
                addressing_mode: AddressingMode::IndirectY,
            }),

            //Register Instructionsinstruction_type
            0xAA => Ok(Instruction {
                instruction_type: InstructionType::TAX,
                addressing_mode: AddressingMode::Implied,
            }),
            0x8A => Ok(Instruction {
                instruction_type: InstructionType::TXA,
                addressing_mode: AddressingMode::Implied,
            }),
            0xCA => Ok(Instruction {
                instruction_type: InstructionType::DEX,
                addressing_mode: AddressingMode::Implied,
            }),
            0xE8 => Ok(Instruction {
                instruction_type: InstructionType::INX,
                addressing_mode: AddressingMode::Implied,
            }),
            0xA8 => Ok(Instruction {
                instruction_type: InstructionType::TAY,
                addressing_mode: AddressingMode::Implied,
            }),
            0x98 => Ok(Instruction {
                instruction_type: InstructionType::TYA,
                addressing_mode: AddressingMode::Implied,
            }),
            0x88 => Ok(Instruction {
                instruction_type: InstructionType::DEY,
                addressing_mode: AddressingMode::Implied,
            }),
            0xC8 => Ok(Instruction {
                instruction_type: InstructionType::INY,
                addressing_mode: AddressingMode::Implied,
            }),

            //ROL
            0x2A => Ok(Instruction {
                instruction_type: InstructionType::ROL,
                addressing_mode: AddressingMode::Accumulator,
            }),
            0x26 => Ok(Instruction {
                instruction_type: InstructionType::ROL,
                addressing_mode: AddressingMode::ZeroPage,
            }),
            0x36 => Ok(Instruction {
                instruction_type: InstructionType::ROL,
                addressing_mode: AddressingMode::ZeroPageX,
            }),
            0x2E => Ok(Instruction {
                instruction_type: InstructionType::ROL,
                addressing_mode: AddressingMode::Absolute,
            }),
            0x3E => Ok(Instruction {
                instruction_type: InstructionType::ROL,
                addressing_mode: AddressingMode::AbsoluteX,
            }),

            //ROR
            0x6A => Ok(Instruction {
                instruction_type: InstructionType::ROR,
                addressing_mode: AddressingMode::Accumulator,
            }),
            0x66 => Ok(Instruction {
                instruction_type: InstructionType::ROR,
                addressing_mode: AddressingMode::ZeroPage,
            }),
            0x76 => Ok(Instruction {
                instruction_type: InstructionType::ROR,
                addressing_mode: AddressingMode::ZeroPageX,
            }),
            0x6E => Ok(Instruction {
                instruction_type: InstructionType::ROR,
                addressing_mode: AddressingMode::Absolute,
            }),
            0x7E => Ok(Instruction {
                instruction_type: InstructionType::ROR,
                addressing_mode: AddressingMode::AbsoluteX,
            }),

            //RTI
            0x40 => Ok(Instruction {
                instruction_type: InstructionType::RTI,
                addressing_mode: AddressingMode::Implied,
            }),

            //RTS
            0x60 => Ok(Instruction {
                instruction_type: InstructionType::RTS,
                addressing_mode: AddressingMode::Implied,
            }),

            //SBC
            0xE9 => Ok(Instruction {
                instruction_type: InstructionType::SBC,
                addressing_mode: AddressingMode::Immediate,
            }),
            0xE5 => Ok(Instruction {
                instruction_type: InstructionType::SBC,
                addressing_mode: AddressingMode::ZeroPage,
            }),
            0xF5 => Ok(Instruction {
                instruction_type: InstructionType::SBC,
                addressing_mode: AddressingMode::ZeroPageX,
            }),
            0xED => Ok(Instruction {
                instruction_type: InstructionType::SBC,
                addressing_mode: AddressingMode::Absolute,
            }),
            0xFD => Ok(Instruction {
                instruction_type: InstructionType::SBC,
                addressing_mode: AddressingMode::AbsoluteX,
            }),
            0xF9 => Ok(Instruction {
                instruction_type: InstructionType::SBC,
                addressing_mode: AddressingMode::AbsoluteY,
            }),
            0xE1 => Ok(Instruction {
                instruction_type: InstructionType::SBC,
                addressing_mode: AddressingMode::IndirectX,
            }),
            0xF1 => Ok(Instruction {
                instruction_type: InstructionType::SBC,
                addressing_mode: AddressingMode::IndirectY,
            }),

            //STA
            0x85 => Ok(Instruction {
                instruction_type: InstructionType::STA,
                addressing_mode: AddressingMode::ZeroPage,
            }),
            0x95 => Ok(Instruction {
                instruction_type: InstructionType::STA,
                addressing_mode: AddressingMode::ZeroPageX,
            }),
            0x8D => Ok(Instruction {
                instruction_type: InstructionType::STA,
                addressing_mode: AddressingMode::Absolute,
            }),
            0x9D => Ok(Instruction {
                instruction_type: InstructionType::STA,
                addressing_mode: AddressingMode::AbsoluteX,
            }),
            0x99 => Ok(Instruction {
                instruction_type: InstructionType::STA,
                addressing_mode: AddressingMode::AbsoluteY,
            }),
            0x81 => Ok(Instruction {
                instruction_type: InstructionType::STA,
                addressing_mode: AddressingMode::IndirectX,
            }),
            0x91 => Ok(Instruction {
                instruction_type: InstructionType::STA,
                addressing_mode: AddressingMode::IndirectY,
            }),

            //Stack Instructions
            0x9A => Ok(Instruction {
                instruction_type: InstructionType::TXS,
                addressing_mode: AddressingMode::Implied,
            }),
            0xBA => Ok(Instruction {
                instruction_type: InstructionType::TSX,
                addressing_mode: AddressingMode::Implied,
            }),
            0x48 => Ok(Instruction {
                instruction_type: InstructionType::PHA,
                addressing_mode: AddressingMode::Implied,
            }),
            0x68 => Ok(Instruction {
                instruction_type: InstructionType::PLA,
                addressing_mode: AddressingMode::Implied,
            }),
            0x08 => Ok(Instruction {
                instruction_type: InstructionType::PHP,
                addressing_mode: AddressingMode::Implied,
            }),
            0x28 => Ok(Instruction {
                instruction_type: InstructionType::PLP,
                addressing_mode: AddressingMode::Implied,
            }),

            //STX
            0x86 => Ok(Instruction {
                instruction_type: InstructionType::STX,
                addressing_mode: AddressingMode::ZeroPage,
            }),
            0x96 => Ok(Instruction {
                instruction_type: InstructionType::STX,
                addressing_mode: AddressingMode::ZeroPageY,
            }),
            0x8E => Ok(Instruction {
                instruction_type: InstructionType::STX,
                addressing_mode: AddressingMode::Absolute,
            }),

            //STY
            0x84 => Ok(Instruction {
                instruction_type: InstructionType::STY,
                addressing_mode: AddressingMode::ZeroPage,
            }),
            0x94 => Ok(Instruction {
                instruction_type: InstructionType::STY,
                addressing_mode: AddressingMode::ZeroPageX,
            }),
            0x8C => Ok(Instruction {
                instruction_type: InstructionType::STY,
                addressing_mode: AddressingMode::Absolute,
            }),

            //UNKNOWN INSTRUCTION
            _ => panic!("Unknown opcode: {:#X}", opcode),
        }
    }

    // Set zero bit if the number read is 0
    fn set_status_if_zero(value: u8, cpu: &mut Cpu) {
        if value == 0 {
            cpu.status_register
                .set_bit(StatusRegisterBit::ZeroBit, true);
        } else {
            cpu.status_register
                .set_bit(StatusRegisterBit::ZeroBit, false);
        }
    }

    // Set negative bit if the number read is negative
    fn set_status_if_negative(value: u8, cpu: &mut Cpu) {
        // Check if 7th bit is set
        cpu.status_register
            .set_bit(StatusRegisterBit::NegativeBit, value & (1 << 7) > 0);
    }

    pub fn execute(&self, cpu: &mut Cpu, memory: &mut Memory) -> Result<(), MainError> {
        let operand_value = cpu.get_operand_value(&self.addressing_mode, memory)?;
        match self.instruction_type {
            InstructionType::LDA => {
                let value = operand_value.value.expect("LDA operand value is None");
                cpu.x_register.set(value);
                Self::set_status_if_zero(value, cpu);
                Self::set_status_if_negative(value, cpu);
                Ok(())
            }

            // test instructions
            InstructionType::LDX => {
                let value = operand_value.value.expect("LDX operand value is None");
                cpu.x_register.set(value);
                Self::set_status_if_zero(value, cpu);
                Self::set_status_if_negative(value, cpu);
                Ok(())
            }

            InstructionType::LDY => {
                let value = operand_value.value.expect("LDY operand value is None");
                cpu.y_register.set(value);
                Self::set_status_if_zero(value, cpu);
                Self::set_status_if_negative(value, cpu);
                Ok(())
            }

            InstructionType::STA => {
                let address: u16 = operand_value.address.expect("STA Address is None");
                cpu.memory_write(address, cpu.accumulator.get(), memory)?;
                Ok(())
            }
            InstructionType::STX => {
                let address: u16 = operand_value.address.expect("STX Address is None");
                cpu.memory_write(address, cpu.x_register.get(), memory)?;
                Ok(())
            }

            InstructionType::STY => {
                let address: u16 = operand_value.address.expect("STY Address is None");
                cpu.memory_write(address, cpu.y_register.get(), memory)?;
                Ok(())
            }

            InstructionType::TAX => {
                cpu.x_register.set(cpu.accumulator.get());
                Self::set_status_if_zero(cpu.x_register.get(), cpu);
                Self::set_status_if_negative(cpu.x_register.get(), cpu);
                Ok(())
            }

            InstructionType::TAY => {
                cpu.y_register.set(cpu.accumulator.get());
                Self::set_status_if_zero(cpu.y_register.get(), cpu);
                Self::set_status_if_negative(cpu.y_register.get(), cpu);
                Ok(())
            }

            InstructionType::TXA => {
                cpu.accumulator.set(cpu.x_register.get());
                Self::set_status_if_zero(cpu.accumulator.get(), cpu);
                Self::set_status_if_negative(cpu.accumulator.get(), cpu);
                Ok(())
            }

            InstructionType::TYA => {
                cpu.accumulator.set(cpu.y_register.get());
                Self::set_status_if_zero(cpu.accumulator.get(), cpu);
                Self::set_status_if_negative(cpu.accumulator.get(), cpu);
                Ok(())
            }

            InstructionType::TSX => {
                cpu.x_register.set(cpu.stack_pointer.get());
                Self::set_status_if_zero(cpu.x_register.get(), cpu);
                Self::set_status_if_negative(cpu.x_register.get(), cpu);
                Ok(())
            }

            InstructionType::TXS => {
                cpu.stack_pointer.set(cpu.x_register.get());
                Self::set_status_if_zero(cpu.stack_pointer.get(), cpu);
                Self::set_status_if_negative(cpu.stack_pointer.get(), cpu);
                Ok(())
            }

            InstructionType::PHA => {
                let address = 0x0100 + cpu.stack_pointer.get() as u16;
                memory.write(address, cpu.accumulator.get())?;
                cpu.stack_pointer.increment();
                Ok(())
            }

            InstructionType::PHP => {
                let address = 0x0100 + cpu.stack_pointer.get() as u16;
                memory.write(address, cpu.status_register.get())?;
                cpu.stack_pointer.increment();
                Ok(())
            }

            InstructionType::PLA => {
                let address = 0x0100 + cpu.stack_pointer.get() as u16;
                cpu.accumulator.set(memory.read(address)?);
                cpu.stack_pointer.decrement();
                Self::set_status_if_zero(cpu.accumulator.get(), cpu);
                Self::set_status_if_negative(cpu.accumulator.get(), cpu);
                Ok(())
            }

            InstructionType::PLP => {
                let address = 0x0100 + cpu.stack_pointer.get() as u16;
                let value = memory.read(address)?;
                cpu.status_register.set_from_stack(value);
                Ok(())
            }

            InstructionType::INC => {
                let address = operand_value.address.expect("INC Address is None");
                let value = operand_value.value.expect("INC value is None");
                let new_value = value.wrapping_add(1);
                cpu.memory_write(address, new_value, memory)?;
                Self::set_status_if_zero(new_value, cpu);
                Self::set_status_if_negative(new_value, cpu);
                Ok(())
            }

            InstructionType::INX => {
                cpu.x_register.set(cpu.x_register.get().wrapping_add(1));
                Self::set_status_if_zero(cpu.x_register.get(), cpu);
                Self::set_status_if_negative(cpu.x_register.get(), cpu);
                Ok(())
            }

            InstructionType::INY => {
                cpu.y_register.set(cpu.y_register.get().wrapping_add(1));
                Self::set_status_if_zero(cpu.y_register.get(), cpu);
                Self::set_status_if_negative(cpu.y_register.get(), cpu);
                Ok(())
            }

            InstructionType::DEC => {
                let address = operand_value.address.expect("DEC Address is None");
                let value = operand_value.value.expect("DEC value is None");
                let new_value = value.wrapping_sub(1);
                cpu.memory_write(address, new_value, memory)?;
                Self::set_status_if_zero(new_value, cpu);
                Self::set_status_if_negative(new_value, cpu);
                Ok(())
            }

            InstructionType::DEX => {
                cpu.x_register.set(cpu.x_register.get().wrapping_sub(1));
                Self::set_status_if_zero(cpu.x_register.get(), cpu);
                Self::set_status_if_negative(cpu.x_register.get(), cpu);
                Ok(())
            }

            InstructionType::DEY => {
                cpu.y_register.set(cpu.y_register.get().wrapping_sub(1));
                Self::set_status_if_zero(cpu.y_register.get(), cpu);
                Self::set_status_if_negative(cpu.y_register.get(), cpu);
                Ok(())
            }

            InstructionType::ADC => {
                let acc = cpu.accumulator.get();
                let op_value = operand_value.value.expect("Operand value for ADC is None");
                let carry = u8::from(cpu.status_register.get_carry());
                let result = acc.wrapping_add(op_value).wrapping_add(carry);
                let did_carry =
                    result < acc || (acc == 0 && carry == 1) || (result == 0xff && carry == 1);
                let did_overflow = (acc > 127 && op_value > 127 && result < 128)
                    || (acc < 128 && result < 128 && result > 127);
                cpu.accumulator.set(result);

                Self::set_status_if_zero(cpu.accumulator.get(), cpu);
                Self::set_status_if_negative(cpu.accumulator.get(), cpu);

                cpu.status_register
                    .set_bit(StatusRegisterBit::CarryBit, did_carry);
                cpu.status_register
                    .set_bit(StatusRegisterBit::OverflowBit, did_overflow);
                Ok(())
            }

            // TODO implement decimal mode and carry
            InstructionType::SBC => {
                let acc = cpu.accumulator.get();
                let op_value = operand_value.value.expect("Operand value for SBC is None");
                let carry = u8::from(cpu.status_register.get_carry());
                let result = acc.wrapping_sub(op_value).wrapping_sub(1 - carry);
                let did_carry = false; // TODO

                // Check if sign is wrong. This happens in the following cases:
                // positive - negative results in negative
                // negative - positive results in positive
                let did_overflow = (acc ^ op_value) & (acc ^ result) & 0x80 != 0;
                cpu.accumulator.set(result);

                Self::set_status_if_zero(cpu.accumulator.get(), cpu);
                Self::set_status_if_negative(cpu.accumulator.get(), cpu);

                cpu.status_register
                    .set_bit(StatusRegisterBit::CarryBit, did_carry);
                cpu.status_register
                    .set_bit(StatusRegisterBit::OverflowBit, did_overflow);
                Ok(())
            }

            InstructionType::CMP | InstructionType::CPX | InstructionType::CPY => {
                let reg = match self.instruction_type {
                    InstructionType::CMP => cpu.accumulator.get(),
                    InstructionType::CPX => cpu.x_register.get(),
                    InstructionType::CPY => cpu.y_register.get(),
                    _ => panic!(),
                };
                let value = operand_value
                    .value
                    .expect("Operand value for CMP/CPX/CPY is None");
                cpu.status_register
                    .set_bit(StatusRegisterBit::CarryBit, reg >= value);
                cpu.status_register
                    .set_bit(StatusRegisterBit::ZeroBit, reg == value);
                Self::set_status_if_negative(reg.wrapping_sub(value), cpu);
                Ok(())
            }

            InstructionType::AND => {
                let value = cpu.accumulator.get()
                    & operand_value.value.expect("Operand value for AND is None");
                cpu.accumulator.set(value);
                Self::set_status_if_zero(cpu.accumulator.get(), cpu);
                Self::set_status_if_negative(cpu.accumulator.get(), cpu);
                Ok(())
            }

            InstructionType::EOR => {
                let value = cpu.accumulator.get()
                    ^ operand_value.value.expect("Operand value for EOR is None");
                cpu.accumulator.set(value);
                Self::set_status_if_zero(cpu.accumulator.get(), cpu);
                Self::set_status_if_negative(cpu.accumulator.get(), cpu);
                Ok(())
            }

            InstructionType::ORA => {
                let value = cpu.accumulator.get()
                    | operand_value.value.expect("Operand value for ORA is None");
                cpu.accumulator.set(value);
                Self::set_status_if_zero(cpu.accumulator.get(), cpu);
                Self::set_status_if_negative(cpu.accumulator.get(), cpu);
                Ok(())
            }

            InstructionType::BIT => {
                let operator_value = operand_value.value.expect("Operand value for BIT is None");
                let value = cpu.accumulator.get() & operator_value;
                Self::set_status_if_zero(value, cpu);
                Self::set_status_if_negative(operator_value, cpu);
                // Check if 6th bit is set
                cpu.status_register
                    .set_bit(StatusRegisterBit::OverflowBit, value & (1 << 6) > 0);
                Ok(())
            }

            InstructionType::ASL => {
                let operator_value = operand_value.value.expect("Operand value for ASL is None");
                let result = operator_value << 1;
                cpu.status_register
                    .set_bit(StatusRegisterBit::CarryBit, operator_value & (1 << 7) != 0);
                Self::set_status_if_zero(result, cpu);
                Self::set_status_if_negative(result, cpu);

                if let Some(address) = operand_value.address {
                    memory.write(address, result)?;
                } else {
                    cpu.accumulator.set(result)
                }
                Ok(())
            }

            InstructionType::LSR => {
                let operator_value = operand_value.value.expect("Operand value for LSR is None");
                let result = operator_value << 1;
                cpu.status_register
                    .set_bit(StatusRegisterBit::CarryBit, operator_value & (1 << 7) != 0);
                Self::set_status_if_zero(result, cpu);
                Self::set_status_if_negative(result, cpu);

                if let Some(address) = operand_value.address {
                    memory.write(address, result)?;
                } else {
                    cpu.accumulator.set(result)
                }
                Ok(())
            }

            InstructionType::ROL => {
                let operator_value = operand_value.value.expect("Operand value for ROL is None");
                let carry = u8::from(cpu.status_register.get_carry());
                let result = operator_value << 1 | carry;
                cpu.status_register
                    .set_bit(StatusRegisterBit::CarryBit, operator_value & (1 << 7) != 0);
                Self::set_status_if_zero(result, cpu);
                Self::set_status_if_negative(result, cpu);

                if let Some(address) = operand_value.address {
                    memory.write(address, result)?;
                } else {
                    cpu.accumulator.set(result)
                }
                Ok(())
            }

            InstructionType::ROR => {
                let operator_value = operand_value.value.expect("Operand value for ROR is None");
                let carry = u8::from(cpu.status_register.get_carry());
                let result = operator_value >> 1 | carry << 7;
                cpu.status_register
                    .set_bit(StatusRegisterBit::CarryBit, operator_value & (1 << 0) != 0);
                Self::set_status_if_zero(result, cpu);
                Self::set_status_if_negative(result, cpu);

                if let Some(address) = operand_value.address {
                    memory.write(address, result)?;
                } else {
                    cpu.accumulator.set(result)
                }
                Ok(())
            }

            InstructionType::CLC => {
                cpu.status_register
                    .set_bit(StatusRegisterBit::CarryBit, false);
                Ok(())
            }

            InstructionType::CLD => {
                cpu.status_register
                    .set_bit(StatusRegisterBit::DecimalBit, false);
                Ok(())
            }

            InstructionType::CLI => {
                cpu.status_register
                    .set_bit(StatusRegisterBit::InterruptBit, false);
                Ok(())
            }

            InstructionType::CLV => {
                cpu.status_register
                    .set_bit(StatusRegisterBit::OverflowBit, false);
                Ok(())
            }

            InstructionType::SEC => {
                cpu.status_register
                    .set_bit(StatusRegisterBit::CarryBit, true);
                Ok(())
            }

            InstructionType::SED => {
                cpu.status_register
                    .set_bit(StatusRegisterBit::DecimalBit, true);
                Ok(())
            }

            InstructionType::SEI => {
                cpu.status_register
                    .set_bit(StatusRegisterBit::InterruptBit, true);
                Ok(())
            }

            InstructionType::BCC => {
                //TODO: Implement
                Ok(())
            }

            InstructionType::BCS => {
                //TODO: Implement
                Ok(())
            }

            InstructionType::BEQ => {
                //TODO: Implement
                Ok(())
            }

            InstructionType::BMI => {
                //TODO: Implement
                Ok(())
            }

            InstructionType::BNE => {
                //TODO: Implement
                Ok(())
            }

            InstructionType::BPL => {
                //TODO: Implement
                Ok(())
            }

            InstructionType::BVC => {
                //TODO: Implement
                Ok(())
            }

            InstructionType::BVS => {
                //TODO: Implement
                Ok(())
            }

            InstructionType::JMP => {
                cpu.program_counter
                    .set(operand_value.address.expect("Expected address for JMP"));
                Ok(())
            }

            InstructionType::JSR => {
                //TODO: Implement
                Ok(())
            }

            InstructionType::RTS => {
                //TODO: Implement
                Ok(())
            }

            InstructionType::BRK => {
                //TODO: Implement
                Ok(())
            }

            InstructionType::RTI => {
                //TODO: Implement
                Ok(())
            }

            InstructionType::NOP => {
                //TODO: Implement
                Ok(())
            }
        }
    }
}
