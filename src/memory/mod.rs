use crate::error::{MemoryError, RomError};
use tudelft_nes_ppu::{Mirroring, PpuRegister};

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

    pub fn write_ppu_byte(&mut self, address: u16, value: u8) -> Result<(), MemoryError> {
        self.cartridge.chr_data[address as usize] = value;
        Ok(())
    }

    pub fn read_ppu_byte(&self, address: u16) -> Result<u8, MemoryError> {
        return Ok(self.cartridge.chr_data[address as usize]);
    }

    pub fn write(&mut self, address: u16, value: u8) -> Result<(), MemoryError> {
        match address {
            ..0x2000 => self.internal_ram[(address & 0x07ff) as usize] = value, // RAM reading, including mirroring
            0x2000..0x4000 => {
                let _register = address_to_ppu_register(address);
                todo!();
            } // NES PPU registers
            0x4000..0x4018 => todo!(), // NES APU and I/O registers
            0x4018..0x4020 => todo!(), // APU and I/O functionality that is normally disabled
            0x4020.. => return self.cartridge.write(address, value), // Cartridge memory
        };

        Ok(())
    }

    pub fn read(&self, address: u16) -> Result<u8, MemoryError> {
        let value = match address {
            ..0x2000 => Ok(self.internal_ram[(address & 0x07ff) as usize]), // RAM reading, including mirroring
            0x2000..0x4000 => {
                let _register = address_to_ppu_register(address);
                todo!()
            } // NES PPU registers
            0x4000..0x4018 => todo!(), // NES APU and I/O registers
            0x4018..0x4020 => todo!(), // APU and I/O functionality that is normally disabled
            0x4020.. => Ok(self.cartridge.read(address)?), // Cartridge memory
        };

        // Debug printing
        if value.is_ok() {
            let tmp = value.unwrap();
            println!("Read memory byte at address {}: {:?}", address, tmp);
            return Ok(tmp);
        } else {
            println!("Read memory byte at address {}: FAILED", address);
        }
        return value;
    }

    pub fn get_mirroring(&self) -> Mirroring {
        self.cartridge.header.mirroring
    }
}

#[derive(Debug, PartialEq)]
pub struct RomHeader {
    mirroring: Mirroring,
    peristent_memory: bool,
    ignore_mirroring_control: bool,
    trainer: bool,
    program_rom_size: u8,
    program_ram_size: u8,
    charactor_memory_size: u8,
    mapper_number: u8,
}

#[derive(Debug, PartialEq)]
pub struct Cartridge {
    header: RomHeader,
    prg_data: Vec<u8>,
    chr_data: Vec<u8>,
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
            mirroring: if (rom_bytes[6] & 1) != 0 {
                Mirroring::Vertical
            } else {
                Mirroring::Horizontal
            },
            ignore_mirroring_control: (rom_bytes[6] >> 3 & 1) != 0,
            peristent_memory: (rom_bytes[6] >> 1 & 1) != 0,
            trainer: (rom_bytes[6] >> 2 & 1) != 0,
            program_ram_size: rom_bytes[8],
            mapper_number: (rom_bytes[6] >> 4) & (rom_bytes[7] & 0b11110000),
        })
    }

    fn new(rom_bytes: &[u8]) -> Result<Cartridge, RomError> {
        let header = Self::parse_header(rom_bytes)?;
        println!("{:?}", header.program_rom_size);
        println!("{:?}", header.charactor_memory_size);
        match header.mapper_number {
            0 => {
                // check if amount of data is correct
                if rom_bytes[16..].len()
                    != (header.charactor_memory_size as usize * 8192
                        + header.program_rom_size as usize * 16384)
                {
                    return Err(RomError::IncorrectDataSize);
                }
                let mut cartridge_prg_rom: Vec<u8> = rom_bytes[16..16400].to_vec();
                let mut Cartridge_chr_rom: Vec<u8> = rom_bytes[16400..].to_vec();
                if header.charactor_memory_size != 2 {
                    cartridge_prg_rom = [cartridge_prg_rom, rom_bytes[16..16400].to_vec()].concat();
                }
                Ok(Cartridge {
                    header,
                    prg_data: cartridge_prg_rom,
                    chr_data: Cartridge_chr_rom,
                    // pgr ram needs to mirror itself to fill 8kib
                    pgr_ram: [0; 8192],
                })
            }
            a => Err(RomError::UnknownMapper(a)),
        }
        // TODO: implement error handling
    }

    fn write(&mut self, address: u16, value: u8) -> Result<(), MemoryError> {
        match address {
            0x6000..0x8000 => self.pgr_ram[(address - 0x6000) as usize] = value, // PGR RAM
            0x8000..0xc000 => self.prg_data[(address - 0x8000) as usize] = value, // first 16 KiB of prg rom
            0xc000.. => self.prg_data[(address - 0xc000 + 0x4000) as usize] = value, // last 16 KiB of prg rom
            _ => return Err(MemoryError::UnknownAddress),
        }

        Ok(())
    }

    fn read(&self, address: u16) -> Result<u8, RomError> {
        match address {
            0x6000..0x8000 => Ok(self.pgr_ram[(address - 0x6000) as usize]), // PGR RAM
            0x8000..0xc000 => Ok(self.prg_data[(address - 0x8000) as usize]), // first 16 KiB of prg rom
            0xc000.. => Ok(self.prg_data[(address - 0xc000 + 0x4000) as usize]), // last 16 KiB of prg rom
            _ => Err(RomError::UnknownAddress),
        }
    }
}

#[test]
fn test_parse_header() {
    let expected_header = RomHeader {
        mirroring: Mirroring::Horizontal,
        trainer: false,
        peristent_memory: false,
        ignore_mirroring_control: false,
        program_ram_size: 0,
        program_rom_size: 1,
        charactor_memory_size: 1,
        mapper_number: 0,
    };
    assert_eq!(
        Cartridge::parse_header(ROM_NROM_TEST).unwrap(),
        expected_header
    );
}

#[ignore]
#[test]
fn test_new_cartridge() {
    let expected_header = RomHeader {
        mirroring: Mirroring::Horizontal,
        trainer: false,
        peristent_memory: false,
        ignore_mirroring_control: false,
        program_ram_size: 0,
        program_rom_size: 1,
        charactor_memory_size: 1,
        mapper_number: 0,
    };
    let expected_correct_cartridge: Cartridge = Cartridge {
        header: expected_header,
        prg_data: ROM_NROM_TEST[16..].to_vec(),
        chr_data: ROM_NROM_TEST[16..].to_vec(),
        pgr_ram: [0; 8192],
    };
    // expect file with exact amount of bytes as specified by the header to work (ROM_NROM_TEST length = 24592)
    assert_eq!(
        Cartridge::new(&ROM_NROM_TEST).unwrap(),
        expected_correct_cartridge
    );
    // expect a file that does not have the amount of bytes specified by the header to generate an error
    assert_eq!(
        Cartridge::new(&ROM_NROM_TEST[0..24231]).unwrap_err(),
        RomError::IncorrectDataSize
    );
}
