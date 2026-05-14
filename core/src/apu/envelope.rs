#[derive(Clone, Copy, Default)]
pub enum EnvelopeDirection {
    #[default]
    Down,
    Up,
}

impl EnvelopeDirection {
    pub fn as_bit(self) -> u8 {
        match self {
            EnvelopeDirection::Down => 0,
            EnvelopeDirection::Up => 1,
        }
    }
    pub fn from_bit(bit: u8) -> Self {
        if bit & 0b01 == 0 { EnvelopeDirection::Down } else { EnvelopeDirection::Up }
    }
}

#[derive(Default)]
pub struct Envelope {
    pub initial_volume: u8,
    pub direction: EnvelopeDirection,
    pub sweep_pace: u8,
}
