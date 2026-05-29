mod pulse_channel;
mod duty_cycle;
mod envelope;
mod length_timer;
mod pulse_phase_timer;
mod pulse_channel_with_sweep;
mod sweep;
mod wave_channel;
mod wave_length_timer;
mod pulse_timer;
mod noise_channel;
mod noise_shape;

use crate::apu::pulse_channel::PulseChannel;
use crate::{AUDIO_SAMPLE_RATE, CPU_CLOCK_SPEED};
use crate::apu::noise_channel::NoiseChannel;
use crate::apu::pulse_channel_with_sweep::PulseChannelWithSweep;
use crate::apu::wave_channel::WaveChannel;

const FRAME_SEQUENCER_PERIOD: u32 = 8192;

#[derive(Default)]
pub struct APU {
    enabled: bool,
    master_volume: u8,
    sound_panning: u8,
    channel_1: PulseChannelWithSweep,
    channel_2: PulseChannel,
    channel_3: WaveChannel,
    channel_4: NoiseChannel,
    sound_buffer: Vec<(f32, f32)>,
    sample_counter: u32,
    frame_sequencer: u8,
    frame_sequencer_counter: u32,
    hp_capacitor: (f32, f32),
}

impl APU {
    pub fn cycle(&mut self, t_cycles: u32) {
        for _ in 0..t_cycles {
            self.channel_1.tick();
            self.channel_2.tick();
            self.channel_3.tick();
            self.channel_4.tick();
            self.sample_counter += AUDIO_SAMPLE_RATE;
            if self.sample_counter >= CPU_CLOCK_SPEED {
                self.sample_counter -= CPU_CLOCK_SPEED;
                self.sample();
            }

            self.frame_sequencer_counter += 1;
            if self.frame_sequencer_counter >= FRAME_SEQUENCER_PERIOD {
                self.frame_sequencer_counter = 0;
                self.tick_frame_sequencer();
            }
        }
    }
    fn sample(&mut self) {
        let analog_sample_1 = match self.channel_1.sample() {
            Some(digital_sample) => (digital_sample as f32 / 7.5) - 1.0,
            None => 0.0
        };
        let analog_sample_2 = match self.channel_2.sample() {
            Some(digital_sample) => (digital_sample as f32 / 7.5) - 1.0,
            None => 0.0
        };
        let analog_sample_3 = match self.channel_3.sample() {
            Some(digital_sample) => (digital_sample as f32 / 7.5) - 1.0,
            None => 0.0
        };
        let analog_sample_4 = match self.channel_4.sample() {
            Some(digital_sample) => (digital_sample as f32 / 7.5) - 1.0,
            None => 0.0
        };
        let stereo_pairs = self.pan((analog_sample_1, analog_sample_2, analog_sample_3, analog_sample_4));
        let mixed_stereo_pairs = self.mix(stereo_pairs);
        self.sound_buffer.push(mixed_stereo_pairs);
    }
    fn pan(&self, samples: (f32, f32, f32, f32)) -> (f32, f32) {
        let left_channel = (
            (self.sound_panning & 0b0001_0000 != 0) as u8 as f32 * samples.0 +
            (self.sound_panning & 0b0010_0000 != 0) as u8 as f32 * samples.1 +
            (self.sound_panning & 0b0100_0000 != 0) as u8 as f32 * samples.2 +
            (self.sound_panning & 0b1000_0000 != 0) as u8 as f32 * samples.3
        ) / 4.0;
        let right_channel = (
            (self.sound_panning & 0b0000_0001 != 0) as u8 as f32 * samples.0 +
            (self.sound_panning & 0b0000_0010 != 0) as u8 as f32 * samples.1 +
            (self.sound_panning & 0b0000_0100 != 0) as u8 as f32 * samples.2 +
            (self.sound_panning & 0b0000_1000 != 0) as u8 as f32 * samples.3
        ) / 4.0;

        (left_channel, right_channel)
    }
    fn mix(&mut self, channels: (f32, f32)) -> (f32, f32) {
        let left_volume = (1.0 + ((self.master_volume & 0b0111_0000) >> 4) as f32) / 8.0;
        let right_volume = (1.0 + (self.master_volume & 0b0000_0111) as f32) / 8.0;
        let left = channels.0 * left_volume;
        let right = channels.1 * right_volume;

        // Høypassfilter simulerer kondensatorkoblet analog output (DC-blokkering).
        // DMG-verdi 0.999958 er per T-syklus; skalert til 48 kHz: 0.999958^(4194304/48000)
        const HP_CHARGE_FACTOR: f32 = 0.99633;
        let out_left = left - self.hp_capacitor.0;
        let out_right = right - self.hp_capacitor.1;
        self.hp_capacitor.0 = left - out_left * HP_CHARGE_FACTOR;
        self.hp_capacitor.1 = right - out_right * HP_CHARGE_FACTOR;
        (out_left, out_right)
    }
    fn tick_frame_sequencer(&mut self) {
        self.frame_sequencer = (self.frame_sequencer + 1) % 8;
        match self.frame_sequencer {
            0 | 4 => self.tick_length_timer(),
            2 | 6 => { self.tick_length_timer(); self.tick_sweep(); },
            7 => self.tick_envelope(),
            _ => {}
        }
    }
    fn tick_length_timer(&mut self) {
        if self.channel_1.length_timer.tick() {
            self.channel_1.enabled = false;
        }
        if self.channel_2.length_timer.tick() {
            self.channel_2.enabled = false;
        }
        if self.channel_3.length_timer.tick() {
            self.channel_3.enabled = false;
        }
    }
    fn tick_envelope(&mut self) {
        self.channel_1.envelope.tick();
        self.channel_2.envelope.tick();
    }
    fn tick_sweep(&mut self) {
        let (new_frequency, disable) = self.channel_1.sweep.tick();
        if let Some(new_frequency) = new_frequency {
            self.channel_1.pulse_phase_timer.period = new_frequency
        }
        if disable { self.channel_1.enabled = false; }
    }
    pub fn read_sound_buffer(&mut self) -> Vec<(f32, f32)> {
        std::mem::take(&mut self.sound_buffer)
    }
    pub fn read_byte(&self, address: u8) -> u8 {
        match address {
            0x10..=0x14 => self.channel_1.read_byte(address),
            0x16..=0x19 => self.channel_2.read_byte(address),
            0x1a..=0x1e => self.channel_3.read_byte(address),
            0x20..=0x23 => self.channel_4.read_byte(address),
            0x24 => self.master_volume,
            0x25 => self.sound_panning,
            0x26 => self.audio_master_control(),
            _ => 0x00 // Other audio channels not implemented
        }
    }
    pub fn write_byte(&mut self, address: u8, value: u8) {
        if !self.enabled {
            if address == 0x26 && value & 0b1000_0000 != 0 {
                self.enabled = true;
                return;
            } else {
                return;
            }
        }
        match address {
            0x10..=0x14 => self.channel_1.write_byte(address, value),
            0x16..=0x19 => self.channel_2.write_byte(address, value),
            0x1a..=0x1e => self.channel_3.write_byte(address, value),
            0x20..=0x23 => self.channel_4.write_byte(address, value),
            0x24 => self.master_volume = value & 0b0111_0111,
            0x25 => self.sound_panning = value,
            0x26 => self.enabled = value & 0b1000_0000 != 0,
            _ => {} // Other audio channels not implemented
        }
    }
    pub fn read_wave_byte(&self, address: u8) -> u8 {
        self.channel_3.wave_ram[address as usize - 0x30]
    }
    pub fn write_wave_byte(&mut self, address: u8, value: u8) {
        self.channel_3.wave_ram[address as usize - 0x30] = value
    }
    fn audio_master_control(&self) -> u8 {
        0b1111_0000
        | (self.channel_3.enabled as u8) << 2
        | (self.channel_2.enabled as u8) << 1
        | self.channel_1.enabled as u8
    }
}
