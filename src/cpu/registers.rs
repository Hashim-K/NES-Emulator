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
    pub(crate) fn set_bit(mut self, bit: StatusRegisterBit, value: bool) -> (){
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
    pub(crate) fn get(self) -> u8 {
        return self.binary_value;
    }

    pub(crate) fn set(&mut self, value: u8) -> (){
        self.binary_value = value;
    }
}

pub(crate) struct ProgramCounter {
    binary_value: u16,
}

impl ProgramCounter {
    pub(crate) fn get(self) -> u16 {
        return self.binary_value;
    }

    pub(crate) fn set(&mut self, value: u16) -> (){
        self.binary_value = value;
    }

    pub(crate) fn increment(&mut self) -> () {
        self.binary_value += 1;
    }
}