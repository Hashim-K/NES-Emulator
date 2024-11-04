use crate::error::{MemoryError, RomError};
use crate::memory::mapper::Mapper;
use crate::memory::Mirroring;

#[derive(Debug, PartialEq)]
pub struct Mapper0 {
    chr_rom: Vec<u8>,
    prg_rom: Vec<u8>,
    prg_ram: [u8; 0x2000],
}

impl Mapper0 {
    pub fn new(prg_rom: Vec<u8>, chr_rom: Vec<u8>, _mirroring: Mirroring) -> Mapper0 {
        Self {
            prg_rom,
            chr_rom,
            prg_ram: [0; 0x2000],
        }
    }
}

impl Mapper for Mapper0 {
    fn write(&mut self, address: u16, value: u8) -> Result<(), MemoryError> {
        match address {
            0x6000..0x8000 => self.prg_ram[(address - 0x6000) as usize] = value, // PGR RAM
            0x8000..0xc000 => self.prg_rom[(address - 0x8000) as usize] = value, // first 16 KiB of prg rom
            0xc000.. => self.prg_rom[(address - 0xc000 + 0x4000) as usize] = value, // last 16 KiB of prg rom
            _ => return Err(MemoryError::UnknownAddress),
        }
        Ok(())
    }

    fn read(&self, address: u16) -> Result<u8, RomError> {
        match address {
            0x6000..0x8000 => Ok(self.prg_ram[(address - 0x6000) as usize]), // PGR RAM
            0x8000.. => Ok(self.prg_rom[address as usize % self.prg_rom.len()]), // first 16 KiB of prg rom
            _ => Err(RomError::UnknownAddress),
        }
    }
}
