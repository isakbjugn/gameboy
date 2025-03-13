
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
}