use registers::{CpuRegister, ProgramCounter, StatusRegister, StatusRegisterBit};

use crate::memory::Memory;
use crate::MainError;
mod registers;

enum AddressingMode {
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

struct operand_value {
    value: Option<u8>,
    address: Option<u16>,
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

enum Instruction {
    //888      8888888888  .d8888b.         d8888 888
    //888      888        d88P  Y88b       d88888 888
    //888      888        888    888      d88P888 888
    //888      8888888    888            d88P 888 888
    //888      888        888  88888    d88P  888 888
    //888      888        888    888   d88P   888 888
    //888      888        Y88b  d88P  d8888888888 888
    //88888888 8888888888  "Y8888P88 d88P     888 88888888

    //Transfer Instructions
    LDA(AddressingMode), // Load Accumulator
    LDX(AddressingMode), // Load X Register
    LDY(AddressingMode), // Load Y Register
    STA(AddressingMode), // Store Accumulator
    STX(AddressingMode), // Store X Register
    STY(AddressingMode), // Store Y Register
    TAX(AddressingMode), // Transfer Accumulator to X
    TAY(AddressingMode), // Transfer Accumulator to Y
    TSX(AddressingMode), // Transfer Stack Pointer to X
    TXA(AddressingMode), // Transfer X to Accumulator
    TXS(AddressingMode), // Transfer X to Stack Pointer
    TYA(AddressingMode), // Transfer Y to Accumulator

    //Stack Instructions
    PHA(AddressingMode), // Push Accumulator
    PHP(AddressingMode), // Push Processor Status
    PLA(AddressingMode), // Pull Accumulator
    PLP(AddressingMode), // Pull Processor Status

    //Decrements & Increments
    DEC(AddressingMode), // Decrement Memory
    DEX(AddressingMode), // Decrement X Register
    DEY(AddressingMode), // Decrement Y Register
    INC(AddressingMode), // Increment Memory
    INX(AddressingMode), // Increment X Register
    INY(AddressingMode), // Increment Y Register

    //Arithmetic Instructions
    ADC(AddressingMode), // Add with Carry (prepare by CLC)
    SBC(AddressingMode), // Subtract with Carry (prepare by SEC)

    //Logical Instructions
    AND(AddressingMode), // AND Memory with Accumulator
    EOR(AddressingMode), // Exclusive OR Memory with Accumulator
    ORA(AddressingMode), // OR Memory with Accumulator

    //Shift & Rotate Instructions
    ASL(AddressingMode), // Arithmetic Shift Left (shifts in a zero bit on the right)
    LSR(AddressingMode), // Logical Shift Right (shifts in a zero bit on the left)
    ROL(AddressingMode), // Rotate Left (shifts in the carry bit on the right)
    ROR(AddressingMode), // Rotate Right (shifts in the carry bit on the left)

    //Flag Instructions
    CLC(AddressingMode), // Clear Carry Flag
    CLD(AddressingMode), // Clear Decimal Mode Flag (BCD arithmetic disabled)
    CLI(AddressingMode), // Clear Interrupt Disable Flag
    CLV(AddressingMode), // Clear Overflow Flag
    SEC(AddressingMode), // Set Carry Flag
    SED(AddressingMode), // Set Decimal Mode Flag (BCD arithmetic enabled)
    SEI(AddressingMode), // Set Interrupt Disable Flag

    //Comparison Instructions
    CMP(AddressingMode), // Compare Memory and Accumulator
    CPX(AddressingMode), // Compare Memory and X Register
    CPY(AddressingMode), // Compare Memory and Y Register

    //Conditional Branch Instructions
    BCC(AddressingMode), // Branch on Carry Clear
    BCS(AddressingMode), // Branch on Carry Set
    BEQ(AddressingMode), // Branch on Equal (zero set)
    BMI(AddressingMode), // Branch on Minus (negative set)
    BNE(AddressingMode), // Branch on Not Equal (zero clear)
    BPL(AddressingMode), // Branch on Plus (negative clear)
    BVC(AddressingMode), // Branch on Overflow Clear
    BVS(AddressingMode), // Branch on Overflow Set

