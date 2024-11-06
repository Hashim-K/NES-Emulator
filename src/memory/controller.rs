use tudelft_nes_ppu::{Buttons, Ppu};

#[derive(Debug)]
pub struct Controller {
    strobe: bool,     // The least significant bit that is written to the controller
    buttons: Buttons, // Holds the button state from when strobe was last high
    read_index: u8,   // Index of the button being read
}

impl Controller {
    // Writes to the controller input.
    //
    // Should be mapped to address $4016
    // Only the least significant bit of the input will be used.
    pub fn write(&mut self, byte: u8, ppu: &Ppu) {
        self.strobe = (byte & 0b1) == 1;
        self.clock_pulse(ppu);
    }

    // Refreshes the buttons when strobe is high. This should be called every clock cycle.
    pub fn clock_pulse(&mut self, ppu: &Ppu) {
        if self.strobe {
            self.buttons = ppu.get_joypad_state();
            self.read_index = 0;
        }
    }

    // Returns the current button's value
    //
    // Should be mapped to address $4016 for controller 1 and $4017 for controller 2
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
