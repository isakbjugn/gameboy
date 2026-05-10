mod pulse_channel;

use log::info;
use crate::apu::pulse_channel::PulseChannel;

#[derive(Default)]
pub struct APU {
    enabled: bool,
    master_volume: u8,
    sound_panning: u8,
    channel_2: PulseChannel,
}

impl APU {
    pub fn cycle(&mut self, _t_cycles: u32) {

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
        info!("Skriver lyd-byte til {:02x}", address);
        match address {
            0x16..=0x19 => self.channel_2.write_byte(address, value),
            0x24 => self.master_volume = value & 0b0111_0111,
            0x25 => self.sound_panning = value,
            0x26 => self.enabled = value & 0b100_0000 != 0,
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
        | (if self.channel_2.enabled { 1 } else { 0 }) << 1
    }
}
