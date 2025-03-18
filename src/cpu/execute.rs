use crate::cpu::CPU;
use crate::registers::{Reg16, Reg8};
use crate::registers::Reg16::HL;

pub enum Address {
    HL,
}

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
        let incremented_value = self.alu_inc(value);
        self.registers.write_8(reg, incremented_value);
    }
    pub fn dec(&mut self, reg: Reg8) {
        let value = self.registers.read_8(reg);
        let decremented_value = self.alu_dec(value);
        self.registers.write_8(reg, decremented_value);
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
    pub fn rra(&mut self) {
        let previous_carry = self.registers.f.carry;
        let (result, carry) = self.registers.a.overflowing_shr(1);
        self.registers.a = result | (if previous_carry { 0x80 } else { 0 });

        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry;
    }
    pub fn jr(&mut self) {
        let offset = self.fetch_byte() as i8;
        self.registers.pc = self.registers.pc.wrapping_add(offset as u16)
    }
    pub fn daa(&mut self) {
        let mut adjustment = 0;
        match self.registers.f.subtract {
            true => {
                if self.registers.f.half_carry { adjustment += 0x6 }
                if self.registers.f.carry { adjustment += 0x60 }
                self.registers.a = self.registers.a.wrapping_sub(adjustment);
            }
            false => {
                if self.registers.f.half_carry || self.registers.a & 0xf > 0x9 { adjustment += 0x6 }
                if self.registers.f.carry || self.registers.a > 0x99 { adjustment += 0x60; self.registers.f.carry = true }
                self.registers.a = self.registers.a.wrapping_add(adjustment);
            }
        }
        
        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.half_carry = false;
    }
    pub fn cpl(&mut self) {
        self.registers.a = !self.registers.a;
        
        self.registers.f.subtract = true;
        self.registers.f.half_carry = true;
    }
    pub fn ccf(&mut self) {
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = !self.registers.f.carry;
    }
    pub fn alu_inc(&mut self, value: u8) -> u8 {
        let incremented_value = value.wrapping_add(1);

        self.registers.f.zero = incremented_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = value & 0x0f == 0x0f;
        
        incremented_value
    }
    pub fn inc_addr(&mut self, addr: Address) {
        let address = match addr {
            Address::HL => self.registers.read_16(HL),
        };
        let incremented_value = self.alu_inc(self.bus.read_byte(address));
        self.bus.write_byte(address, incremented_value);
    }
    pub fn alu_dec(&mut self, value: u8) -> u8 {
        let decremented_value = value.wrapping_sub(1);

        self.registers.f.zero = decremented_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = value & 0x0f == 0;
        
        decremented_value
    }
    pub fn dec_addr(&mut self, addr: Address) {
        let address = match addr {
            Address::HL => self.registers.read_16(HL),
        };
        
        let decremented_value = self.alu_dec(self.bus.read_byte(address));
        self.bus.write_byte(address, decremented_value);
    }
}