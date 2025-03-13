use std::ops::Index;

const BOOTROM_SIZE: usize = 0x100;

pub struct Bootrom {
    data: [u8; BOOTROM_SIZE],
    active: bool,
}

impl Bootrom {
    pub fn new() -> Self {
        Self {
            data: *include_bytes!("../dmg_boot.bin"),
            active: true
        }
    }
    pub fn is_active(&self) -> bool {
        self.active
    }
    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

impl Index<u16> for Bootrom {
    type Output = u8;
    fn index(&self, index: u16) -> &u8 {
        &self.data[index as usize]
    }
}