    //Jump & Subroutine Instructions
    JMP(AddressingMode), // Jump
    JSR(AddressingMode), // Jump to Subroutine
    RTS(AddressingMode), // Return from Subroutine

    //Interrupt Instructions
    BRK(AddressingMode), // Force Break
    RTI(AddressingMode), // Return from Interrupt

    //Miscellaneous Instructions
    BIT(AddressingMode), // Bit Test
    NOP(AddressingMode), // No Operation
}

impl Instruction {
    fn decode(opcode: u8) -> Result<Instruction, MainError> {
        match opcode {
            //ADC
            0x69 => Ok(Instruction::ADC(AddressingMode::Immediate)),
            0x65 => Ok(Instruction::ADC(AddressingMode::ZeroPage)),
            0x75 => Ok(Instruction::ADC(AddressingMode::ZeroPageX)),
            0x6D => Ok(Instruction::ADC(AddressingMode::Absolute)),
            0x7D => Ok(Instruction::ADC(AddressingMode::AbsoluteX)),
            0x79 => Ok(Instruction::ADC(AddressingMode::AbsoluteY)),
            0x61 => Ok(Instruction::ADC(AddressingMode::IndirectX)),
            0x71 => Ok(Instruction::ADC(AddressingMode::IndirectY)),

            //AND
            0x29 => Ok(Instruction::AND(AddressingMode::Immediate)),
            0x25 => Ok(Instruction::AND(AddressingMode::ZeroPage)),
            0x35 => Ok(Instruction::AND(AddressingMode::ZeroPageX)),
            0x2D => Ok(Instruction::AND(AddressingMode::Absolute)),
            0x3D => Ok(Instruction::AND(AddressingMode::AbsoluteX)),
            0x39 => Ok(Instruction::AND(AddressingMode::AbsoluteY)),
            0x21 => Ok(Instruction::AND(AddressingMode::IndirectX)),
            0x31 => Ok(Instruction::AND(AddressingMode::IndirectY)),

            //ASL
            0x0A => Ok(Instruction::ASL(AddressingMode::Accumulator)),
            0x06 => Ok(Instruction::ASL(AddressingMode::ZeroPage)),
            0x16 => Ok(Instruction::ASL(AddressingMode::ZeroPageX)),
            0x0E => Ok(Instruction::ASL(AddressingMode::Absolute)),
            0x1E => Ok(Instruction::ASL(AddressingMode::AbsoluteX)),

            //BIT
            0x24 => Ok(Instruction::BIT(AddressingMode::ZeroPage)),
            0x2C => Ok(Instruction::BIT(AddressingMode::Absolute)),

            //Branch
            0x10 => Ok(Instruction::BPL(AddressingMode::Relative)),
            0x30 => Ok(Instruction::BMI(AddressingMode::Relative)),
            0x50 => Ok(Instruction::BVC(AddressingMode::Relative)),
            0x70 => Ok(Instruction::BVS(AddressingMode::Relative)),
            0x90 => Ok(Instruction::BCC(AddressingMode::Relative)),
            0xB0 => Ok(Instruction::BCS(AddressingMode::Relative)),
            0xD0 => Ok(Instruction::BNE(AddressingMode::Relative)),
            0xF0 => Ok(Instruction::BEQ(AddressingMode::Relative)),

            //BRK
            0x00 => Ok(Instruction::BRK(AddressingMode::Implied)),

            //CMP
            0xC9 => Ok(Instruction::CMP(AddressingMode::Immediate)),
            0xC5 => Ok(Instruction::CMP(AddressingMode::ZeroPage)),
            0xD5 => Ok(Instruction::CMP(AddressingMode::ZeroPageX)),
            0xCD => Ok(Instruction::CMP(AddressingMode::Absolute)),
            0xDD => Ok(Instruction::CMP(AddressingMode::AbsoluteX)),
            0xD9 => Ok(Instruction::CMP(AddressingMode::AbsoluteY)),
            0xC1 => Ok(Instruction::CMP(AddressingMode::IndirectX)),
            0xD1 => Ok(Instruction::CMP(AddressingMode::IndirectY)),

            //CPX
            0xE0 => Ok(Instruction::CPX(AddressingMode::Immediate)),
            0xE4 => Ok(Instruction::CPX(AddressingMode::ZeroPage)),
            0xEC => Ok(Instruction::CPX(AddressingMode::Absolute)),

            //CPY
            0xC0 => Ok(Instruction::CPY(AddressingMode::Immediate)),
            0xC4 => Ok(Instruction::CPY(AddressingMode::ZeroPage)),
            0xCC => Ok(Instruction::CPY(AddressingMode::Absolute)),

            //DEC
            0xC6 => Ok(Instruction::DEC(AddressingMode::ZeroPage)),
            0xD6 => Ok(Instruction::DEC(AddressingMode::ZeroPageX)),
            0xCE => Ok(Instruction::DEC(AddressingMode::Absolute)),
            0xDE => Ok(Instruction::DEC(AddressingMode::AbsoluteX)),

            //EOR
            0x49 => Ok(Instruction::EOR(AddressingMode::Immediate)),
            0x45 => Ok(Instruction::EOR(AddressingMode::ZeroPage)),
            0x55 => Ok(Instruction::EOR(AddressingMode::ZeroPageX)),
            0x4D => Ok(Instruction::EOR(AddressingMode::Absolute)),
            0x5D => Ok(Instruction::EOR(AddressingMode::AbsoluteX)),
            0x59 => Ok(Instruction::EOR(AddressingMode::AbsoluteY)),
            0x41 => Ok(Instruction::EOR(AddressingMode::IndirectX)),
            0x51 => Ok(Instruction::EOR(AddressingMode::IndirectY)),

            //Flag
            0x18 => Ok(Instruction::CLC(AddressingMode::Implied)),
            0x38 => Ok(Instruction::SEC(AddressingMode::Implied)),
            0x58 => Ok(Instruction::CLI(AddressingMode::Implied)),
            0x78 => Ok(Instruction::SEI(AddressingMode::Implied)),
            0xB8 => Ok(Instruction::CLV(AddressingMode::Implied)),
            0xD8 => Ok(Instruction::CLD(AddressingMode::Implied)),
            0xF8 => Ok(Instruction::SED(AddressingMode::Implied)),

            //INC
            0xE6 => Ok(Instruction::INC(AddressingMode::ZeroPage)),
            0xF6 => Ok(Instruction::INC(AddressingMode::ZeroPageX)),
            0xEE => Ok(Instruction::INC(AddressingMode::Absolute)),
            0xFE => Ok(Instruction::INC(AddressingMode::AbsoluteX)),

            //JMP
            0x4C => Ok(Instruction::JMP(AddressingMode::Absolute)),
            0x6C => Ok(Instruction::JMP(AddressingMode::Indirect)),

            //JSR
            0x20 => Ok(Instruction::JSR(AddressingMode::Absolute)),

            //LDA
            0xA9 => Ok(Instruction::LDA(AddressingMode::Immediate)),
            0xA5 => Ok(Instruction::LDA(AddressingMode::ZeroPage)),
            0xB5 => Ok(Instruction::LDA(AddressingMode::ZeroPageX)),
            0xAD => Ok(Instruction::LDA(AddressingMode::Absolute)),
            0xBD => Ok(Instruction::LDA(AddressingMode::AbsoluteX)),
            0xB9 => Ok(Instruction::LDA(AddressingMode::AbsoluteY)),
            0xA1 => Ok(Instruction::LDA(AddressingMode::IndirectX)),
            0xB1 => Ok(Instruction::LDA(AddressingMode::IndirectY)),

            //LDX
            0xA2 => Ok(Instruction::LDX(AddressingMode::Immediate)),
            0xA6 => Ok(Instruction::LDX(AddressingMode::ZeroPage)),
            0xB6 => Ok(Instruction::LDX(AddressingMode::ZeroPageY)),
            0xAE => Ok(Instruction::LDX(AddressingMode::Absolute)),
            0xBE => Ok(Instruction::LDX(AddressingMode::AbsoluteY)),

            //LDY
            0xA0 => Ok(Instruction::LDY(AddressingMode::Immediate)),
            0xA4 => Ok(Instruction::LDY(AddressingMode::ZeroPage)),
            0xB4 => Ok(Instruction::LDY(AddressingMode::ZeroPageX)),
            0xAC => Ok(Instruction::LDY(AddressingMode::Absolute)),
            0xBC => Ok(Instruction::LDY(AddressingMode::AbsoluteX)),

            //LSR
            0x4A => Ok(Instruction::LSR(AddressingMode::Accumulator)),
            0x46 => Ok(Instruction::LSR(AddressingMode::ZeroPage)),
            0x56 => Ok(Instruction::LSR(AddressingMode::ZeroPageX)),
            0x4E => Ok(Instruction::LSR(AddressingMode::Absolute)),
            0x5E => Ok(Instruction::LSR(AddressingMode::AbsoluteX)),

            //NOP
            0xEA => Ok(Instruction::NOP(AddressingMode::Implied)),

            //ORA
            0x09 => Ok(Instruction::ORA(AddressingMode::Immediate)),
            0x05 => Ok(Instruction::ORA(AddressingMode::ZeroPage)),
            0x15 => Ok(Instruction::ORA(AddressingMode::ZeroPageX)),
            0x0D => Ok(Instruction::ORA(AddressingMode::Absolute)),
            0x1D => Ok(Instruction::ORA(AddressingMode::AbsoluteX)),
            0x19 => Ok(Instruction::ORA(AddressingMode::AbsoluteY)),
            0x01 => Ok(Instruction::ORA(AddressingMode::IndirectX)),
            0x11 => Ok(Instruction::ORA(AddressingMode::IndirectY)),

            //Register Instructions
            0xAA => Ok(Instruction::TAX(AddressingMode::Implied)),
            0x8A => Ok(Instruction::TXA(AddressingMode::Implied)),
            0xCA => Ok(Instruction::DEX(AddressingMode::Implied)),
            0xE8 => Ok(Instruction::INX(AddressingMode::Implied)),
            0xA8 => Ok(Instruction::TAY(AddressingMode::Implied)),
            0x98 => Ok(Instruction::TYA(AddressingMode::Implied)),
            0x88 => Ok(Instruction::DEY(AddressingMode::Implied)),
            0xC8 => Ok(Instruction::INY(AddressingMode::Implied)),

            //ROL
            0x2A => Ok(Instruction::ROL(AddressingMode::Accumulator)),
            0x26 => Ok(Instruction::ROL(AddressingMode::ZeroPage)),
            0x36 => Ok(Instruction::ROL(AddressingMode::ZeroPageX)),
            0x2E => Ok(Instruction::ROL(AddressingMode::Absolute)),
            0x3E => Ok(Instruction::ROL(AddressingMode::AbsoluteX)),

            //ROR
            0x6A => Ok(Instruction::ROR(AddressingMode::Accumulator)),
            0x66 => Ok(Instruction::ROR(AddressingMode::ZeroPage)),
            0x76 => Ok(Instruction::ROR(AddressingMode::ZeroPageX)),
            0x6E => Ok(Instruction::ROR(AddressingMode::Absolute)),
            0x7E => Ok(Instruction::ROR(AddressingMode::AbsoluteX)),

            //RTI
            0x40 => Ok(Instruction::RTI(AddressingMode::Implied)),

            //RTS
            0x60 => Ok(Instruction::RTS(AddressingMode::Implied)),

            //SBC
            0xE9 => Ok(Instruction::SBC(AddressingMode::Immediate)),
            0xE5 => Ok(Instruction::SBC(AddressingMode::ZeroPage)),
            0xF5 => Ok(Instruction::SBC(AddressingMode::ZeroPageX)),
            0xED => Ok(Instruction::SBC(AddressingMode::Absolute)),
            0xFD => Ok(Instruction::SBC(AddressingMode::AbsoluteX)),
            0xF9 => Ok(Instruction::SBC(AddressingMode::AbsoluteY)),
            0xE1 => Ok(Instruction::SBC(AddressingMode::IndirectX)),
            0xF1 => Ok(Instruction::SBC(AddressingMode::IndirectY)),

            //STA
            0x85 => Ok(Instruction::STA(AddressingMode::ZeroPage)),
            0x95 => Ok(Instruction::STA(AddressingMode::ZeroPageX)),
            0x8D => Ok(Instruction::STA(AddressingMode::Absolute)),
            0x9D => Ok(Instruction::STA(AddressingMode::AbsoluteX)),
            0x99 => Ok(Instruction::STA(AddressingMode::AbsoluteY)),
            0x81 => Ok(Instruction::STA(AddressingMode::IndirectX)),
            0x91 => Ok(Instruction::STA(AddressingMode::IndirectY)),

            //Stack Instructions
            0x9A => Ok(Instruction::TXS(AddressingMode::Implied)),
            0xBA => Ok(Instruction::TSX(AddressingMode::Implied)),
            0x48 => Ok(Instruction::PHA(AddressingMode::Implied)),
            0x68 => Ok(Instruction::PLA(AddressingMode::Implied)),
            0x08 => Ok(Instruction::PHP(AddressingMode::Implied)),
            0x28 => Ok(Instruction::PLP(AddressingMode::Implied)),

            //STX
            0x86 => Ok(Instruction::STX(AddressingMode::ZeroPage)),
            0x96 => Ok(Instruction::STX(AddressingMode::ZeroPageY)),
            0x8E => Ok(Instruction::STX(AddressingMode::Absolute)),

            //STY
            0x84 => Ok(Instruction::STY(AddressingMode::ZeroPage)),
            0x94 => Ok(Instruction::STY(AddressingMode::ZeroPageX)),
            0x8C => Ok(Instruction::STY(AddressingMode::Absolute)),

            //UNKNOWN INSTRUCTION
            _ => panic!("Unknown opcode: {:#X}", opcode),
        }
    }

