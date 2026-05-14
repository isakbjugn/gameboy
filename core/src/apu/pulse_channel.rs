#[derive(Default)]
pub struct PulseChannel {
    pub enabled: bool,
    pulse_phase_timer: PulsePhaseTimer,
    duty_cycle: DutyCycle,
    initial_length_timer: u8,
    envelope: Envelope,
    length_timer: LengthTimer,
}

impl PulseChannel {
    pub fn tick(&mut self) {
        self.pulse_phase_timer.tick();
    }
    pub fn sample(&self) -> Option<u8> {
        if !self.enabled {
            return None;
        }
        let phase = self.pulse_phase_timer.phase;
        let waveform_step = self.duty_cycle.waveform_step(phase);
        let volume = self.envelope.initial_volume;
        Some(waveform_step * volume)
    }
}

#[derive(Clone, Copy, Default)]
enum DutyCycle {
    #[default]
    Eight,
    Quarter,
    Half,
    ThreeQuarter,
}

impl DutyCycle {
    fn to_bits(self) -> u8 {
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
    fn waveform_step(self, phase: u8) -> u8 {
        let waveform = match self {
            DutyCycle::Eight => [0, 0, 0, 0, 0, 0, 0, 1],
            DutyCycle::Quarter => [1, 0, 0, 0, 0, 0, 0, 1],
            DutyCycle::Half => [1, 0, 0, 0, 0, 1, 1, 1],
            DutyCycle::ThreeQuarter => [0, 1, 1, 1, 1, 1, 1, 0],
        };
        waveform[phase as usize]
    }
}

#[derive(Clone, Copy, Default)]
enum EnvelopeDirection {
    #[default]
    Down,
    Up,
}

impl EnvelopeDirection {
    fn as_bit(self) -> u8 {
        match self {
            EnvelopeDirection::Down => 0,
            EnvelopeDirection::Up => 1,
        }
    }
    fn from_bit(bit: u8) -> Self {
        if bit & 0b01 == 0 { EnvelopeDirection::Down } else { EnvelopeDirection::Up }
    }
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
            0x17 => self.envelope.initial_volume << 4 | self.envelope.direction.as_bit() << 3 | self.envelope.sweep_pace,
            0x18 => panic!("FF18 er write-only"),
            0x19 if self.length_timer.enabled => 0b0100_0000,
            _ => 0x00,
        }
    }
    pub fn write_byte(&mut self, address: u8, value: u8) {
        match address {
            0x16 => {
                self.duty_cycle = DutyCycle::from_bits(value >> 6);
                self.initial_length_timer = value & 0b0011_1111;
            }
            0x17 => {
                self.envelope.initial_volume = (value & 0b1111_0000) >> 4;
                self.envelope.direction = EnvelopeDirection::from_bit(value >> 3);
                self.envelope.sweep_pace = value & 0b0000_0111;
            }
            0x18 => {
                self.pulse_phase_timer.period = self.pulse_phase_timer.period & 0xff00 | value as u16;
            }
            0x19 => {
                if value & 0b1000_0000 != 0 { self.enabled = true }
                self.length_timer.enabled = value & 0b0100_0000 != 0;
                self.pulse_phase_timer.period = (((value & 0b0000_0111) as u16) << 8) | (self.pulse_phase_timer.period & 0x00ff)
            }
            _ => {}
        }
    }
}

#[derive(Default)]
struct PulsePhaseTimer {
    period: u16,
    counter: u16,
    pub phase: u8,
}

impl PulsePhaseTimer {
    fn tick(&mut self) {
        if self.counter == 0 {
            self.counter = 4 * (2048 - self.period);
            self.phase = (self.phase + 1) % 8;
        }
        self.counter -= 1;
    }
}

#[derive(Default)]
struct LengthTimer {
    enabled: bool,
}
