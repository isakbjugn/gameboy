use std::cmp::PartialEq;

#[derive(Default)]
pub struct PulseChannel {
    duty_cycle: DutyCycle,
    initial_length_timer: u8,
    envelope: Envelope,
}

#[derive(Default)]
enum DutyCycle {
    #[default]
    Eight,
    Quarter,
    Half,
    ThreeQuarter,
}

impl DutyCycle {
    fn to_bits(&self) -> u8 {
        match self {
            DutyCycle::Eight => 0b00,
            DutyCycle::Quarter => 0b01,
            DutyCycle::Half => 0b10,
            DutyCycle::ThreeQuarter => 0b11,
        }
    }

    fn from_bits(byte: u8) -> Self {
        match byte {
            0b00 => DutyCycle::Eight,
            0b01 => DutyCycle::Quarter,
            0b10 => DutyCycle::Half,
            0b11 => DutyCycle::ThreeQuarter,
            _ => unreachable!()
        }
    }
}

#[derive(Default, PartialEq)]
enum EnvelopeDirection {
    #[default]
    Up,
    Down,
}

#[derive(Default)]
struct Envelope {
    initial_volume: u8,
    direction: EnvelopeDirection,
    sweep_pace: u8,
}

impl PulseChannel {
    pub fn read_byte(&self, address: u8) -> u8 {
        match address {
            0x16 => self.duty_cycle.to_bits() << 6,
            0x17 => self.envelope.initial_volume << 4 | if self.envelope.direction == EnvelopeDirection::Up { 1 } else { 0 } << 3 | self.envelope.sweep_pace,
            _ => 0x00,
        }
    }
    pub fn write_byte(&mut self, address: u8, value: u8) {
        match address {
            0x16 => {
                self.duty_cycle = DutyCycle::from_bits(value >> 6);
                self.initial_length_timer = value & 0b00111111;
            }
            0x17 => {
                self.envelope.initial_volume = (value & 0b11110000) >> 4;
                self.envelope.direction = if value == 0b00001000 { EnvelopeDirection::Up } else { EnvelopeDirection::Down };
                self.envelope.sweep_pace = value & 0b00000111;
            }
            _ => {}
        }
    }
}