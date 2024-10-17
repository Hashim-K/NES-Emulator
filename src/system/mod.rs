use crate::cpu::Cpu;
use crate::error::{MyGetCpuError, MyTickError};
use crate::memory::Memory;
use tudelft_nes_ppu::{Cpu as CpuTemplate, Ppu};
use tudelft_nes_test::TestableCpu;

pub struct System {
    memory: Memory,
    cpu: Cpu,
    ppu: Ppu,
}

/// See docs of `Cpu` for explanations of each function
impl CpuTemplate for System {
    type TickError = MyTickError;

    fn tick(&mut self, _ppu: &mut Ppu) -> Result<(), MyTickError> {
        self.cpu.tick(&mut self.memory)
    }

    fn ppu_read_chr_rom(&self, _offset: u16) -> u8 {
        self.memory
            .read(_offset)
            .expect("Failed reading character ROM")
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
        let memory = Memory::new(_rom)?;
        let cpu = Cpu::new();
        let ppu = Ppu::new(memory.get_mirroring());

        return Ok(System { memory, cpu, ppu });
    }

    fn set_program_counter(&mut self, _value: u16) {
        todo!()
    }

    fn memory_read(&self, _address: u16) -> u8 {
        return self
            .memory
            .read(_address)
            .expect("Could not read from memory");
    }
}
