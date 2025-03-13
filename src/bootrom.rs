
pub struct Bootrom {
    active: bool,
}

impl Bootrom {
    pub fn new() -> Self {
        Self { active: true }
    }
    pub fn is_active(&self) -> bool {
        self.active
    }
}