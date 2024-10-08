use crate::error::RomError;

#[cfg(test)]
use tudelft_nes_test::ROM_NROM_TEST;

#[derive(Debug, PartialEq)]
pub struct RomHeader {
    mirroring: bool,
    peristent_memory: bool,
    ignore_mirroring_control: bool,
    program_rom_size: u8,
    charactor_memory_size: u8,
    mapper_number: u8,
}

pub struct Cartrigde {
}

impl Cartrigde {

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

    fn parse_rom (rom_bytes: &[u8]) {
        todo!()
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
    assert_eq!(Cartrigde::parse_header(ROM_NROM_TEST).unwrap(), expected_header);
}
