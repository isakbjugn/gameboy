use crate::apu::pulse_timer::PulseTimer;
use crate::apu::wave_length_timer::WaveLengthTimer;

#[derive(Default)]
pub struct WaveChannel {
    pub enabled: bool,
    pub dac_enabled: bool,
    pub length_timer: WaveLengthTimer,
    output_level: u8,
    pub pulse_timer: PulseTimer,
    pub wave_ram:  [u8; 16],
}

impl WaveChannel {
    pub fn tick(&mut self) {
        self.pulse_timer.tick();
    }
    pub fn sample(&self) -> Option<u8> {
        if !self.enabled {
            return None;
        }
        let wave_sample_index = self.pulse_timer.wave_sample_index;
        let upper_nibble = wave_sample_index % 2 == 0;
        let wave_byte_index = wave_sample_index / 2;
        let wave_byte = self.wave_ram[wave_byte_index as usize];
        let wave_sample = match upper_nibble {
            true => (wave_byte & 0xf0) >> 4,
            false => wave_byte & 0x0f
        };
        let bit_shift = match self.output_level {
            0 => 4,
            1 => 0,
            2 => 1,
            3 => 2,
            _ => panic!("Ugyldig volum i lydkanal 3")
        };
        let volume_adjusted_wave_sample = wave_sample >> bit_shift;
        Some(volume_adjusted_wave_sample)
    }
    fn trigger(&mut self) {
        if !self.dac_enabled { return; }
        self.enabled = true;
        self.length_timer.trigger();
        self.pulse_timer.trigger();
        self.pulse_timer.wave_sample_index = 0;
    }
    pub fn read_byte(&self, address: u8) -> u8 {
        match address {
            0x1a => (self.dac_enabled as u8) << 7,
            0x1b => panic!("FF1B er write-only"),
            0x1c => self.output_level << 5,
            0x1d => panic!("FF1D er write-only"),
            0x1e if self.length_timer.enabled => 0b0100_0000,
            _ => 0x00,
        }
    }
    pub fn write_byte(&mut self, address: u8, value: u8) {
        match address {
            0x1a => self.dac_enabled = value & 0b1000_0000 != 0,
            0x1b => self.length_timer.load(value as u16),
            0x1c => self.output_level = (value & 0b0110_0000) >> 5,
            0x1d => {
                self.pulse_timer.period = self.pulse_timer.period & 0xff00 | value as u16;
            }
            0x1e => {
                self.pulse_timer.period = (((value & 0b0000_0111) as u16) << 8) | (self.pulse_timer.period & 0x00ff);
                self.length_timer.enabled = value & 0b0100_0000 != 0;
                if value & 0b1000_0000 != 0 { self.trigger() }
            }
            _ => {}
        }
    }
}
