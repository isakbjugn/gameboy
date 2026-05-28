use crate::apu::envelope::{Envelope, EnvelopeDirection};
use crate::apu::length_timer::LengthTimer;

#[derive(Default)]
pub struct NoiseChannel {
    enabled: bool,
    length_timer: LengthTimer,
    envelope: Envelope,
}

impl NoiseChannel {
    pub fn read_byte(&self, address: u8) -> u8 {
        match address {
            0x20 => panic!("FF20 er write-only"),
            0x21 => self.envelope.initial_volume << 4 | self.envelope.direction.as_bit() << 3 | self.envelope.sweep_pace,
            _ => 0x00
        }
    }
    pub fn write_byte(&mut self, address: u8, value: u8) {
        match address {
            0x20 => self.length_timer.load(value & 0b0011_1111),
            0x21 => {
                self.envelope.initial_volume = (value & 0b1111_0000) >> 4;
                self.envelope.direction = EnvelopeDirection::from_bit(value >> 3);
                if self.envelope.initial_volume == 0 && self.envelope.direction == EnvelopeDirection::Down {
                    self.enabled = false;
                }
                self.envelope.sweep_pace = value & 0b0000_0111;
            }
            _ => {}
        }
    }
}
