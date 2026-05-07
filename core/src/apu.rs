mod pulse_channel;

use log::info;
use crate::apu::pulse_channel::PulseChannel;

pub struct APU {
    master_volume: u8,
    sound_panning: u8,
    audio_master_control: u8,
    channel_2: PulseChannel,
}

impl APU {
    pub fn new() -> Self {
        Self {
            master_volume: 0,
            sound_panning: 0,
            audio_master_control: 0,
            channel_2: PulseChannel {}
        }
    }
    pub fn cycle(&mut self, _t_cycles: u32) {

    }
    pub fn read_byte(&self, address: u8) -> u8 {
        info!("Leser lyd-byte fra {:02x}", address);
        match address {
            0x16..=0x19 => self.channel_2.read_byte(address),
            0x24 => self.master_volume,
            0x25 => self.sound_panning,
            0x26 => self.audio_master_control,
            _ => 0x00 // Other audio channels not implemented
        }
    }
    pub fn write_byte(&mut self, address: u8, value: u8) {
        info!("Skriver lyd-byte til {:02x}", address);
        match address {
            0x16..=0x19 => self.channel_2.write_byte(address, value),
            0x24 => self.master_volume = value & 0b01110111,
            0x25 => self.audio_master_control = value,
            0x26 => self.audio_master_control = value & 0b1000000,
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
}
