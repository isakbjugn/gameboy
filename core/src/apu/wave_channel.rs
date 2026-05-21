use crate::apu::wave_length_timer::WaveLengthTimer;

#[derive(Default)]
pub struct WaveChannel {
    pub enabled: bool,
    pub length_timer: WaveLengthTimer,
    output_level: u8,
    period: u16,
}
impl WaveChannel {
    pub fn tick(&mut self) {

    }
    pub fn sample(&self) -> Option<u8> {
        if !self.enabled {
            return None;
        }
        None
    }
    fn trigger(&mut self) {
        self.enabled = true;
        self.length_timer.trigger();
    }
    pub fn read_byte(&self, address: u8) -> u8 {
        match address {
            0x1a => (self.enabled as u8) << 7,
            0x1b => panic!("FF1B er write-only"),
            0x1c => self.output_level << 5,
            0x1d => panic!("FF1D er write-only"),
            0x19 if self.length_timer.enabled => 0b0100_0000,
            _ => 0x00,
        }
    }
    pub fn write_byte(&mut self, address: u8, value: u8) {
        match address {
            0x1a => self.enabled = value & 0b1000_0000 != 0,
            0x1b => self.length_timer.load(value as u16),
            0x1c => self.output_level = (value & 0b0110_0000) >> 5,
            0x1d => {
                self.period = self.period & 0xff00 | value as u16;
            }
            0x1e => {
                if value & 0b1000_0000 != 0 { self.trigger() }
                self.length_timer.enabled = value & 0b0100_0000 != 0;
                self.period = (((value & 0b0000_0111) as u16) << 8) | (self.period & 0x00ff)
            }
            _ => {}
        }
    }
}
