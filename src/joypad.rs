
pub enum JoypadKey {
    A,
    B,
    Select,
    Start,
    Right,
    Left,
    Up,
    Down,
}

pub struct Joypad {
    data: u8,
    action_row: u8,
    d_pad_row: u8,
    pub interrupt: u8,
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            data: 0xff,
            action_row: 0x0f,
            d_pad_row: 0x0f,
            interrupt: 0,
        }
    }
    pub fn read_byte(&self) -> u8 {
        self.data
    }
    pub fn write_byte(&mut self, value: u8) {
        self.data = (self.data & 0xcf) | (value & 0x30);
        self.update()
    }
    pub fn key_down(&mut self, key: JoypadKey) {
        match key {
            JoypadKey::A      => self.action_row &= !0x1,
            JoypadKey::B      => self.action_row &= !0x2,
            JoypadKey::Select => self.action_row &= !0x3,
            JoypadKey::Start  => self.action_row &= !0x4,
            JoypadKey::Right  => self.d_pad_row  &= !0x1,
            JoypadKey::Left   => self.d_pad_row  &= !0x2,
            JoypadKey::Up     => self.d_pad_row  &= !0x3,
            JoypadKey::Down   => self.d_pad_row  &= !0x4,
        }
        self.interrupt |= 1 << 4;
        self.update()
    }
    pub fn key_up(&mut self, key: JoypadKey) {
        match key {
            JoypadKey::A      => self.action_row |= 0x1,
            JoypadKey::B      => self.action_row |= 0x2,
            JoypadKey::Select => self.action_row |= 0x3,
            JoypadKey::Start  => self.action_row |= 0x4,
            JoypadKey::Right  => self.d_pad_row  |= 0x1,
            JoypadKey::Left   => self.d_pad_row  |= 0x2,
            JoypadKey::Up     => self.d_pad_row  |= 0x3,
            JoypadKey::Down   => self.d_pad_row  |= 0x4,
        }
        self.update()
    }
    fn update(&mut self) {
        self.data &= 0xf0;
        if self.data & 0x10 == 0 {
            self.data &= self.action_row & 0x0f;
        }
        if self.data & 0x20 == 0 {
            self.data &= self.d_pad_row & 0x0f;
        }
    }
}