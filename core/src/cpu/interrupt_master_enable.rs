
pub struct InterruptMasterEnable {
    value: bool,
    enable_counter: u8,
    disable_counter: u8,
}

impl InterruptMasterEnable {
    pub fn new() -> Self {
        Self {
            value: true,
            enable_counter: 0,
            disable_counter: 0,
        }
    }
    pub fn read(&mut self) -> bool {
        self.enable_counter = match self.enable_counter {
            2 => 1,
            1 => { self.value = true; 0 },
            _ => 0,
        };
        self.disable_counter = match self.disable_counter {
            2 => 1,
            1 => { self.value = false; 0 },
            _ => 0,
        };
        self.value
    }
    pub fn ei(&mut self) {
        self.enable_counter = 2;
    }
    pub fn di(&mut self) {
        self.disable_counter = 2;
    }
    pub fn reti(&mut self) {
        self.enable_counter = 1;
    }
}
