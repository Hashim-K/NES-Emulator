#[derive(Clone, Copy, Debug)]
pub(crate) enum StatusRegisterBit {
    CarryBit,
    ZeroBit,
    InterruptBit,
    DecimalBit,
    BreakBit,
    OverflowBit,
    NegativeBit,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct StatusRegister {
    carry_bit: bool,
    zero_bit: bool,
    interrupt_bit: bool,
    decimal_bit: bool,
    break_bit: bool,
    overflow_bit: bool,
    negative_bit: bool,    
}


impl StatusRegister {
    pub(crate) fn set_bit(&mut self, bit: StatusRegisterBit, value: bool) -> (){
        match bit {
            StatusRegisterBit::CarryBit => self.carry_bit = value,
            StatusRegisterBit::ZeroBit => self.zero_bit = value,
            StatusRegisterBit::InterruptBit => self.interrupt_bit = value,
            StatusRegisterBit::DecimalBit => self.decimal_bit = value,
            StatusRegisterBit::BreakBit => self.break_bit = value,
            StatusRegisterBit::OverflowBit => self.overflow_bit = value,
            StatusRegisterBit::NegativeBit => self.negative_bit = value,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CpuRegister {
    binary_value: u8,
}

impl CpuRegister {
    pub(crate) fn get(&self) -> u8 {
        return self.binary_value;
    }

    pub(crate) fn set(&mut self, value: u8) -> (){
        self.binary_value = value;
    }

    pub(crate) fn increment(&mut self) -> (){
        self.binary_value = self.binary_value.wrapping_add(1);
    }
}

pub(crate) struct ProgramCounter {
    binary_value: u16,
}

impl ProgramCounter {
    pub(crate) fn get(&self) -> u16 {
        return self.binary_value;
    }

    pub(crate) fn set(&mut self, value: u16) -> (){
        self.binary_value = value;
    }

    //TODO: implement endianness-dependent version
    // pub(crate) fn set_lobit(&mut self, value: u8) {
    //     let addition_value: u16 = value as u16;
    //     self.binary_value = self.binary_value & 0b0000_0000_1111_1111;
    // }

    // pub(crate) fn set_hibit(&mut self, value: u8) {
    //     let comparison_value: u16 = (value as u16) << 8;
    //     self.binary_value = self.binary_value & comparison_value;
    // }

    pub(crate) fn increment(&mut self) -> () {
        self.binary_value = self.binary_value.wrapping_add(1);
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
        sr.set_bit(StatusRegisterBit::CarryBit, true);
        assert_eq!(sr.carry_bit, true);

        // Test setting the ZeroBit to true
        sr.set_bit(StatusRegisterBit::ZeroBit, true);
        assert_eq!(sr.zero_bit, true);

        // Test setting the InterruptBit to false
        sr.set_bit(StatusRegisterBit::InterruptBit, false);
        assert_eq!(sr.interrupt_bit, false);

        // Test setting the DecimalBit to true
        sr.set_bit(StatusRegisterBit::DecimalBit, true);
        assert_eq!(sr.decimal_bit, true);

        // Test setting the BreakBit to false
        sr.set_bit(StatusRegisterBit::BreakBit, false);
        assert_eq!(sr.break_bit, false);

        // Test setting the OverflowBit to true
        sr.set_bit(StatusRegisterBit::OverflowBit, true);
        assert_eq!(sr.overflow_bit, true);

        // Test setting the NegativeBit to false
        sr.set_bit(StatusRegisterBit::NegativeBit, false);
        assert_eq!(sr.negative_bit, false);
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
    fn test_cpu_register_increment() {
        let mut cpu_reg = CpuRegister { binary_value: 0xFF };

        cpu_reg.increment();
        assert_eq!(cpu_reg.get(), 0);

        cpu_reg.increment();
        assert_eq!(cpu_reg.get(), 1);
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
    }

    #[test]
    fn test_program_counter_increment() {
        let mut pc: ProgramCounter = ProgramCounter { binary_value: 0xFFFF };

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