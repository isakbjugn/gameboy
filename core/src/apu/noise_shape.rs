
#[derive(Default)]
pub struct NoiseShape {
    clock_shift: u8,
    lfsr_width: u8,
    short_mode: bool,
    clock_divider: u8,
    lfsr: u16,
}

impl NoiseShape {
    pub fn tick(&mut self) {
        let bit_fifteen = ((self.lfsr & 0x0002) >> 1) ^ (self.lfsr & 0x0001);
        self.lfsr = self.lfsr & 0x7fff | bit_fifteen << 15;
        if self.short_mode {
            self.lfsr = self.lfsr & 0xff7f | bit_fifteen << 7;
        }
        self.lfsr >>= 1;
    }
    pub fn read(&self) -> u8 {
        (self.clock_shift << 4)
        | (self.lfsr_width << 3)
        | self.clock_divider
    }
    pub fn write(&mut self, value: u8) {
        self.clock_shift = (value & 0b1111_0000) >> 4;
        self.lfsr_width = (value & 0b0000_1000) >> 3;
        self.short_mode = matches!(self.lfsr_width, 1);
        self.clock_divider = value & 0b0000_0111;
    }
    pub fn reset(&mut self) {
        self.lfsr = 0;
    }
}
