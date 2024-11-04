use log::warn;
use mapper0::Mapper0;
use mapper1::Mapper1;

use crate::error::{MemoryError, RomError};
use crate::memory::Mirroring;

mod mapper0;
mod mapper1;

pub trait Mapper: Send {
    fn read(&self, address: u16) -> Result<u8, RomError>;
    fn write(&mut self, address: u16, value: u8) -> Result<(), MemoryError>;
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

fn parse_header(rom_bytes: &[u8]) -> Result<RomHeader, RomError> {
    // Check rom signature
    if rom_bytes[0..4] != *(b"NES\x1a") {
        warn!("Found incorrect Ines header signature");
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
        mapper_number: (rom_bytes[6] >> 4) | (rom_bytes[7] & 0b11110000),
    })
}

pub fn get_mapper(rom: &[u8]) -> Result<Box<dyn Mapper + Send>, RomError> {
    let header = parse_header(rom)?;
    let mut total_length: u32 =
        header.charactor_memory_size as u32 * 8192 + header.program_rom_size as u32 * 16384;
    if header.trainer {
        total_length += 512
    }
    if rom[16..].len() != total_length as usize {
        return Err(RomError::IncorrectDataSize);
    }
    let prg_rom_start_index: usize = 16 + (header.trainer as usize) * 512;
    let prg_rom_end_index: usize =
        16 + (header.trainer as usize) * 512 + (header.program_rom_size as usize) * 0x4000;
    let mut prg_rom: Vec<u8> = rom[prg_rom_start_index..prg_rom_end_index].to_vec();
    let mut chr_rom: Vec<u8> = vec![];
    if header.charactor_memory_size != 0 {
        chr_rom.append(&mut rom[prg_rom_end_index..(rom.len() - 272)].to_vec());
    } else {
        let chr_ram: [u8; 8192] = [0; 8192];
        chr_rom.append(&mut chr_ram.to_vec());
    }
    if header.charactor_memory_size == 1 {
        prg_rom = [
            prg_rom,
            rom[prg_rom_start_index..prg_rom_end_index].to_vec(),
        ]
        .concat();
    }
    // type MapperType = Mapper0;
    match header.mapper_number {
        0 => Ok(Box::new(Mapper0::new(prg_rom, chr_rom, header.mirroring))),
        1 => Ok(Box::new(Mapper1::new(prg_rom, chr_rom, header.mirroring))),
        _ => Err(RomError::UnknownMapper(header.mapper_number)),
    }
}

#[cfg(test)]
use tudelft_nes_test::ROM_NROM_TEST;

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
    assert_eq!(parse_header(ROM_NROM_TEST).unwrap(), expected_header);
}
