mod pulse_channel;

use log::info;
use crate::apu::pulse_channel::PulseChannel;

const CPU_CLOCK_SPEED: u32 = 4_194_304;
const SAMPLE_RATE: u32 = 48_000;

#[derive(Default)]
pub struct APU {
    enabled: bool,
    master_volume: u8,
    sound_panning: u8,
    channel_2: PulseChannel,
    sound_buffer: Vec<f32>,
    sample_counter: u32,
}

impl APU {
    pub fn cycle(&mut self, t_cycles: u32) {
        for _ in 0..t_cycles {
            self.channel_2.tick();
            self.sample_counter += SAMPLE_RATE;
            if self.sample_counter >= CPU_CLOCK_SPEED {
                self.sample_counter -= CPU_CLOCK_SPEED;
                let analog_sample = match self.channel_2.sample() {
                    Some(digital_sample) => (digital_sample as f32 / 7.5) - 1.0,
                    None => 0.0
                };
                self.sound_buffer.push(analog_sample);
            }
        }
    }
    pub fn read_sound_buffer(&mut self) -> Vec<f32> {
        std::mem::take(&mut self.sound_buffer)
    }
    pub fn read_byte(&self, address: u8) -> u8 {
        info!("Leser lyd-byte fra {:02x}", address);
        match address {
            0x16..=0x19 => self.channel_2.read_byte(address),
            0x24 => self.master_volume,
            0x25 => self.sound_panning,
            0x26 => self.audio_master_control(),
            _ => 0x00 // Other audio channels not implemented
        }
    }
    pub fn write_byte(&mut self, address: u8, value: u8) {
        if !self.enabled {
            if address == 0x26 && value & 0b1000_0000 != 0 {
                self.enabled = true;
                return;
            } else {
                return;
            }
        }
        info!("Skriver lyd-byte til {:02x}", address);
        match address {
            0x16..=0x19 => self.channel_2.write_byte(address, value),
            0x24 => self.master_volume = value & 0b0111_0111,
            0x25 => self.sound_panning = value,
            0x26 => self.enabled = value & 0b1000_0000 != 0,
            _ => {} // Other audio channels not implemented
        }
    }
    pub fn read_wave_byte(&self, _address: u8) -> u8 {
        // Wave pattern not implemented
        0xff
    }
    pub fn write_wave_byte(&self, _address: u8, _value: u8) {
        // Wave pattern not implemented
    }
    fn audio_master_control(&self) -> u8 {
        0b1111_0000
        | (self.channel_2.enabled as u8) << 1
    }
}
