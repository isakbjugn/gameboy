use crate::apu::duty_cycle::DutyCycle;
use crate::apu::envelope::{Envelope, EnvelopeDirection};
use crate::apu::length_timer::LengthTimer;
use crate::apu::pulse_phase_timer::PulsePhaseTimer;
use crate::apu::sweep::{Sweep, SweepDirection};

#[derive(Default)]
pub struct PulseChannelWithSweep {
    pub enabled: bool,
    pulse_phase_timer: PulsePhaseTimer,
    duty_cycle: DutyCycle,
    pub envelope: Envelope,
    pub length_timer: LengthTimer,
    sweep: Sweep,
}

impl PulseChannelWithSweep {
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
    fn trigger(&mut self) {
        self.enabled = true;
        self.length_timer.trigger();
        self.pulse_phase_timer.trigger();
        self.envelope.trigger();
    }
    pub fn read_byte(&self, address: u8) -> u8 {
        match address {
            0x10 => {
                self.sweep.period << 4
                | self.sweep.direction.as_bit() << 3
                | self.sweep.shift_amount
            }
            0x11 => self.duty_cycle.to_bits() << 6,
            0x12 => self.envelope.initial_volume << 4 | self.envelope.direction.as_bit() << 3 | self.envelope.sweep_pace,
            0x13 => panic!("FF18 er write-only"),
            0x14 if self.length_timer.enabled => 0b0100_0000,
            _ => 0x00,
        }
    }
    pub fn write_byte(&mut self, address: u8, value: u8) {
        match address {
            0x10 => {
                self.sweep.period = (value & 0b0111_0000) >> 4;
                self.sweep.direction = SweepDirection::from_bit((value & 0b0000_1000) >> 3);
                self.sweep.shift_amount = value & 0b0000_0111;
            }
            0x11 => {
                self.duty_cycle = DutyCycle::from_bits(value >> 6);
                self.length_timer.load(value & 0b0011_1111);
            }
            0x12 => {
                self.envelope.initial_volume = (value & 0b1111_0000) >> 4;
                self.envelope.direction = EnvelopeDirection::from_bit(value >> 3);
                if self.envelope.initial_volume == 0 && self.envelope.direction == EnvelopeDirection::Down {
                    self.enabled = false;
                }
                self.envelope.sweep_pace = value & 0b0000_0111;
            }
            0x13 => {
                self.pulse_phase_timer.period = self.pulse_phase_timer.period & 0xff00 | value as u16;
            }
            0x14 => {
                if value & 0b1000_0000 != 0 { self.trigger() }
                self.length_timer.enabled = value & 0b0100_0000 != 0;
                self.pulse_phase_timer.period = (((value & 0b0000_0111) as u16) << 8) | (self.pulse_phase_timer.period & 0x00ff)
            }
            _ => {}
        }
    }
}
