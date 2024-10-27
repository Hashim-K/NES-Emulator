#[derive(Clone, Copy, Debug)]
pub(crate) enum StatusRegisterBit {
    Carry,
    Zero,
    Interrupt,
    Decimal,
    Overflow,
    Negative,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct StatusRegister {
    carry_bit: bool,
    zero_bit: bool,
    interrupt_bit: bool,
    decimal_bit: bool,
    overflow_bit: bool,
    negative_bit: bool,
}

impl StatusRegister {
    pub(crate) fn set_bit(&mut self, bit: StatusRegisterBit, value: bool) {
        match bit {
            StatusRegisterBit::Carry => self.carry_bit = value,
            StatusRegisterBit::Zero => self.zero_bit = value,
            StatusRegisterBit::Interrupt => self.interrupt_bit = value,
            StatusRegisterBit::Decimal => self.decimal_bit = value,
            StatusRegisterBit::Overflow => self.overflow_bit = value,
            StatusRegisterBit::Negative => self.negative_bit = value,
        }
    }

    pub(crate) fn get_for_stack(self) -> u8 {
        (self.carry_bit as u8)
            | (self.zero_bit as u8) << 1
            | (self.interrupt_bit as u8) << 2
            | (self.decimal_bit as u8) << 3
            | 1 << 4
            | 1 << 5
            | (self.overflow_bit as u8) << 6
            | (self.negative_bit as u8) << 7
    }

    pub(crate) fn get_for_debug(self) -> u8 {
        (self.carry_bit as u8) << 0
            | (self.zero_bit as u8) << 1
            | (self.interrupt_bit as u8) << 2
            | (self.decimal_bit as u8) << 3
            | 1 << 5
            | (self.overflow_bit as u8) << 6
            | (self.negative_bit as u8) << 7
    }

    pub(crate) fn set_from_stack(&mut self, value: u8) {
        self.carry_bit = (value & 1 << 0) != 0;
        self.zero_bit = (value & 1 << 1) != 0;
        self.interrupt_bit = (value & 1 << 2) != 0;
        self.decimal_bit = (value & 1 << 3) != 0;
        self.overflow_bit = (value & 1 << 6) != 0;
        self.negative_bit = (value & 1 << 7) != 0;
    }

    pub(crate) fn get_bit(&mut self, bit: StatusRegisterBit) -> bool {
        match bit {
            StatusRegisterBit::Carry => self.carry_bit,
            StatusRegisterBit::Zero => self.zero_bit,
            StatusRegisterBit::Interrupt => self.interrupt_bit,
            StatusRegisterBit::Decimal => self.decimal_bit,
            StatusRegisterBit::Overflow => self.overflow_bit,
            StatusRegisterBit::Negative => self.negative_bit,
        }
    }

