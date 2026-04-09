use crate::cpu::CPU;

pub enum Condition {
    Zero,
    NotZero,
    Carry,
    NotCarry,
    True,
}

impl CPU {
    pub fn check_condition(&self, condition: Condition) -> bool {
        match condition {
            Condition::Zero => self.registers.f.zero,
            Condition::NotZero => !self.registers.f.zero,
            Condition::Carry => self.registers.f.carry,
            Condition::NotCarry => !self.registers.f.carry,
            Condition::True => true,
        }
    }
}