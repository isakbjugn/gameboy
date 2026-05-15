#[derive(Clone, Copy, Default, PartialEq)]
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
    pub volume: u8,
    pub direction: EnvelopeDirection,
    pub sweep_pace: u8,
    counter: u8,
}

impl Envelope {
    pub fn tick(&mut self) {
        if self.sweep_pace == 0 {
            return;
        }
        self.counter -= 1;
        if self.counter == 0 {
            self.counter = self.sweep_pace;
            match (self.direction, self.volume) {
                (EnvelopeDirection::Down, 0) | (EnvelopeDirection::Up, 15) => {},
                (EnvelopeDirection::Up, _) => self.volume += 1,
                (EnvelopeDirection::Down, _) => self.volume -= 1,
            }
        }
    }
    pub fn trigger(&mut self) {
        self.volume = self.initial_volume;
        self.counter = self.sweep_pace;
    }
}