    pub(crate) fn get_carry(self) -> bool {
        self.carry_bit
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct CpuRegister {
    binary_value: u8,
}

impl CpuRegister {
    pub(crate) fn get(&self) -> u8 {
        self.binary_value
    }

    pub(crate) fn set(&mut self, value: u8) {
        self.binary_value = value;
    }

    pub(crate) fn increment(&mut self) {
        self.binary_value = self.binary_value.wrapping_add(1);
    }

    pub(crate) fn decrement(&mut self) {
        self.binary_value = self.binary_value.wrapping_sub(1);
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ProgramCounter {
    binary_value: u16,
}

impl ProgramCounter {
    pub(crate) fn get(&self) -> u16 {
        self.binary_value
    }

    pub(crate) fn set(&mut self, value: u16) {
        self.binary_value = value;
    }

    pub(crate) fn get_lobyte(&self) -> u8 {
        self.binary_value as u8
    }

    pub(crate) fn get_hibyte(&self) -> u8 {
        (self.binary_value >> 8) as u8
    }

    pub(crate) fn set_lobyte(&mut self, value: u8) {
        self.binary_value = (self.binary_value & 0xFF00) | value as u16;
    }

    pub(crate) fn set_hibyte(&mut self, value: u8) {
        self.binary_value = (self.binary_value & 0x00FF) | ((value as u16) << 8);
    }

    pub(crate) fn increment(&mut self) {
        self.binary_value = self.binary_value.wrapping_add(1);
    }

    pub(crate) fn new() -> Self {
        ProgramCounter {
            binary_value: 0xFFFC,
        }
    }

    // pub(crate) fn reset(&mut self) {
    //     //TODO: Doublecheck reset value
    //     self.binary_value = 0xFFFC;
    // }
}

//
//
//
//
//
//
//
//  ooooooooooooo      oooooooooooo       .oooooo..o      ooooooooooooo       .oooooo..o
//  8'   888   `8      `888'     `8      d8P'    `Y8      8'   888   `8      d8P'    `Y8
//       888            888              Y88bo.                888           Y88bo.
//       888            888oooo8          `"Y8888o.            888            `"Y8888o.
//       888            888    "              `"Y88b           888                `"Y88b
//       888            888       o      oo     .d8P           888           oo     .d8P
//      o888o          o888ooooood8      8""88888P'           o888o          8""88888P'
//
//
//
//
//
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_register_set_bit() {
        let mut sr = StatusRegister::default();

        // Test setting the CarryBit to true
        sr.set_bit(StatusRegisterBit::Carry, true);
        assert!(sr.carry_bit);

        // Test setting the ZeroBit to true
        sr.set_bit(StatusRegisterBit::Zero, true);
        assert!(sr.zero_bit);

        // Test setting the InterruptBit to false
        sr.set_bit(StatusRegisterBit::Interrupt, false);
        assert!(!sr.interrupt_bit);

        // Test setting the DecimalBit to true
        sr.set_bit(StatusRegisterBit::Decimal, true);
        assert!(sr.decimal_bit);

        // Test setting the OverflowBit to true
        sr.set_bit(StatusRegisterBit::Overflow, true);
        assert!(sr.overflow_bit);

        // Test setting the NegativeBit to false
        sr.set_bit(StatusRegisterBit::Negative, false);
        assert!(!sr.negative_bit);
    }

    #[test]
    fn test_cpu_register_get_set() {
        let mut cpu_reg = CpuRegister { binary_value: 0 };

        // Test setting a value
        cpu_reg.set(42);
        assert_eq!(cpu_reg.get(), 42);

        // Test setting another value
        cpu_reg.set(255);
        assert_eq!(cpu_reg.get(), 255);
    }

    #[test]
    fn test_cpu_register_increment_decrement() {
        let mut cpu_reg = CpuRegister { binary_value: 0xFF };

        cpu_reg.increment();
        assert_eq!(cpu_reg.get(), 0);

        cpu_reg.increment();
        assert_eq!(cpu_reg.get(), 1);

        cpu_reg.decrement();
        assert_eq!(cpu_reg.get(), 0);

        cpu_reg.decrement();
        assert_eq!(cpu_reg.get(), 0xFF);
    }

    #[test]
    fn test_program_counter_get_set() {
        let mut pc: ProgramCounter = ProgramCounter { binary_value: 0 };

        // Test setting a value
        pc.set(0x1234);
        assert_eq!(pc.get(), 0x1234);

        // Test setting another value
        pc.set(0xFFFF);
        assert_eq!(pc.get(), 0xFFFF);

        pc.set(0x1111);
        pc.set_hibyte(0xFF);
        assert_eq!(pc.get(), 0xFF11);
        assert_eq!(pc.get_hibyte(), 0xFF);

        pc.set(0x1111);
        pc.set_lobyte(0xFF);
        assert_eq!(pc.get(), 0x11FF);
        assert_eq!(pc.get_lobyte(), 0xFF);

        pc.set_hibyte(0xFF);
        assert_eq!(pc.get(), 0xFFFF);
        pc.set_lobyte(0xFF);
        assert_eq!(pc.get(), 65535);
    }

    #[test]
    fn test_program_counter_increment() {
        let mut pc: ProgramCounter = ProgramCounter {
            binary_value: 0xFFFF,
        };

        pc.increment();
        assert_eq!(pc.get(), 0);

        pc.increment();
        assert_eq!(pc.get(), 1);
    }

    // #[test]
    // fn test_program_counter_reset() {
    //     let mut pc = ProgramCounter { binary_value: 0xABCD };

    //     pc.reset();
    //     assert_eq!(pc.get(), 0xFFFC)
    // }
}
