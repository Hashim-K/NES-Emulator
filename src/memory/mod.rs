use crate::cpu::Cpu;
use crate::error::{MemoryError, RomError};
use tudelft_nes_ppu::{Mirroring, Ppu, PpuRegister};

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

#[derive(Debug)]
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

    pub fn write(&mut self, address: u16, value: u8, ppu: &mut Ppu) -> Result<(), MemoryError> {
        println!(
            "Writing value 0x{:02X} to address: 0x{:04X}",
            value, address
        );
        match address {
            ..0x2000 => self.internal_ram[(address & 0x07ff) as usize] = value, // RAM reading, including mirroring
            0x2000..0x4000 => {
                let _register = address_to_ppu_register(address);
                ppu.write_ppu_register(_register, value)
            } // NES PPU registers
            0x4000..0x4018 => todo!(), // NES APU and I/O registers
            0x4018..0x4020 => todo!(), // APU and I/O functionality that is normally disabled
            0x4020.. => return self.cartridge.write(address, value), // Cartridge memory
        };

        Ok(())
    }

    pub fn read(&self, address: u16, cpu: &Cpu, ppu: &mut Ppu) -> Result<u8, MemoryError> {
        let value = match address {
            0x2000..0x4000 => {
                let register = address_to_ppu_register(address);
                Ok(ppu.read_ppu_register(register, cpu))
            }
            _ => self.read_cpu_mem(address),
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

    pub fn read_cpu_mem(&self, address: u16) -> Result<u8, MemoryError> {
        match address {
            // RAM reading, including mirroring
            ..0x2000 => Ok(self.internal_ram[(address & 0x07ff) as usize]),
            // NES PPU registers
            0x2000..0x4000 => {
                panic!("You have to use the read function if you want to access the ppu memory")
            }
            // NES APU and I/O registers
            0x4000..0x4018 => {
                // TODO: Implement APU and I/O registers
                Ok(0)
            }
            // APU and I/O functionality that is normally disabled
            0x4018..0x4020 => {
                // TODO: Implement APU and I/O functionality
                Ok(0)
            }
            // Cartridge memory
            0x4020.. => Ok(self.cartridge.read(address)?),
        }
    }

    pub fn get_mirroring(&self) -> Mirroring {
        self.cartridge.header.mirroring
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum ProgramBankMode {
    Fullswitch,
    Fixfirst,
    Fixlast,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum CharacterBankMode {
    Fullswitch,
    Halfswitch,
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
    prg_bank: u8,
    chr_bank_0: u8,
    chr_bank_1: u8,
    shift_register: u8,
    prg_bank_mode: ProgramBankMode,
    chr_bank_mode: CharacterBankMode,
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
        let mut total_length: u32 =
            header.charactor_memory_size as u32 * 8192 + header.program_rom_size as u32 * 16384;
        if header.trainer {
            total_length += 512
        }
        if rom_bytes[16..].len() != total_length as usize {
            return Err(RomError::IncorrectDataSize);
        }
        let prg_rom_start_index: usize = 16 + (header.trainer as usize) * 512 as usize;
        let prg_rom_end_index: usize =
            16 + (header.trainer as usize) * 512 + (header.program_rom_size as usize) * 16384;
        println!("{:?}", prg_rom_start_index);
        println!("{:?}", prg_rom_end_index);
        let mut cartridge_prg_rom: Vec<u8> =
            rom_bytes[prg_rom_start_index..prg_rom_end_index].to_vec();
        let mut cartridge_chr_rom: Vec<u8> = vec![];
        if header.charactor_memory_size != 0 {
            cartridge_chr_rom.append(&mut rom_bytes[(prg_rom_end_index + 1)..].to_vec());
        } else {
            let chr_ram: [u8; 8192] = [0; 8192];
            cartridge_chr_rom.append(&mut chr_ram.to_vec());
        }
        if header.charactor_memory_size != 2 {
            cartridge_prg_rom = [
                cartridge_prg_rom,
                rom_bytes[prg_rom_start_index..prg_rom_end_index].to_vec(),
            ]
            .concat();
        }
        Ok(Cartridge {
            header,
            prg_data: cartridge_prg_rom,
            chr_data: cartridge_chr_rom,
            prg_bank: 1,
            chr_bank_0: 1,
            chr_bank_1: 2,
            shift_register: 16,
            prg_bank_mode: ProgramBankMode::Fixlast,
            chr_bank_mode: CharacterBankMode::Fullswitch,
            // pgr ram needs to mirror itself to fill 8kib
            pgr_ram: [0; 8192],
        })
        // TODO: implement error handling
    }

    fn write(&mut self, address: u16, value: u8) -> Result<(), MemoryError> {
        match self.header.mapper_number {
            0 => {
                match address {
                    0x6000..0x8000 => self.pgr_ram[(address - 0x6000) as usize] = value, // PGR RAM
                    0x8000..0xc000 => self.prg_data[(address - 0x8000) as usize] = value, // first 16 KiB of prg rom
                    0xc000.. => self.prg_data[(address - 0xc000 + 0x4000) as usize] = value, // last 16 KiB of prg rom
                    _ => return Err(MemoryError::UnknownAddress),
                }
            }
            1 => {
                if (self.shift_register & 1) != 1 {
                    self.shift_register = (self.shift_register >> 1) + ((value & 1) << 4)
                } else {
                    match address {
                        0x8000..0xa000 => {
                            match self.shift_register & 3 {
                                0 => self.header.mirroring = Mirroring::SingleScreenLower,
                                1 => self.header.mirroring = Mirroring::SingleScreenUpper,
                                2 => self.header.mirroring = Mirroring::Horizontal,
                                3 => self.header.mirroring = Mirroring::Vertical,
                                _ => return Err(MemoryError::ShiftAddressError),
                            }
                            match (self.shift_register >> 2) & 3 {
                                0 | 1 => self.prg_bank_mode = ProgramBankMode::Fullswitch,
                                2 => self.prg_bank_mode = ProgramBankMode::Fixfirst,
                                3 => self.prg_bank_mode = ProgramBankMode::Fixlast,
                                _ => return Err(MemoryError::ShiftAddressError),
                            }
                            if (self.shift_register >> 4) & 1 == 0 {
                                self.chr_bank_mode = CharacterBankMode::Fullswitch
                            } else {
                                self.chr_bank_mode = CharacterBankMode::Halfswitch
                            }
                        }
                        0xa000..0xc000 => self.chr_bank_0 = self.shift_register,
                        0xc000..0xe000 => self.chr_bank_1 = self.shift_register,
                        0xe000.. => self.prg_bank = self.shift_register,
                        _ => return Err(MemoryError::MapperAddressError(address)),
                    }
                }
            }
            a => Err(RomError::UnknownMapper(a))?,
        }

        Ok(())
    }

    fn read(&self, address: u16) -> Result<u8, RomError> {
        match self.header.mapper_number {
            0 => {
                match address {
                    0x6000..0x8000 => Ok(self.pgr_ram[(address - 0x6000) as usize]), // PGR RAM
                    0x8000..0xc000 => Ok(self.prg_data[(address - 0x8000) as usize]), // first 16 KiB of prg rom
                    0xc000.. => Ok(self.prg_data[(address - 0xc000 + 0x4000) as usize]), // last 16 KiB of prg rom
                    _ => return Err(RomError::UnknownAddress),
                }
            }
            1 => {
                match self.prg_bank_mode {
                    ProgramBankMode::Fullswitch => {
                        let banknr = self.prg_bank >> 1;
                        match address {
                            0x6000..0x8000 => Ok(self.pgr_ram[(address - 0x6000) as usize]), // PGR RAM
                            0x8000.. => Ok(self.prg_data
                                [(address - 0x8000 + ((banknr - 1) as u16) * 16384) as usize]), // switch in 32kb blocks
                            _ => return Err(RomError::UnknownAddress),
                        }
                    }
                    ProgramBankMode::Fixfirst => {
                        match address {
                            0x6000..0x8000 => Ok(self.pgr_ram[(address - 0x6000) as usize]), // PGR RAM
                            0x8000..0xc000 => Ok(self.prg_data[(address - 0x8000) as usize]), // fix first bank to 0x8000
                            0xc000.. => Ok(self.prg_data[(address - 0xc000
                                + 0x4000
                                + ((self.prg_bank - 1) as u16) * 16384)
                                as usize]), // make 0x8000 - 0xc000 switchable
                            _ => return Err(RomError::UnknownAddress),
                        }
                    }
                    ProgramBankMode::Fixlast => {
                        match address {
                            0x6000..0x8000 => Ok(self.pgr_ram[(address - 0x6000) as usize]), // PGR RAM
                            0x8000..0xc000 => Ok(self.prg_data[(address - 0x8000
                                + ((self.prg_bank - 1) as u16) * 16384)
                                as usize]), // make 0x8000 - 0xc000 switchable
                            0xc000.. => Ok(self.prg_data[(address - 0xc000
                                + 0x4000
                                + (self.header.program_rom_size as u16) * 16384)
                                as usize]), // Fix last bank to 0xc000
                            _ => return Err(RomError::UnknownAddress),
                        }
                    }
                }
            }
            a => Err(RomError::UnknownMapper(a))?,
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
        prg_bank: 1,
        chr_bank_0: 1,
        chr_bank_1: 2,
        shift_register: 16,
        chr_bank_mode: CharacterBankMode::Fullswitch,
        prg_bank_mode: ProgramBankMode::Fullswitch,
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
