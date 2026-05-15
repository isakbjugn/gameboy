#[derive(Clone, Copy, Default)]
pub enum DutyCycle {
    #[default]
    Eight,
    Quarter,
    Half,
    ThreeQuarter,
}

impl DutyCycle {
    pub fn to_bits(self) -> u8 {
        match self {
            DutyCycle::Eight => 0b00,
            DutyCycle::Quarter => 0b01,
            DutyCycle::Half => 0b10,
            DutyCycle::ThreeQuarter => 0b11,
        }
    }
    pub fn from_bits(byte: u8) -> Self {
        match byte {
            0b00 => DutyCycle::Eight,
            0b01 => DutyCycle::Quarter,
            0b10 => DutyCycle::Half,
            0b11 => DutyCycle::ThreeQuarter,
            _ => unreachable!()
        }
    }
    pub fn waveform_step(self, phase: u8) -> u8 {
        let waveform = match self {
            DutyCycle::Eight => [0, 0, 0, 0, 0, 0, 0, 1],
            DutyCycle::Quarter => [1, 0, 0, 0, 0, 0, 0, 1],
            DutyCycle::Half => [1, 0, 0, 0, 0, 1, 1, 1],
            DutyCycle::ThreeQuarter => [0, 1, 1, 1, 1, 1, 1, 0],
        };
        waveform[phase as usize]
    }
}
