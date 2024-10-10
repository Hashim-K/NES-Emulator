use crate::error::{MyGetCpuError, MyTickError};
use tudelft_nes_ppu::{Cpu, Ppu};
use tudelft_nes_test::TestableCpu;

pub struct MyCpu {
    pub rom: Vec<u8>,
}

/// See docs of `Cpu` for explanations of each function
impl Cpu for MyCpu {
    type TickError = MyTickError;

    fn tick(&mut self, _ppu: &mut Ppu) -> Result<(), MyTickError> {
        todo!()
    }

    fn ppu_read_chr_rom(&self, _offset: u16) -> u8 {
        todo!()
    }

    fn non_maskable_interrupt(&mut self) {
        todo!()
    }
}

/// Implementing this trait allows automated tests to be run on your cpu.
/// The crate `tudelft-nes-test` contains all kinds of small and large scale
/// tests to find bugs in your cpu.
impl TestableCpu for MyCpu {
    type GetCpuError = MyGetCpuError;

    fn get_cpu(_rom: &[u8]) -> Result<Self, MyGetCpuError> {
        Ok(MyCpu { rom: _rom.to_vec() })
    }

    fn set_program_counter(&mut self, _value: u16) {
        todo!()
    }

    fn memory_read(&self, _address: u16) -> u8 {
        todo!()
    }
}
