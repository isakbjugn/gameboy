use crate::cpu::CPU;
use crate::registers::{Reg16, Reg8};
use crate::registers::Reg16::HL;

impl CPU {
    pub fn inc_16(&mut self, reg: Reg16) {
        let value = self.registers.read_16(reg).wrapping_add(1);
        self.registers.write_16(reg, value);
    }
    pub fn dec_16(&mut self, reg: Reg16) {
        let value = self.registers.read_16(reg).wrapping_sub(1);
        self.registers.write_16(reg, value);
    }
    pub fn inc(&mut self, reg: Reg8) {
        let value = self.registers.read_8(reg);
        let incremented_value = value.wrapping_add(1);
        self.registers.write_8(reg, incremented_value);
        
        self.registers.f.zero = incremented_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = value & 0x0f == 0x0f;
    }
    pub fn dec(&mut self, reg: Reg8) {
        let value = self.registers.read_8(reg);
        let decremented_value = value.wrapping_sub(1);
        self.registers.write_8(reg, decremented_value);
        
        self.registers.f.zero = decremented_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = value & 0x0f == 0;
    }
    pub fn rlca(&mut self) {
        self.registers.f.carry = (self.registers.a & 0x80) != 0;
        self.registers.a = self.registers.a.rotate_left(1);
        
        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }
    pub fn rrca(&mut self) {
        self.registers.f.carry = (self.registers.a & 0x01) != 0;
        self.registers.a = self.registers.a.rotate_right(1);
        
        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }
    pub fn add_16(&mut self, reg: Reg16) {
        let a = self.registers.read_16(reg);
        let b = self.registers.read_16(HL);
        let (sum, carry) = a.overflowing_add(b);
        self.registers.write_16(HL, sum);
        
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (((a & 0xfff) + (b & 0xfff)) & 0x1000) == 0x1000;
        self.registers.f.carry = carry;
    }
    pub fn rla(&mut self) {
        let previous_carry = self.registers.f.carry;
        let (result, carry) = self.registers.a.overflowing_shl(1);
        self.registers.a = result | (if previous_carry { 1 } else { 0 });
        
        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry;
    }
}