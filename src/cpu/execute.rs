use crate::cpu::CPU;
use crate::registers::{Reg16, Reg8};

impl CPU {
    pub fn inc_16(&mut self, reg: Reg16) {
        let value = self.registers.read_16(reg).wrapping_add(1);
        self.registers.write_16(reg, value);
    }
    pub fn inc(&mut self, reg: Reg8) {
        let value = self.registers.read_8(reg);
        let incremented_value = value.wrapping_add(1);
        self.registers.f.zero = incremented_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = value & 0x0f == 0x0f;
        self.registers.write_8(reg, incremented_value);
    }
    pub fn dec(&mut self, reg: Reg8) {
        let value = self.registers.read_8(reg);
        let decremented_value = value.wrapping_sub(1);
        self.registers.f.zero = decremented_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = value & 0x0f == 0;
        self.registers.write_8(reg, decremented_value);
    }
    pub fn rlca(&mut self) {
        self.registers.f.carry = (self.registers.a & 0x80) != 0;
        self.registers.a = self.registers.a.rotate_left(1);
        
        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }
}