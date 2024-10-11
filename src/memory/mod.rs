use crate::error::{MemoryError, RomError};
use tudelft_nes_ppu::PpuRegister;

#[cfg(test)]
use tudelft_nes_test::ROM_NROM_TEST;

fn address_to_ppu_register(a: u16) -> PpuRegister {
    let reg_num = (a & 0b111) as u8; // Translate address to register number
    unsafe { std::mem::transmute(reg_num) } // Translate register number to enum
}

#[test]
fn test_address_to_ppu_register() {
    assert_eq!(address_to_ppu_register(0x2000), PpuRegister::Controller);
    assert_eq!(address_to_ppu_register(0x2002), PpuRegister::Status);
    assert_eq!(address_to_ppu_register(0x3456), PpuRegister::Address);
    assert_eq!(address_to_ppu_register(0x345e), PpuRegister::Address);
    assert_eq!(address_to_ppu_register(0x3fff), PpuRegister::Data);
}

pub struct Memory {
    internal_ram: [u8; 2048],
    cartridge: Cartridge,
}

impl Memory {
    pub fn new(rom_bytes: &[u8]) -> Result<Memory, RomError> {
        Ok(Memory {
            cartridge: Cartridge::new(rom_bytes)?,
            internal_ram: [0; 2048],
        })
    }

    pub fn write_memory_byte(mut self, address: u16, value: u8) -> Option<MemoryError> {
        match address {
            ..0x2000 => self.internal_ram[(address & 0x07ff) as usize] = value, // RAM reading, including mirroring
            0x2000..0x4000 => {
                let register = address_to_ppu_register(address);
                todo!();
            } // NES PPU registers
            0x4000..0x4018 => todo!(), // NES APU and I/O registers
            0x4018..0x4020 => todo!(), // APU and I/O functionality that is normally disabled
            0x4020.. => return self.cartridge.write_memory_byte(address, value), // Cartridge memory
        };

        return None;
    }

    pub fn get_memory_byte(&self, address: u16) -> Result<u8, MemoryError> {
        match address {
            ..0x2000 => Ok(self.internal_ram[(address & 0x07ff) as usize]), // RAM reading, including mirroring
            0x2000..0x4000 => {
                let register = address_to_ppu_register(address);
                todo!()
            } // NES PPU registers
            0x4000..0x4018 => todo!(), // NES APU and I/O registers
            0x4018..0x4020 => todo!(), // APU and I/O functionality that is normally disabled
            0x4020.. => Ok(self.cartridge.get_memory_byte(address)?), // Cartridge memory
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RomHeader {
    mirroring: bool,
    peristent_memory: bool,
    ignore_mirroring_control: bool,
    program_rom_size: u8,
    charactor_memory_size: u8,
    mapper_number: u8,
}

pub struct Cartridge {
    header: RomHeader,
    data: Vec<u8>,
    pgr_ram: [u8; 8192], // 8 KiB of program ram
}

impl Cartridge {
    fn parse_header(rom_bytes: &[u8]) -> Result<RomHeader, RomError> {
        // Check rom signature
        if rom_bytes[0..4] != *(b"NES\x1a") {
            println!("{:?}", b"NES\x1a");
            println!("{:?}", &rom_bytes[0..4]);
            return Err(RomError::IncorrectSignature);
        }

        // Parse rom header
        Ok(RomHeader {
            program_rom_size: rom_bytes[4],
            charactor_memory_size: rom_bytes[5],
            mirroring: (rom_bytes[6] & 1) != 0,
            ignore_mirroring_control: (rom_bytes[6] >> 3 & 1) != 0,
            peristent_memory: (rom_bytes[6] >> 1 & 1) != 0,
            mapper_number: (rom_bytes[6] >> 4) & (rom_bytes[7] & 0b11110000),
        })
    }

    fn new(rom_bytes: &[u8]) -> Result<Cartridge, RomError> {
        let header = Self::parse_header(rom_bytes)?;

        match header.mapper_number {
            0 => Ok(Cartridge {
                header,
                data: rom_bytes[16..].to_vec(),
                pgr_ram: [0; 8192],
            }),
            a => Err(RomError::UnknownMapper(a)),
        }
        // TODO: implement error handling
    }

    fn write_memory_byte(mut self, address: u16, value: u8) -> Option<MemoryError> {
        match address {
            0x6000..0x8000 => self.pgr_ram[(address - 0x6000) as usize] = value, // PGR RAM
            0x8000..0xc000 => self.data[(address - 0x8000) as usize] = value, // first 16 KiB of rom
            0xc000.. => self.data[(address - 0xc000 + 0x4000) as usize] = value, // second 16 KiB of rom
            _ => return Some(MemoryError::UnknownAddress),
        }

        return None;
    }

    fn get_memory_byte(&self, address: u16) -> Result<u8, RomError> {
        match address {
            0x6000..0x8000 => Ok(self.pgr_ram[(address - 0x6000) as usize]), // PGR RAM
            0x8000..0xc000 => Ok(self.data[(address - 0x8000) as usize]),    // first 16 KiB of rom
            0xc000.. => Ok(self.data[(address - 0xc000 + 0x4000) as usize]), // second 16 KiB of rom
            _ => Err(RomError::UnknownAddress),
        }
    }
}

#[test]
fn test_parse_header() {
    let expected_header = RomHeader {
        mirroring: false,
        peristent_memory: false,
        ignore_mirroring_control: false,
        program_rom_size: 1,
        charactor_memory_size: 1,
        mapper_number: 0,
    };
    assert_eq!(
        Cartridge::parse_header(ROM_NROM_TEST).unwrap(),
        expected_header
    );
}
