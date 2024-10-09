use crate::error::{RomError,MemoryError};
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
    pub fn new (rom_bytes: &[u8]) -> Result<Memory, RomError> {
        Ok(Memory { cartridge: Cartridge::new(rom_bytes)?, internal_ram: [0; 2048] })
    }

    fn get_memory_byte(self, address: u16) -> Result<u8, MemoryError> {
        match address{
            a if a <= 0x1fff => Ok(self.internal_ram[(a & 0x07ff) as usize]), // RAM reading, including mirroring
            a if a >= 0x2000 && a <= 0x3fff => { let register = address_to_ppu_register(a); todo!()}, // NES PPU registers
            a if a >= 0x4000 && a <= 0x4017 => todo!(), // NES APU and I/O registers
            a if a >= 0x4018 && a <= 0x401f => todo!(), // APU and I/O functionality that is normally disabled
            a if a >= 0x4020 => Ok(self.cartridge.get_memory_byte(a)?), // APU and I/O functionality that is normally disabled
            _ => Err(MemoryError::UnknownAddress)
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

    fn parse_header (rom_bytes: &[u8]) -> Result<RomHeader, RomError> {
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
            mapper_number: (rom_bytes[6] >> 4) & (rom_bytes[7] & 0b11110000)
        })    
    }

    fn new (rom_bytes: &[u8]) -> Result<Cartridge, RomError> {
        let header = Self::parse_header(rom_bytes)?;

        match header.mapper_number {
            0 => Ok(Cartridge { header, data: rom_bytes[16..].to_vec(), pgr_ram: [0; 8192]}),
            a => Err(RomError::UnknownMapper(a)),
        }
        // TODO: implement error handling
    }

    fn get_memory_byte(self, address: u16) -> Result<u8, RomError> {
        match address{
            a if a >= 0x6000 && a <= 0x7fff => Ok(self.pgr_ram[(a-0x6000) as usize]), // PGR RAM
            a if a >= 0x8000 && a <= 0xbfff => Ok(self.data[(a-0x8000) as usize]), // first 16 KiB of rom
            a if a >= 0xc000 => Ok(self.data[(a-0xc000 + 0x4000) as usize]), // second 16 KiB of rom
            _ => Err(RomError::UnknownAddress)
        }
    }
}

#[test]
fn test_parse_header () {
    let expected_header = RomHeader {
        mirroring: false,
        peristent_memory: false,
        ignore_mirroring_control: false,
        program_rom_size: 1,
        charactor_memory_size: 1,
        mapper_number: 0
    };
    assert_eq!(Cartridge::parse_header(ROM_NROM_TEST).unwrap(), expected_header);
}
