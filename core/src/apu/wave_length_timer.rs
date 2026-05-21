#[derive(Default)]
pub struct WaveLengthTimer {
    pub enabled: bool,
    pub counter: u16,
}

impl WaveLengthTimer {
    pub fn tick(&mut self) -> bool {
        if !self.enabled || self.counter == 0 {
            return false
        }
        self.counter -= 1;
        self.counter == 0
    }
    pub fn trigger(&mut self) {
        if self.counter == 0 {
            self.counter = 256;
        }
    }
    pub fn load(&mut self, length_timer: u16) {
        self.counter = 256 - length_timer;
    }
}
