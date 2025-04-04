use crate::cpu::CPU;
use crate::cpu::read_write::Operand;
use crate::cpu::read_write::Operand::RegA;
use crate::cpu::registers::{Reg16, Reg8};
use crate::cpu::registers::Reg16::HL;

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
    pub fn rl(&mut self, operand: Operand) {
        let value = self.read(operand);
        let previous_carry = self.registers.f.carry;
        let (result, carry) = value.overflowing_shl(1);
        let result = result | (if previous_carry { 1 } else { 0 });
        self.write(operand, result);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry;
    }
    pub fn rlca(&mut self) {
        self.rlc(RegA);
        self.registers.f.subtract = false;
    }
    pub fn rlc(&mut self, operand: Operand) -> u8 {
        let value = self.read(operand);
        self.registers.f.carry = (value >> 7) != 0;
        let result = value.rotate_left(1);
        self.write(operand, result);

        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;

        result
    }
    pub fn rrca(&mut self) {
        self.rrc(RegA);
        self.registers.f.zero = false;
    }
    pub fn rrc(&mut self, operand: Operand) {
        let value = self.read(operand);
        self.registers.f.carry = (value & 0x01) != 0;
        let result = value.rotate_right(1);
        self.write(operand, result);
        
        self.registers.f.zero = result == 0;
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
        self.rl(RegA);
        self.registers.f.zero = false;
    }
    pub fn rra(&mut self) {
        self.rr(RegA);
        self.registers.f.zero = false;
    }
    pub fn rr(&mut self, operand: Operand) {
        let value = self.read(operand);
        let previous_carry = self.registers.f.carry;
        let carry = value & 0x01 == 0x01;
        let result = (value >> 1) | if previous_carry { 1 << 7 } else { 0 };
        self.write(operand, result);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry;
    }
    pub fn jr(&mut self) {
        let offset = self.fetch_byte() as i8;
        self.registers.pc = (self.registers.pc as u32 as i32).wrapping_add(offset as i32) as u16;
    }
    pub fn jp(&mut self) {
        self.registers.pc = self.fetch_word()
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
    pub fn load(&mut self, destination: Operand, source: Operand) {
        let value = self.read(source);
        self.write(destination, value);
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
    pub fn alu_xor(&mut self, value: u8) {
        self.registers.a ^= value;
        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
    }
    pub fn bit(&mut self, operand: Operand, bit: u8) {
        let value = self.read(operand);
        self.registers.f.zero = value & (1 << bit) != 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
    }
    pub fn alu_or(&mut self, reg8: Reg8) {
        self.alu_or_val(self.registers.read_8(reg8))
    }
    pub fn alu_or_val(&mut self, value: u8) {
        self.registers.a |= value;

        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
    }
    pub fn alu_and(&mut self, value: u8) {
        self.registers.a &= value;

        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        self.registers.f.carry = false;
    }
    pub fn alu_cp(&mut self, value: u8) {
        let (result, overflow) = self.registers.a.overflowing_sub(value);

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0x0f) < (value & 0x0f);
        self.registers.f.carry = overflow;
    }
    pub fn alu_adc(&mut self, operand: Operand) {
        let value = self.read(operand);
        let carry = if self.registers.f.carry { 1 } else { 0 };
        let (sum, overflow_first) = self.registers.a.overflowing_add(value);
        let (sum, overflow_second) = sum.overflowing_add(carry);

        self.registers.f.zero = sum == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (((self.registers.a & 0x0f) + (value & 0x0f) + carry) & 0x10) == 0x10;
        self.registers.f.carry = overflow_first | overflow_second;
        
        self.registers.a = sum;
    }
    pub fn alu_add(&mut self, b: u8) {
        let a = self.registers.a;
        let (sum, carry) = a.overflowing_add(b);

        self.registers.f.zero = sum == 0;
        self.registers.f.half_carry = (a & 0xF) + (b & 0xF) > 0xF;
        self.registers.f.subtract = false;
        self.registers.f.carry = carry;

        self.registers.a = sum;
    }
    pub fn alu_sub(&mut self, b: u8) {
        let a = self.registers.a;
        let r = a.wrapping_sub(b);

        self.registers.f.zero = r == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (a & 0x0F) < (b & 0x0F);
        self.registers.f.carry = (a as u16) < (b as u16);

        self.registers.a = r;
    }
    pub fn alu_sbc(&mut self, operand: Operand) {
        let value = self.read(operand);
        let carry = if self.registers.f.carry { 1 } else { 0 };
        let (difference, overflow_first) = self.registers.a.overflowing_sub(value);
        let (difference, overflow_second) = difference.overflowing_sub(carry);

        self.registers.f.zero = difference == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0x0f) < (value & 0x0f) + carry;
        self.registers.f.carry = overflow_first | overflow_second;
        
        self.registers.a = difference;
    }
    pub fn srl(&mut self, operand: Operand) {
        self.sra(operand);
        let value = self.read(operand);
        self.write(operand, value & 0x7f);
    }
    pub fn swap(&mut self, operand: Operand) {
        let value = self.read(operand);
        let lower = value & 0x0f;
        let swapped_value = (lower << 4) | (value >> 4);
        self.write(operand, swapped_value);

        self.registers.f.zero = swapped_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
    }
    pub fn alu_add_s8(&mut self, value: u16) -> u16 {
        let byte = self.fetch_byte() as i8 as i32;
        let value_i32 = value as u32 as i32;
        let (sum, carry) = value_i32.overflowing_add(byte);
        
        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = ((value_i32 & 0x000f) + (byte & 0x000f)) & 0x0010 == 0x0010;
        self.registers.f.carry = ((value_i32 & 0x00ff) + (byte & 0x00ff)) & 0x0100 == 0x0100;
        
        sum as u16
    }
    pub fn sla(&mut self, operand: Operand) {
        let value = self.read(operand);
        let carry = value >> 7 == 1;
        let result = value << 1;
        self.write(operand, result);
        
        self.registers.f.carry = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry;
    }
    pub fn sra(&mut self, operand: Operand) {
        let value = self.read(operand);
        let carry = value & 0x01 == 1;
        let result = (value & 0x80) | (value >> 1);
        self.write(operand, result);
        
        self.registers.f.carry = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry;
    }
    pub fn res(&mut self, operand: Operand, bit: u8) {
        let value = self.read(operand);
        let result = value & !(1 << bit);
        self.write(operand, result);
    }
    pub fn set(&mut self, operand: Operand, bit: u8) {
        let value = self.read(operand);
        let result = value | (1 << bit);
        self.write(operand, result);
    }
}