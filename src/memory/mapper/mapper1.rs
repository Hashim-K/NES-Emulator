use crate::error::{MemoryError, RomError};
use crate::memory::mapper::Mapper;
use tudelft_nes_ppu::Mirroring;

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
pub struct Mapper1 {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    prg_bank: u8,
    chr_bank_0: u8,
    chr_bank_1: u8,
    shift_register: u8,
    prg_bank_mode: ProgramBankMode,
    chr_bank_mode: CharacterBankMode,
    pgr_ram: [u8; 8192], // 8 KiB of program ram
    chr_ram: [u8; 8192],
    init_code: Vec<u8>,
    mirroring: Mirroring,
}

impl Mapper1 {
    pub fn new(prg_rom: Vec<u8>, chr_rom: Vec<u8>, mirroring: Mirroring) -> Mapper1 {
        let init_code = prg_rom[prg_rom.len() - 257..].to_vec();
        Self {
            prg_rom,
            chr_rom,
            prg_bank: 0,
            chr_bank_0: 0,
            chr_bank_1: 0,
            shift_register: 16,
            prg_bank_mode: ProgramBankMode::Fixlast,
            chr_bank_mode: CharacterBankMode::Fullswitch,
            // pgr ram needs to mirror itself to fill 8kib
            pgr_ram: [0; 8192],
            chr_ram: [0; 8192],
            init_code,
            mirroring,
        }
        // TODO: implement error handling
    }
}

impl Mapper for Mapper1 {
    fn write(&mut self, address: u16, value: u8) -> Result<(), MemoryError> {
        if (value & 0b10000000) == 128 {
            self.mirroring = Mirroring::SingleScreenLower;
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
                        0 => self.mirroring = Mirroring::SingleScreenLower,
                        1 => self.mirroring = Mirroring::SingleScreenUpper,
                        2 => self.mirroring = Mirroring::Horizontal,
                        3 => self.mirroring = Mirroring::Vertical,
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

        Ok(())
    }

    fn read(&self, address: u16) -> Result<u8, RomError> {
        match self.prg_bank_mode {
            ProgramBankMode::Fullswitch => {
                let banknr = self.prg_bank & 0x0F;
                match address {
                    0x6000..0x8000 => Ok(self.pgr_ram[(address - 0x6000) as usize]), // PGR RAM
                    0x8000.. => {
                        let target: u32 = address as u32 - 0x8000 + (banknr as u32 * 0x8000);
                        Ok(self.prg_rom[target as usize])
                    } // switch in 32kb blocks
                    _ => Err(RomError::UnknownAddress),
                }
            }
            ProgramBankMode::Fixfirst => {
                match address {
                    0x6000..0x8000 => Ok(self.pgr_ram[(address - 0x6000) as usize]), // PGR RAM
                    0x8000..0xc000 => Ok(self.prg_rom[(address - 0x8000) as usize]), // fix first bank to 0x8000
                    0xc000.. => {
                        let target: u32 = address as u32 - 0xc000 + (self.prg_bank as u32) * 0x4000;
                        Ok(self.prg_rom[target as usize]) // make 0xc000 - 0x switchable
                    }
                    _ => Err(RomError::UnknownAddress),
                }
            }
            ProgramBankMode::Fixlast => {
                match address {
                    0x6000..0x8000 => Ok(self.pgr_ram[(address - 0x6000) as usize]), // PGR RAM
                    0x8000..0xc000 => {
                        Ok(self.prg_rom
                            [(address - 0x8000 + (self.prg_bank as u16) * 16384) as usize])
                    } // make 0x8000 - 0xc000 switchable
                    0xc000..0xff00 => {
                        let target: u32 =
                            address as u32 - 0xc000 + ((self.prg_rom.len() - 1) as u32) * 16384;
                        Ok(self.prg_rom[target as usize]) // Fix last bank to 0xc000
                    }
                    0xff00.. => Ok(self.init_code[(address - 0xff00) as usize]),
                    _ => Err(RomError::UnknownAddress),
                }
            }
        }
    }
}
