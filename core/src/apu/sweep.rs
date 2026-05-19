#[derive(Clone, Copy, Default)]
pub enum SweepDirection {
    #[default]
    Up,
    Down,
}

impl SweepDirection {
    pub fn as_bit(self) -> u8 {
        match self {
            SweepDirection::Up => 0,
            SweepDirection::Down => 1,
        }
    }
    pub fn from_bit(bit: u8) -> Self {
        if bit & 0b01 == 0 { SweepDirection::Up } else { SweepDirection::Down }
    }
}

#[derive(Default)]
pub struct Sweep {
    pub period: u8,
    pub direction: SweepDirection,
    pub sweep_shift: u8,
    enabled: bool,
    shadow_frequency: u16,
    sweep_timer: u8,
}

impl Sweep {
    pub fn tick(&mut self) -> (Option<u16>, bool) {
        if self.sweep_timer > 0 {
            self.sweep_timer -= 1;
        }
        if self.sweep_timer == 0 {
            self.sweep_timer = match self.period {
                0 => 8,
                n => n,
            };
            if self.enabled && self.period > 0 {
                let (new_frequency, overflow) = self.calculate_frequency();
                if new_frequency <= 2047 && self.sweep_shift > 0 {
                    self.shadow_frequency = new_frequency;
                    let (_, possible_overflow) = self.calculate_frequency();
                    return (Some(new_frequency), overflow | possible_overflow)
                }
            }
        }
        (None, false)
    }
    fn calculate_frequency(&mut self) -> (u16, bool) {
        let frequency_diff = self.shadow_frequency >> self.sweep_shift;
        let new_frequency = match self.direction {
            SweepDirection::Up => self.shadow_frequency + frequency_diff,
            SweepDirection::Down => self.shadow_frequency - frequency_diff,
        };

        (new_frequency, new_frequency > 2047)
    }
    pub fn trigger(&mut self, current_frequency: u16) -> bool {
        self.shadow_frequency = current_frequency;
        self.sweep_timer = match self.period {
            0 => 8,
            n => n,
        };
        self.enabled = self.period != 0 || self.sweep_shift != 0;
        if self.sweep_shift != 0 {
            let (_, disable) = self.calculate_frequency();
            return disable
        }
        false
    }
}
