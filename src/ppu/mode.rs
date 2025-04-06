pub enum Mode {
    HorizontalBlank,
    VerticalBlank,
    OAMScan,
    Drawing,
}

impl Mode {
    pub fn bits(&self) -> u8 {
        match self {
            Mode::HorizontalBlank => 0,
            Mode::VerticalBlank => 1,
            Mode::OAMScan => 2,
            Mode::Drawing => 3,
        }
    }
}