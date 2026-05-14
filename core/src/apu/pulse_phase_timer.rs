#[derive(Default)]
pub struct PulsePhaseTimer {
    pub period: u16,
    pub counter: u16,
    pub phase: u8,
}

impl PulsePhaseTimer {
    pub fn tick(&mut self) {
        if self.counter == 0 {
            self.counter = 4 * (2048 - self.period);
            self.phase = (self.phase + 1) % 8;
        }
        self.counter -= 1;
    }
}
