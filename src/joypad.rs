
pub enum JoypadKey {
    Right,
    Left,
    Up,
    Down,
    A,
    B,
    Start,
    Select,
}

pub struct Joypad {
    data: u8,
}

impl Joypad {
    pub fn new() -> Self {
        Self { data: 0xFF }
    }
    pub fn read_byte(&self) -> u8 {
        self.data
    }
    pub fn write_byte(&self, value: u8) {
        todo!()
    }
}