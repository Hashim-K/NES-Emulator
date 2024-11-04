use crate::cpu::debug::DebugMode;
use crate::cpu::Cpu;
use crate::error::{MemoryError, RomError};
use controller::Controller;
use std::cell::RefCell;
use tudelft_nes_ppu::{Mirroring, Ppu, PpuRegister};

mod controller;

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
    controller: RefCell<Controller>,
    debug: DebugMode,
}

impl Memory {
    pub fn new(rom_bytes: &[u8], debugmode: DebugMode) -> Result<Memory, RomError> {
        Ok(Memory {
            cartridge: Cartridge::new(rom_bytes, debugmode.clone())?,
            internal_ram: [0; 2048],
            controller: RefCell::new(Controller::new()),
            debug: debugmode,
        })
    }

    pub fn write_ppu_byte(&mut self, address: u16, value: u8) -> Result<(), MemoryError> {
        if self.cartridge.header.charactor_memory_size != 0 {
            if self.cartridge.chr_bank_mode == CharacterBankMode::Fullswitch {
                let banknr: u32 = self.cartridge.chr_bank_0 as u32 >> 1;
                let target: u32 = address as u32 + banknr * 0x2000;
                self.cartridge.chr_data[target as usize] = value;
            } else {
                match address {
                    0x0000..0x1000 => {
                        let target: u32 =
                            address as u32 + self.cartridge.chr_bank_0 as u32 * 0x1000;
                        self.cartridge.chr_data[target as usize] = value;
                    }
                    0x1000..0x2000 => {
                        let target: u32 =
                            address as u32 + self.cartridge.chr_bank_0 as u32 * 0x1000;
                        self.cartridge.chr_data[target as usize] = value;
                    }
                    _ => return Err(MemoryError::UnknownAddress),
                }
            }
        } else {
            if address > 0x2000 {
                self.debug
                    .info_log(format!("address too large: {:4X}", address));
            }
            self.cartridge.chr_ram[address as usize] = value;
        }
        Ok(())
    }

    pub fn read_ppu_byte(&self, address: u16) -> Result<u8, MemoryError> {
        if self.cartridge.header.charactor_memory_size != 0 {
            if self.cartridge.chr_bank_mode == CharacterBankMode::Fullswitch {
                let banknr: u32 = self.cartridge.chr_bank_0 as u32 >> 1;
                let target: u32 = address as u32 + banknr * 0x2000;
                Ok(self.cartridge.chr_data[target as usize])
            } else {
                match address {
                    0x0000..0x1000 => {
                        let target: u32 =
                            address as u32 + self.cartridge.chr_bank_0 as u32 * 0x1000;
                        Ok(self.cartridge.chr_data[target as usize])
                    }
                    0x1000..0x2000 => {
                        let target: u32 =
                            address as u32 + self.cartridge.chr_bank_0 as u32 * 0x1000;
                        Ok(self.cartridge.chr_data[target as usize])
                    }
                    _ => Err(MemoryError::UnknownAddress),
                }
            }
        } else {
            if address > 0x2000 {
                self.debug
                    .info_log(format!("address too large: {:4X}", address));
            }
            Ok(self.cartridge.chr_ram[address as usize])
        }
    }

    pub fn write(&mut self, address: u16, value: u8, ppu: &mut Ppu) -> Result<(), MemoryError> {
        //self.debug.info_log(format!(
        //    "Writing value 0x{:02X} to address: 0x{:04X}",
        //    value, address
        //));
        match address {
            ..0x2000 => self.internal_ram[(address & 0x07ff) as usize] = value, // RAM reading, including mirroring
            0x2000..0x4000 => {
                let _register = address_to_ppu_register(address);
                ppu.write_ppu_register(_register, value)
            } // NES PPU registers
            0x4000..0x4016 => {} // TODO: NES APU and I/O registers
            0x4016 => self.controller.borrow_mut().write(value), // NES APU and I/O registers
            0x4017..0x4020 => {} // TODO: APU and I/O functionality that is normally disabled
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
            0x4016 => Ok(self.controller.borrow_mut().read(ppu)),
            _ => self.read_cpu_mem(address),
        };
        // Debug printing
        self.debug.info_log(format!(
            "Currently in prg bank: {:?}, with mode: {:?}",
            self.cartridge.prg_bank, self.cartridge.prg_bank_mode
        ));
        if value.is_ok() {
            let tmp = value.unwrap();
            self.debug.info_log(format!(
                "Read memory byte at address 0x{:04X}: 0x{:02X}",
                address, tmp
            ));
            return Ok(tmp);
        } else {
            self.debug.info_log(format!(
                "Read memory byte at address 0x{:04X}: FAILED",
                address
            ));
        }
        value
    }

