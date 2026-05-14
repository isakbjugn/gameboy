
pub struct APU {
    
}

impl APU {
    pub fn new() -> Self {
        Self {}
    }
    pub fn cycle(&mut self, _t_cycles: u32) {

    }
    pub fn read_byte(&self, _address: u8) -> u8 {
        // Audio not implemented
        0x00
    }
    pub fn write_byte(&self, _address: u8, _value: u8) {
        // Audio not implemented
    }
    pub fn read_wave_byte(&self, _address: u8) -> u8 {
        // Wave pattern not implemented
        0xff
    }
    pub fn write_wave_byte(&self, _address: u8, _value: u8) {
        // Wave pattern not implemented
    }
}