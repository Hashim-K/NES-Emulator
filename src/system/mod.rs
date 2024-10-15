use tudelft_nes_test::TestableCpu;
use crate::error::{MyTickError, MyGetCpuError};
use tudelft_nes_ppu::{Cpu as CpuTemplate, Ppu};
use crate::memory::Memory;
use crate::cpu::Cpu;


pub struct System {
    memory: Memory,
    cpu: Cpu,
    ppu: Ppu,

}

/// See docs of `Cpu` for explanations of each function
impl CpuTemplate for System {
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
impl TestableCpu for System {
    type GetCpuError = MyGetCpuError;

    fn get_cpu(_rom: &[u8]) -> Result<Self, MyGetCpuError> {
        // return Ok(Cpu{ memory: Memory::new(_rom)? }
        todo!()
    }

    fn set_program_counter(&mut self, _value: u16) {
        todo!()
    }

    fn memory_read(&self, _address: u16) -> u8 {
        // return self.memory.get_memory_byte(_address).expect("Could not read from memory");
        todo!()
    }
}

