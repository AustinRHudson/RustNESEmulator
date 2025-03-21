use bitflags::bitflags;

bitflags!{
    pub struct JoypadButtons: u8 {
        const RIGHT    = 0b10000000;
        const LEFT     = 0b01000000;
        const DOWN     = 0b00100000;
        const UP       = 0b00010000;
        const START    = 0b00001000;
        const SELECT   = 0b00000100;
        const BUTTON_B = 0b00000010;
        const BUTTON_A = 0b00000001;
    }
}

pub struct Joypad {
    strobe: bool,
    button_index: u8,
    button_status: JoypadButtons
}

impl Joypad {
    pub fn new() -> Self{
        Joypad {
            strobe: false,
            button_index: 0,
            button_status: JoypadButtons::from_bits_truncate(0)
        }
    }

    pub fn write_joypad(&mut self, data: u8){
        self.strobe = data & 1 == 1;
        if self.strobe {
            self.button_index = 0
        }
    }

    pub fn read_joypad(&mut self) -> u8 {
        if self.button_index > 7 {
            return 1;
        }
        let response = (self.button_status.bits & (1 << self.button_index)) >> self.button_index;
        if !self.strobe && self.button_index <= 7 {
            self.button_index += 1;
        }
        response
    }

    pub fn set_button_pressed_status(&mut self, button: JoypadButtons, pressed: bool) {
        self.button_status.set(button, pressed);
    }
}