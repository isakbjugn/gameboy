#[derive(Default)]
pub struct PulseTimer {
    pub period: u16,
    pub counter: u16,
    pub wave_sample_index: u8,
}

impl PulseTimer {
    pub fn tick(&mut self) {
        if self.counter == 0 {
            self.counter = 2 * (2048 - self.period);
            self.wave_sample_index = (self.wave_sample_index + 1) % 32;
        }
        self.counter -= 1;
    }
    pub fn trigger(&mut self) {
        self.counter = 2 * (2048 - self.period);
    }
}
