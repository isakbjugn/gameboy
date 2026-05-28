use crate::apu::length_timer::LengthTimer;

#[derive(Default)]
pub struct NoiseChannel {
    length_timer: LengthTimer,
}

impl NoiseChannel {
    pub fn read_byte(&self, address: u8) -> u8 {
        match address {
            0x20 => panic!("FF20 er write-only"),
            _ => 0x00
        }
    }
    pub fn write_byte(&mut self, address: u8, value: u8) {
        match address {
            0x20 => self.length_timer.load(value & 0b0011_1111),
            _ => {}
        }
    }
}