    fn execute(&self, cpu: &mut Cpu) -> Result<(), MainError> {
        let operand_value = cpu.get_operand_value(self.get_addressing_mode())?;
        match self {
            Instruction::LDA(_) => {
                let value = operand_value.value.expect("LDA operand value is None");
                cpu.x_register.set(value);
                // set zero bit if the number read is 0
                if value == 0 {
                    cpu.status_register
                        .set_bit(StatusRegisterBit::ZeroBit, true);
                } else {
                    cpu.status_register
                        .set_bit(StatusRegisterBit::ZeroBit, true);
                }
                // set negative bit to the value of the 7th bit
                cpu.status_register
                    .set_bit(StatusRegisterBit::NegativeBit, (value as u8) > 127);
                Ok(())
            }

            // test instructions
            Instruction::LDX(_) => {
                let value = operand_value.value.expect("LDX operand value is None");
                cpu.x_register.set(value);
                // set zero bit if the number read is 0
                if value == 0 {
                    cpu.status_register
                        .set_bit(StatusRegisterBit::ZeroBit, true);
                } else {
                    cpu.status_register
                        .set_bit(StatusRegisterBit::ZeroBit, true);
                }
                // set negative bit to the value of the 7th bit
                cpu.status_register
                    .set_bit(StatusRegisterBit::NegativeBit, (value as u8) > 127);
                Ok(())
            }

            Instruction::LDY(_) => {
                let value = operand_value.value.expect("LDY operand value is None");
                cpu.y_register.set(value);
                // set zero bit if the number read is 0
                if value == 0 {
                    cpu.status_register
                        .set_bit(StatusRegisterBit::ZeroBit, true);
                } else {
                    cpu.status_register
                        .set_bit(StatusRegisterBit::ZeroBit, true);
                }
                // set negative bit to the value of the 7th bit
                cpu.status_register
                    .set_bit(StatusRegisterBit::NegativeBit, (value as u8) > 127);
                Ok(())
            }

            Instruction::STA(_) => {
                let address: u16 = operand_value.address.expect("STA Address is None");
                cpu.memory_write(address, cpu.accumulator.get())?;
                Ok(())
            }
            Instruction::STX(_) => {
                let address: u16 = operand_value.address.expect("STX Address is None");
                cpu.memory_write(address, cpu.x_register.get())?;
                Ok(())
            }

            Instruction::STY(_) => {
                let address: u16 = operand_value.address.expect("STY Address is None");
                cpu.memory_write(address, cpu.y_register.get())?;
                Ok(())
            }

            Instruction::TAX(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::TAY(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::TSX(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::TXA(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::TXS(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::TYA(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::PHA(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::PHP(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::PLA(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::PLP(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::DEC(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::DEX(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::DEY(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::INC(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::INX(_) => {
                let value: u8 = cpu.x_register.get().wrapping_add(1);
                cpu.x_register.set(value);

                // set zero bit if the number read is 0
                if value == 0 {
                    cpu.status_register
                        .set_bit(StatusRegisterBit::ZeroBit, true);
                } else {
                    cpu.status_register
                        .set_bit(StatusRegisterBit::ZeroBit, false);
                }
                // set negative bit to the value of the 7th bit
                cpu.status_register
                    .set_bit(StatusRegisterBit::NegativeBit, (value as u8) > 127);

                Ok(())
            }

            Instruction::INY(_) => {
                let value: u8 = cpu.y_register.get().wrapping_add(1);
                cpu.y_register.set(value);

                // set zero bit if the number read is 0
                if value == 0 {
                    cpu.status_register
                        .set_bit(StatusRegisterBit::ZeroBit, true);
                } else {
                    cpu.status_register
                        .set_bit(StatusRegisterBit::ZeroBit, false);
                }
                // set negative bit to the value of the 7th bit
                cpu.status_register
                    .set_bit(StatusRegisterBit::NegativeBit, (value as u8) > 127);

                Ok(())
            }

            Instruction::ADC(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::SBC(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::AND(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::EOR(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::ORA(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::ASL(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::LSR(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::ROL(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::ROR(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::CLC(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::CLD(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::CLI(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::CLV(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::SEC(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::SED(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::SEI(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::CMP(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::CPX(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::CPY(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::BCC(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::BCS(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::BEQ(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::BMI(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::BNE(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::BPL(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::BVC(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::BVS(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::JMP(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::JSR(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::RTS(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::BRK(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::RTI(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::BIT(_) => {
                //TODO: Implement
                Ok(())
            }

            Instruction::NOP(_) => {
                //TODO: Implement
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
            | Instruction::BPL(mode)
            | Instruction::BVC(mode)
            | Instruction::BVS(mode)
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
    tick_function: fn(),
}

impl Cpu {
    fn get_operand_value(
        &mut self,
        addressing_mode: &AddressingMode,
    ) -> Result<operand_value, MainError> {
        let mut hh: u8 = 0;
        let mut ll: u8 = 0;

        match addressing_mode.length() {
            1 => (),
            2 => ll = self.read_next_value()?,
            3 => {
                ll = self.read_next_value()?;
                hh = self.read_next_value()?;
            }
            _ => panic!("Unknown addressing mode"),
        }
        match addressing_mode {
            // A	        Accumulator	            OPC A	        operand is AC (implied single byte instruction)
            AddressingMode::Accumulator => Ok(operand_value {
                value: Some(self.accumulator.get()),
                address: None,
            }),

            // abs	        absolute	            OPC $LLHH	    operand is address $HHLL *
            AddressingMode::Absolute => {
                let address: u16 = (hh as u16) << 8 | ll as u16;
                Ok(operand_value {
                    address: Some(address),
                    value: Some(self.memory_read(address)?),
                })
            }

            // abs,X	    absolute, X-indexed	    OPC $LLHH,X	    operand is address; effective address is address incremented by X with carry **
            AddressingMode::AbsoluteX => {
                let address: u16 = (hh as u16) << 8 | ll as u16;
                Ok(operand_value {
                    address: Some(address + self.x_register.get() as u16),
                    value: Some(self.memory_read(address + self.y_register.get() as u16)?),
                })
            }

            // abs,Y	    absolute, Y-indexed	    OPC $LLHH,Y	    operand is address; effective address is address incremented by Y with carry **
            AddressingMode::AbsoluteY => {
                let address: u16 = (hh as u16) << 8 | ll as u16;
                Ok(operand_value {
                    address: Some(address + self.y_register.get() as u16),
                    value: Some(self.memory_read(address + self.y_register.get() as u16)?),
                })
            }

            // #	        immediate	            OPC #$BB	    operand is byte BB
            AddressingMode::Immediate => Ok(operand_value {
                value: Some(ll),
                address: None,
            }),

            // impl	        implied	                OPC	            operand implied
            AddressingMode::Implied => Ok(operand_value {
                value: None,
                address: None,
            }),

            // ind	        indirect	            OPC ($LLHH)	    operand is address; effective address is contents of word at address: C.w($HHLL)
            AddressingMode::Indirect => {
                let address: u16 = (hh as u16) << 8 | ll as u16;
                let memory_ll: u8 = self.memory_read(address)?;
                let memory_hh: u8 = self.memory_read(address + 1)?;
                let memory_address: u16 = (memory_hh as u16) << 8 | memory_ll as u16;
                Ok(operand_value {
                    address: Some(memory_address),
                    value: Some(self.memory_read(memory_address)?),
                })
            }

            // X,ind	    X-indexed, indirect	    OPC ($LL,X)	    operand is zeropage address; effective address is word in (LL + X, LL + X + 1), inc. without carry: C.w($00LL + X)
            AddressingMode::IndirectX => {
                let address: u16 = ll.saturating_add(self.x_register.get()) as u16;
                let memory_ll: u8 = self.memory_read(address)?;
                let memory_hh: u8 = self.memory_read(address + 1)?;
                let memory_address: u16 = (memory_hh as u16) << 8 | memory_ll as u16;
                Ok(operand_value {
                    address: Some(memory_address),
                    value: Some(self.memory_read(memory_address)?),
                })
            }

            // ind,Y	    indirect, Y-indexed	    OPC ($LL),Y	    operand is zeropage address; effective address is word in (LL, LL + 1) incremented by Y with carry: C.w($00LL) + Y
            AddressingMode::IndirectY => {
                let address: u16 = ll as u16;
                let memory_ll: u8 = self.memory_read(address)? + self.y_register.get();
                let memory_hh: u8 = self.memory_read(address + 1)?;
                let memory_address: u16 = (memory_hh as u16) << 8 | memory_ll as u16;
                Ok(operand_value {
                    address: Some(memory_address),
                    value: Some(self.memory_read(memory_address)?),
                })
            }

            // rel	        relative	            OPC $BB	        branch target is PC + signed offset BB ***
            AddressingMode::Relative => {
                let offset: i8 = ll as i8;
                Ok(operand_value {
                    value: None,
                    address: Some((self.program_counter.get() as i16 + offset as i16) as u16),
                })
            }

            // zpg	        zeropage	            OPC $LL	        operand is zeropage address (hi-byte is zero, address = $00LL)
            AddressingMode::ZeroPage => {
                let address: u16 = (0 as u16) << 8 | ll as u16;
                Ok(operand_value {
                    address: Some(address),
                    value: Some(self.memory_read(address)?),
                })
            }

            // zpg,X	    zeropage, X-indexed	    OPC $LL,X	    operand is zeropage address; effective address is address incremented by X without carry **
            AddressingMode::ZeroPageX => {
                let address: u16 = ll.saturating_add(self.x_register.get()) as u16;
                Ok(operand_value {
                    address: Some(address),
                    value: Some(self.memory_read(address)?),
                })
            }

            // zpg,Y	    zeropage, Y-indexed	    OPC $LL,Y	    operand is zeropage address; effective address is address incremented by Y without carry **
            AddressingMode::ZeroPageY => {
                let address: u16 = ll.saturating_add(self.x_register.get()) as u16;
                Ok(operand_value {
                    address: Some(address),
                    value: Some(self.memory_read(address)?),
                })
            }
        }
    }

    fn read_next_value(&mut self) -> Result<u8, MainError> {
        let value = self.memory.get_memory_byte(self.program_counter.get())?;
        self.program_counter.increment();
        Ok(value)
    }

    fn memory_read(&self, address: u16) -> Result<u8, MainError> {
        let memory_value = self.memory.get_memory_byte(address)?;
        Ok(memory_value)
    }

    fn memory_write(&mut self, address: u16, value: u8) -> Result<(), MainError> {
        self.memory.write_memory_byte(address, value)?;
        Ok(())
    }

    fn wait_clock_cycles(&self, cycles: u8) {
        for _ in 0..cycles {
            (self.tick_function)();
        }
    }

    pub fn link_tick_function(&mut self, function: fn()) {
        self.tick_function = function;
    }
}
