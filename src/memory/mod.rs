use crate::cpu::debug::{self, DebugMode};
use crate::cpu::Cpu;
use crate::error::{MemoryError, RomError};
use log::warn;
use mapper::get_mapper;
use mapper::Mapper;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use tudelft_nes_ppu::Buttons;
use tudelft_nes_ppu::{Mirroring, Ppu, PpuRegister};

mod mapper;

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
    cartridge: Arc<Mutex<Box<dyn Mapper + Send>>>,
    controller: RefCell<Controller>,
    debug: DebugMode,
}

impl Memory {
    pub fn new(rom_bytes: &[u8], debugmode: DebugMode) -> Result<Memory, RomError> {
        Ok(Memory {
            cartridge: Arc::new(Mutex::new(get_mapper(rom_bytes)?)),
            internal_ram: [0; 2048],
            controller: RefCell::new(Controller::new()),
            debug: debugmode,
        })
    }

    pub fn write_ppu_byte(&mut self, _address: u16, _value: u8) -> Result<(), MemoryError> {
        warn!("Writing ppu byte not yet iplemented");
        Ok(())
        // if self.cartridge..header.charactor_memory_size != 0 {
        //     if self.cartridge.chr_bank_mode == CharacterBankMode::Fullswitch {
        //         let banknr: u32 = self.cartridge.chr_bank_0 as u32 >> 1;
        //         let target: u32 = address as u32 + banknr * 0x2000;
        //         self.cartridge.chr_data[target as usize] = value;
        //     } else {
        //         match address {
        //             0x0000..0x1000 => {
        //                 let target: u32 =
        //                     address as u32 + self.cartridge.chr_bank_0 as u32 * 0x1000;
        //                 self.cartridge.chr_data[target as usize] = value;
        //             }
        //             0x1000..0x2000 => {
        //                 let target: u32 =
        //                     address as u32 + self.cartridge.chr_bank_0 as u32 * 0x1000;
        //                 self.cartridge.chr_data[target as usize] = value;
        //             }
        //             _ => return Err(MemoryError::UnknownAddress),
        //         }
        //     }
        // } else {
        //     if address > 0x2000 {
        //         self.debug
        //             .info_log(format!("address too large: {:4X}", address));
        //     }
        //     self.cartridge.chr_ram[address as usize] = value;
        // }
        // Ok(())
    }

    pub fn read_ppu_byte(&self, _address: u16) -> Result<u8, MemoryError> {
        // println!("Reading address {:x}", _address);
        // Ok(self.cartridge.lock().unwrap().read(_address)?)
        warn!("Reading ppu byte not yet iplemented address {:x}", _address);
        Ok(0)
        // if self.cartridge.header.charactor_memory_size != 0 {
        //     if self.cartridge.chr_bank_mode == CharacterBankMode::Fullswitch {
        //         let banknr: u32 = self.cartridge.chr_bank_0 as u32 >> 1;
        //         let target: u32 = address as u32 + banknr * 0x2000;
        //         Ok(self.cartridge.chr_data[target as usize])
        //     } else {
        //         match address {
        //             0x0000..0x1000 => {
        //                 let target: u32 =
        //                     address as u32 + self.cartridge.chr_bank_0 as u32 * 0x1000;
        //                 Ok(self.cartridge.chr_data[target as usize])
        //             }
        //             0x1000..0x2000 => {
        //                 let target: u32 =
        //                     address as u32 + self.cartridge.chr_bank_0 as u32 * 0x1000;
        //                 Ok(self.cartridge.chr_data[target as usize])
        //             }
        //             _ => Err(MemoryError::UnknownAddress),
        //         }
        //     }
        // } else {
        //     if address > 0x2000 {
        //         self.debug
        //             .info_log(format!("address too large: {:4X}", address));
        //     }
        //     Ok(self.cartridge.chr_ram[address as usize])
        // }
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
            0x4020.. => return self.cartridge.lock().unwrap().write(address, value), // Cartridge memory
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
        return value;
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
            0x4020.. => Ok(self.cartridge.lock().unwrap().read(address).unwrap()),
        }
    }
}

#[derive(Debug)]
struct Controller {
    strobe: bool,
    buttons: Buttons,
    read_index: u8,
}

impl Controller {
    fn write(&mut self, byte: u8) {
        self.strobe = (byte & 0b1) == 1;
    }

    fn clock_pulse(&mut self, ppu: &Ppu) {
        if self.strobe {
            self.buttons = ppu.get_joypad_state();
            self.read_index = 0;
        }
    }

    fn read(&mut self, ppu: &Ppu) -> u8 {
        let result = u8::from(match self.read_index {
            0 => self.buttons.a,
            1 => self.buttons.b,
            2 => self.buttons.select,
            3 => self.buttons.start,
            4 => self.buttons.up,
            5 => self.buttons.down,
            6 => self.buttons.left,
            7 => self.buttons.right,
            _ => panic!("Button reading out of bounds!"),
        });

        // Advance reading index
        self.read_index += 1;
        if self.read_index > 7 {
            self.read_index = 0
        }
        self.clock_pulse(ppu);
        result
    }

    fn new() -> Self {
        Controller {
            strobe: false,
            buttons: Buttons::default(),
            read_index: 0,
        }
    }
}