    pub fn read_cpu_mem(&self, address: u16) -> Result<u8, MemoryError> {
        match address {
            // RAM reading, including mirroring
            ..0x2000 => Ok(self.internal_ram[(address & 0x07ff) as usize]),
            // NES PPU registers
            0x2000..0x4000 => self.read_ppu_byte(address),
            //panic!("You have to use the read function if you want to access the ppu memory")
            //}
            // Open bus, undefined behavior
            0x4000..0x4016 => Ok(0),
            0x4016 => {
                panic!("You have to use the read function if you want to access the controller")
            }
            0x4017 => {
                // TODO: impelement controller 2
                Ok(0)
            }
            // Open bus, undefined behavior
            0x4018..0x4020 => Ok(0),
            // Cartridge memory
            0x4020.. => Ok(self.cartridge.read(address)?),
        }
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
    chr_ram: [u8; 8192],
    init_code: Vec<u8>,
    debug: DebugMode,
}

impl Cartridge {
    fn parse_header(rom_bytes: &[u8], debug: &DebugMode) -> Result<RomHeader, RomError> {
        // Check rom signature
        if rom_bytes[0..4] != *(b"NES\x1a") {
            debug.info_log(format!("{:?}", b"NES\x1a"));
            debug.info_log(format!("{:?}", &rom_bytes[0..4]));
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

    fn new(rom_bytes: &[u8], debug: DebugMode) -> Result<Cartridge, RomError> {
        let header = Self::parse_header(rom_bytes, &debug)?;
        let mut total_length: u32 =
            header.charactor_memory_size as u32 * 8192 + header.program_rom_size as u32 * 16384;
        if header.trainer {
            total_length += 512
        }
        if rom_bytes[16..].len() != total_length as usize {
            return Err(RomError::IncorrectDataSize);
        }
        let prg_rom_start_index: usize = 16 + (header.trainer as usize) * 512_usize;
        let prg_rom_end_index: usize =
            16 + (header.trainer as usize) * 512 + (header.program_rom_size as usize) * 0x4000;
        let mut cartridge_prg_rom: Vec<u8> =
            rom_bytes[prg_rom_start_index..prg_rom_end_index].to_vec();
        let mut cartridge_chr_rom: Vec<u8> = vec![];
        if header.charactor_memory_size != 0 {
            cartridge_chr_rom
                .append(&mut rom_bytes[prg_rom_end_index..(rom_bytes.len() - 272)].to_vec());
        } else {
            let chr_ram: [u8; 8192] = [0; 8192];
            cartridge_chr_rom.append(&mut chr_ram.to_vec());
        }
        let cartridge_init_code: Vec<u8> = rom_bytes[(prg_rom_end_index - 256)..].to_vec();
        if header.charactor_memory_size == 1 {
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
            prg_bank: 0,
            chr_bank_0: 0,
            chr_bank_1: 0,
            shift_register: 16,
            prg_bank_mode: ProgramBankMode::Fixlast,
            chr_bank_mode: CharacterBankMode::Fullswitch,
            // pgr ram needs to mirror itself to fill 8kib
            pgr_ram: [0; 8192],
            chr_ram: [0; 8192],
            init_code: cartridge_init_code,
            debug,
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
                if (value & 0b10000000) == 128 {
                    self.header.mirroring = Mirroring::SingleScreenLower;
                    self.prg_bank_mode = ProgramBankMode::Fixlast;
                    self.chr_bank_mode = CharacterBankMode::Fullswitch;
                } else if (self.shift_register & 1) != 1 {
                    self.shift_register = (self.shift_register >> 1) | ((value & 1) << 4);
                } else {
                    self.shift_register = (self.shift_register >> 1) | ((value & 1) << 4);
                    match address {
                        0x8000..0xa000 => {
                            //self.debug.info_log(format!("editing control register to {:08b}", self.shift_register));
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
                                //self.debug.info_log(format!("changed chr bank mode to fullswitch"));
                                self.chr_bank_mode = CharacterBankMode::Fullswitch
                            } else {
                                //self.debug.info_log(format!("changed chr bank mode to halfswitch"));
                                self.chr_bank_mode = CharacterBankMode::Halfswitch
                            }
                        }
                        0xa000..0xc000 => {
                            //self.debug.info_log(format!("editing chr0 register to {:08b}", self.shift_register));
                            self.chr_bank_0 = self.shift_register;
                        }
                        0xc000..0xe000 => {
                            //self.debug.info_log(format!("editing chr1 register to {:08b}", self.shift_register));
                            self.chr_bank_1 = self.shift_register;
                        }
                        0xe000.. => {
                            //self.debug.info_log(format!("editing prg register to {:08b}", self.shift_register));
                            self.prg_bank = self.shift_register;
                        }
                        _ => return Err(MemoryError::MapperAddressError(address)),
                    }
                    self.shift_register = 16;
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
                    0xc000..0xff00 => Ok(self.prg_data[(address - 0xc000 + 0x4000) as usize]), // last 16 KiB of prg rom
                    0xff00.. => Ok(self.init_code[(address - 0xff00) as usize]),
                    _ => Err(RomError::UnknownAddress),
                }
            }
            1 => {
                match self.prg_bank_mode {
                    ProgramBankMode::Fullswitch => {
                        let banknr = self.prg_bank & 0x0F;
                        match address {
                            0x6000..0x8000 => Ok(self.pgr_ram[(address - 0x6000) as usize]), // PGR RAM
                            0x8000.. => {
                                let target: u32 =
                                    address as u32 - 0x8000 + (banknr as u32 * 0x8000);
                                Ok(self.prg_data[target as usize])
                            } // switch in 32kb blocks
                            _ => Err(RomError::UnknownAddress),
                        }
                    }
                    ProgramBankMode::Fixfirst => {
                        match address {
                            0x6000..0x8000 => Ok(self.pgr_ram[(address - 0x6000) as usize]), // PGR RAM
                            0x8000..0xc000 => Ok(self.prg_data[(address - 0x8000) as usize]), // fix first bank to 0x8000
                            0xc000.. => {
                                let target: u32 =
                                    address as u32 - 0xc000 + (self.prg_bank as u32) * 0x4000;
                                Ok(self.prg_data[target as usize]) // make 0xc000 - 0x switchable
                            }
                            _ => Err(RomError::UnknownAddress),
                        }
                    }
                    ProgramBankMode::Fixlast => {
                        match address {
                            0x6000..0x8000 => Ok(self.pgr_ram[(address - 0x6000) as usize]), // PGR RAM
                            0x8000..0xc000 => Ok(self.prg_data
                                [(address - 0x8000 + (self.prg_bank as u16) * 16384) as usize]), // make 0x8000 - 0xc000 switchable
                            0xc000..0xff00 => {
                                let target: u32 = address as u32 - 0xc000
                                    + ((self.header.program_rom_size - 1) as u32) * 16384;
                                Ok(self.prg_data[target as usize]) // Fix last bank to 0xc000
                            }
                            0xff00.. => Ok(self.init_code[(address - 0xff00) as usize]),
                            _ => Err(RomError::UnknownAddress),
                        }
                    }
                }
            }
            a => Err(RomError::UnknownMapper(a))?,
        }
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
    assert_eq!(
        Cartridge::parse_header(ROM_NROM_TEST, &DebugMode::No).unwrap(),
        expected_header
    );
}
