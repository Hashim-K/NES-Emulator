use tudelft_nes_ppu::{Buttons, Ppu};

#[derive(Debug)]
pub struct Controller {
    strobe: bool,
    buttons: Buttons,
    read_index: u8,
}

impl Controller {
    pub fn write(&mut self, byte: u8) {
        self.strobe = (byte & 0b1) == 1;
    }

    pub fn clock_pulse(&mut self, ppu: &Ppu) {
        if self.strobe {
            self.buttons = ppu.get_joypad_state();
            self.read_index = 0;
        }
    }

    pub fn read(&mut self, ppu: &Ppu) -> u8 {
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

    pub fn new() -> Self {
        Controller {
            strobe: false,
            buttons: Buttons::default(),
            read_index: 0,
        }
    }
}
