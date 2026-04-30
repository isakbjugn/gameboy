
pub struct APU {
    audio_master_control: u8,
}

impl APU {
    pub fn new() -> Self {
        Self {
            audio_master_control: 0,
        }
    }
    pub fn cycle(&mut self, _t_cycles: u32) {

    }
    pub fn read_byte(&self, address: u8) -> u8 {
        match address {
            0x26 => self.audio_master_control,
            _ => 0x00 // Other audio channels not implemented
        }
    }
    pub fn write_byte(&mut self, address: u8, value: u8) {
        match address {
            0x26 => self.audio_master_control &= (value | 0b1000000),
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