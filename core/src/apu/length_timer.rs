pub struct LengthTimer {
    pub enabled: bool,
    pub counter: u8,
}

impl LengthTimer {
    pub fn tick(&mut self) -> bool {
        if self.enabled {
            self.counter -= 1;
            return self.counter == 0
        }
        false
    }
    pub fn trigger(&mut self) {
        if self.counter == 0 {
            self.counter = 64;
        }
    }
    pub fn load(&mut self, length_timer: u8) {
        self.counter = 64 - length_timer;
    }
}

impl Default for LengthTimer {
    fn default() -> Self {
        Self { enabled: false, counter: 64 }
    }
}
