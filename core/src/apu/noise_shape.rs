
#[derive(Default)]
pub struct NoiseShape {
    clock_shift: u8,
    lfsr_width: u8,
    clock_divider: u8,
}

impl NoiseShape {
    pub fn read(&self) -> u8 {
        (self.clock_shift << 4)
        | (self.lfsr_width << 3)
        | self.clock_divider
    }
    pub fn write(&mut self, value: u8) {
        self.clock_shift = (value & 0b1111_0000) >> 4;
        self.lfsr_width = (value & 0b0000_1000) >> 3;
        self.clock_divider = value & 0b0000_0111;
    }
}
