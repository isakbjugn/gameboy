use crate::cpu::CPU;
use crate::registers::Reg16;

impl CPU {
    pub fn inc_16(&mut self, reg: Reg16) {
        let value = self.registers.read_16(reg).wrapping_add(1);
        self.registers.write_16(reg, value);
    }
}