
pub struct Timer {
    divider: u8,
    internal_divider: u32,
    timer: u8,
    internal_counter: u32,
    timer_modulo: u8,
    enable: bool,
    step: u32,
    pub interrupt: u8,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            divider: 0,
            internal_divider: 0,
            timer: 0,
            internal_counter: 0,
            timer_modulo: 0,
            enable: false,
            step: 4 * 256,
            interrupt: 0,
        }
    }
    pub fn cycle(&mut self, t_cycles: u32) {
        self.internal_divider += t_cycles;
        while self.internal_divider > 256 {
            self.divider = self.divider.wrapping_add(1);
            self.internal_divider -= 256;
        }
        
        if self.enable {
            self.internal_counter += t_cycles;
            while self.internal_counter > self.step {
                self.timer = self.timer.wrapping_add(1);
                if self.timer == 0 {
                    self.timer = self.timer_modulo;
                    self.interrupt |= 1 << 2;
                }
                self.internal_counter -= self.step;
            }
        }
    }
    pub fn read_byte(&self, address: u8) -> u8 {
        match address {
            0x04 => self.divider,
            0x05 => self.timer,
            0x06 => self.timer_modulo,
            0x07 => self.read_tac(),
            _ => unreachable!()
        }
    }
    fn read_tac(&self) -> u8 {
        (if self.enable { 0x4 } else { 0 })
            | (4 * match self.step { 256 => 0x0, 4 => 0x1, 16 => 0x2, 64 => 0x3, _ => unreachable!() })
    }
    pub fn write_byte(&mut self, address: u8, value: u8) {
        match address {
            0x04 => { self.divider = 0 },
            0x05 => { self.timer = value },
            0x06 => { self.timer_modulo = value },
            0x07 => self.write_tac(value),
            _ => unreachable!()
        }
    }
    fn write_tac(&mut self, value: u8) {
        self.enable = value & 0x4 != 0;
        self.step = 4 * match value & 0x3 { 0x0 => 256, 0x1 => 4, 0x2 => 16, 0x3 => 64, _ => unreachable!() }
    }
}