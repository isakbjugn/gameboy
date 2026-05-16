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
    pub shift_amount: u8,
    enabled: bool,
    shadow_frequency: u32,
    sweep_timer: u32,
}
