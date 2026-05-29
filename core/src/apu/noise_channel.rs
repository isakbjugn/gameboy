use crate::apu::envelope::{Envelope, EnvelopeDirection};
use crate::apu::length_timer::LengthTimer;
use crate::apu::noise_shape::NoiseShape;

#[derive(Default)]
pub struct NoiseChannel {
    pub enabled: bool,
    pub length_timer: LengthTimer,
    pub envelope: Envelope,
    noise_shape: NoiseShape,
}

impl NoiseChannel {
    pub fn tick(&mut self) {
        self.noise_shape.tick();
    }
    fn trigger(&mut self) {
        self.enabled = true;
        self.length_timer.trigger();
        self.envelope.trigger();
        self.noise_shape.reset();
    }
    pub fn sample(&self) -> Option<u8> {
        if !self.enabled {
            return None;
        }
        match self.noise_shape.bit_0() {
            false => Some(0),
            true => Some(self.envelope.volume)
        }
    }
    pub fn read_byte(&self, address: u8) -> u8 {
        match address {
            0x20 => panic!("FF20 er write-only"),
            0x21 => self.envelope.initial_volume << 4 | self.envelope.direction.as_bit() << 3 | self.envelope.sweep_pace,
            0x22 => self.noise_shape.read(),
            0x23 => (self.length_timer.enabled as u8) << 6,
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
            0x22 => self.noise_shape.write(value),
            0x23 => {
                self.length_timer.enabled = value & 0b0100_0000 != 0;
                if value & 0b1000_0000 != 0 { self.trigger(); }
            }
            _ => {}
        }
    }
}
