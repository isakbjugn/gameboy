use log::debug;
use crate::cpu::CPU;
use crate::cpu::read_write::Operand::{RegA, RegB, RegC, RegD, RegE, RegH, RegL, AddressBC, AddressDE, AddressHL, AddressHLI, AddressHLD, Immediate8};
use crate::cpu::registers::Reg8::{A, B, C, D, E, H, L};
use crate::cpu::registers::Reg16::{AF, BC, DE, HL, SP};

impl CPU {
    pub fn call(&mut self) -> u32 {
        let opcode = self.fetch_byte();
        debug!("Dekoder nå opkode {:#04x}", opcode);
        match opcode {
            0x00 => { 1 }
            0x01 => { let word = self.fetch_word(); self.registers.write_16(BC, word); 3 }
            0x02 => { self.load(AddressBC, RegA); 2 }
            0x03 => { self.inc_16(BC); 2 }
            0x04 => { self.alu_inc(RegB); 1 }
            0x05 => { self.alu_dec(RegB); 1 }
            0x06 => { self.load(RegB, Immediate8); 2 }
            0x07 => { self.rlca(); 1 }
            0x08 => { let address = self.fetch_word(); self.bus.write_word(address, self.registers.sp); 5 }
            0x09 => { self.add_16(BC); 2 }
            0x0a => { self.load(RegA, AddressBC); 2 }
            0x0b => { self.dec_16(BC); 2 }
            0x0c => { self.alu_inc(RegC); 1 }
            0x0d => { self.alu_dec(RegC); 1 }
            0x0e => { self.load(RegC, Immediate8); 2 }
            0x0f => { self.rrca(); 1 }
            0x10 => { panic!("STOP") }
            0x11 => { let word = self.fetch_word(); self.registers.write_16(DE, word); 3 }
            0x12 => { self.load(AddressDE, RegA); 2 }
            0x13 => { self.inc_16(DE); 2 }
            0x14 => { self.alu_inc(RegD); 1 }
            0x15 => { self.alu_dec(RegD); 1 }
            0x16 => { self.load(RegD, Immediate8); 2 }
            0x17 => { self.rla(); 1 }
            0x18 => { self.jr(); 3 }
            0x19 => { self.add_16(DE); 2 }
            0x1a => { self.load(RegA, AddressDE); 2 }
            0x1b => { self.dec_16(DE); 2 }
            0x1c => { self.alu_inc(RegE); 1 }
            0x1d => { self.alu_dec(RegE); 1 }
            0x1e => { self.load(RegE, Immediate8); 2 }
            0x1f => { self.rra(); 1 }
            0x20 => { if !self.registers.f.zero { self.jr(); 3 } else { self.registers.pc += 1; 2 } }
            0x21 => { let word = self.fetch_word(); self.registers.write_16(HL, word); 3 }
            0x22 => { self.load(AddressHLI, RegA); 2 }
            0x23 => { self.inc_16(HL); 2 }
            0x24 => { self.alu_inc(RegH); 1 }
            0x25 => { self.alu_dec(RegH); 1 }
            0x26 => { self.load(RegH, Immediate8); 2 }
            0x27 => { self.daa(); 1 }
            0x28 => { if self.registers.f.zero { self.jr(); 3 } else { self.registers.pc += 1; 2 } }
            0x29 => { self.add_16(HL); 2 }
            0x2a => { self.load(RegA, AddressHLI); 2 }
            0x2b => { self.dec_16(HL); 2 }
            0x2c => { self.alu_inc(RegL); 1 }
            0x2d => { self.alu_dec(RegL); 1 }
            0x2e => { self.load(RegL, Immediate8); 2 }
            0x2f => { self.cpl(); 1 }
            0x30 => { if !self.registers.f.carry { self.jr(); 3 } else { self.registers.pc += 1; 2 } }
            0x31 => { self.registers.sp = self.fetch_word(); 3 }
            0x32 => { self.load(AddressHLD, RegA); 2 }
            0x33 => { self.inc_16(SP); 2 }
            0x34 => { self.alu_inc(AddressHL); 3 }
            0x35 => { self.alu_dec(AddressHL); 3 }
            0x36 => { self.load(AddressHL, Immediate8); 3 }
            0x37 => { self.registers.f.scf(); 1 }
            0x38 => { if self.registers.f.carry { self.jr(); 3 } else { self.registers.pc += 1; 2 } }
            0x39 => { self.add_16(SP); 2 }
            0x3a => { self.load(RegA, AddressHLD); 2 }
            0x3b => { self.dec_16(SP); 2 }
            0x3c => { self.alu_inc(RegA); 1 }
            0x3d => { self.alu_dec(RegA); 1 }
            0x3e => { self.load(RegA, Immediate8); 2 }
            0x3f => { self.ccf(); 1 }
            0x40 => { self.load(RegB, RegB); 1 }
            0x41 => { self.load(RegB, RegC); 1 }
            0x42 => { self.load(RegB, RegD); 1 }
            0x43 => { self.load(RegB, RegE); 1 }
            0x44 => { self.load(RegB, RegH); 1 }
            0x45 => { self.load(RegB, RegL); 1 }
            0x46 => { self.load(RegB, AddressHL); 2 }
            0x47 => { self.load(RegB, RegA); 1 }
            0x48 => { self.load(RegC, RegB); 1 }
            0x49 => { self.load(RegC, RegC); 1 }
            0x4a => { self.load(RegC, RegD); 1 }
            0x4b => { self.load(RegC, RegE); 1 }
            0x4c => { self.load(RegC, RegH); 1 }
            0x4d => { self.load(RegC, RegL); 1 }
            0x4e => { self.load(RegC, AddressHL); 2 }
            0x4f => { self.load(RegC, RegA); 1 }
            0x50 => { self.load(RegD, RegB); 1 }
            0x51 => { self.load(RegD, RegC); 1 }
            0x52 => { self.load(RegD, RegD); 1 }
            0x53 => { self.load(RegD, RegE); 1 }
            0x54 => { self.load(RegD, RegH); 1 }
            0x55 => { self.load(RegD, RegL); 1 }
            0x56 => { self.load(RegD, AddressHL); 2 }
            0x57 => { self.load(RegD, RegA); 1 }
            0x58 => { self.load(RegE, RegB); 1 }
            0x59 => { self.load(RegE, RegC); 1 }
            0x5a => { self.load(RegE, RegD); 1 }
            0x5b => { self.load(RegE, RegE); 1 }
            0x5c => { self.load(RegE, RegH); 1 }
            0x5d => { self.load(RegE, RegL); 1 }
            0x5e => { self.load(RegE, AddressHL); 2 }
            0x5f => { self.load(RegE, RegA); 1 }
            0x60 => { self.load(RegH, RegB); 1 }
            0x61 => { self.load(RegH, RegC); 1 }
            0x62 => { self.load(RegH, RegD); 1 }
            0x63 => { self.load(RegH, RegE); 1 }
            0x64 => { self.load(RegH, RegH); 1 }
            0x65 => { self.load(RegH, RegL); 1 }
            0x66 => { self.load(RegH, AddressHL); 2 }
            0x67 => { self.load(RegH, RegA); 1 }
            0x68 => { self.load(RegL, RegB); 1 }
            0x69 => { self.load(RegL, RegC); 1 }
            0x6a => { self.load(RegL, RegD); 1 }
            0x6b => { self.load(RegL, RegE); 1 }
            0x6c => { self.load(RegL, RegH); 1 }
            0x6d => { self.load(RegL, RegL); 1 }
            0x6e => { self.load(RegL, AddressHL); 2 }
            0x6f => { self.load(RegL, RegA); 1 }
            0x70 => { self.load(AddressHL, RegB); 2 }
            0x71 => { self.load(AddressHL, RegC); 2 }
            0x72 => { self.load(AddressHL, RegD); 2 }
            0x73 => { self.load(AddressHL, RegE); 2 }
            0x74 => { self.load(AddressHL, RegH); 2 }
            0x75 => { self.load(AddressHL, RegL); 2 }
            0x76 => { self.is_halted = true; 1 }
            0x77 => { self.load(AddressHL, RegA); 2 }
            0x78 => { self.load(RegA, RegB); 1 }
            0x79 => { self.load(RegA, RegC); 1 }
            0x7a => { self.load(RegA, RegD); 1 }
            0x7b => { self.load(RegA, RegE); 1 }
            0x7c => { self.load(RegA, RegH); 1 }
            0x7d => { self.load(RegA, RegL); 1 }
            0x7e => { self.load(RegA, AddressHL); 2 }
            0x7f => { self.load(RegA, RegA); 1 }
            0x80 => { self.alu_add(RegB); 1 }
            0x81 => { self.alu_add(RegC); 1 }
            0x82 => { self.alu_add(RegD); 1 }
            0x83 => { self.alu_add(RegE); 1 }
            0x84 => { self.alu_add(RegH); 1 }
            0x85 => { self.alu_add(RegL); 1 }
            0x86 => { self.alu_add(AddressHL); 2 }
            0x87 => { self.alu_add(RegA); 1 }
            0x88 => { self.alu_adc(RegB); 1 }
            0x89 => { self.alu_adc(RegC); 1 }
            0x8a => { self.alu_adc(RegD); 1 }
            0x8b => { self.alu_adc(RegE); 1 }
            0x8c => { self.alu_adc(RegH); 1 }
            0x8d => { self.alu_adc(RegL); 1 }
            0x8e => { self.alu_adc(AddressHL); 2 }
            0x8f => { self.alu_adc(RegA); 1 }
            0x90 => { self.alu_sub(RegB); 1 }
            0x91 => { self.alu_sub(RegC); 1 }
            0x92 => { self.alu_sub(RegD); 1 }
            0x93 => { self.alu_sub(RegE); 1 }
            0x94 => { self.alu_sub(RegH); 1 }
            0x95 => { self.alu_sub(RegL); 1 }
            0x96 => { self.alu_sub(AddressHL); 2 }
            0x97 => { self.alu_sub(RegA); 1 }
            0x98 => { self.alu_sbc(RegB); 1 }
            0x99 => { self.alu_sbc(RegC); 1 }
            0x9a => { self.alu_sbc(RegD); 1 }
            0x9b => { self.alu_sbc(RegE); 1 }
            0x9c => { self.alu_sbc(RegH); 1 }
            0x9d => { self.alu_sbc(RegL); 1 }
            0x9e => { self.alu_sbc(AddressHL); 2 }
            0x9f => { self.alu_sbc(RegA); 1 }
            0xa0 => { self.alu_and(RegB); 1 }
            0xa1 => { self.alu_and(RegC); 1 }
            0xa2 => { self.alu_and(RegD); 1 }
            0xa3 => { self.alu_and(RegE); 1 }
            0xa4 => { self.alu_and(RegH); 1 }
            0xa5 => { self.alu_and(RegL); 1 }
            0xa6 => { self.alu_and(AddressHL); 2 }
            0xa7 => { self.alu_and(RegA); 1 }
            0xa8 => { self.alu_xor(RegB); 1 }
            0xa9 => { self.alu_xor(RegC); 1 }
            0xaa => { self.alu_xor(RegD); 1 }
            0xab => { self.alu_xor(RegE); 1 }
            0xac => { self.alu_xor(RegH); 1 }
            0xad => { self.alu_xor(RegL); 1 }
            0xae => { self.alu_xor(AddressHL); 2 }
            0xaf => { self.alu_xor(RegA); 1 }
            0xb0 => { self.alu_or(RegB); 1 }
            0xb1 => { self.alu_or(RegC); 1 }
            0xb2 => { self.alu_or(RegD); 1 }
            0xb3 => { self.alu_or(RegE); 1 }
            0xb4 => { self.alu_or(RegH); 1 }
            0xb5 => { self.alu_or(RegL); 1 }
            0xb6 => { self.alu_or(AddressHL); 2 }
            0xb7 => { self.alu_or(RegA); 1 }
            0xb8 => { self.alu_cp(RegB); 1 }
            0xb9 => { self.alu_cp(RegC); 1 }
            0xba => { self.alu_cp(RegD); 1 }
            0xbb => { self.alu_cp(RegE); 1 }
            0xbc => { self.alu_cp(RegH); 1 }
            0xbd => { self.alu_cp(RegL); 1 }
            0xbe => { self.alu_cp(AddressHL); 2 }
            0xbf => { self.alu_cp(RegA); 1 }
            0xc0 => { if !self.registers.f.zero { self.registers.pc = self.pop_stack(); 5 } else { 2 } }
            0xc1 => { let value = self.pop_stack(); self.registers.write_16(BC, value); 3 }
            0xc2 => { if !self.registers.f.zero { self.registers.pc = self.fetch_word(); 4 } else { self.registers.pc += 2; 3 } }
            0xc3 => { self.registers.pc = self.fetch_word(); 4 }
            0xc4 => { if !self.registers.f.zero { self.push_stack(self.registers.pc + 2); self.registers.pc = self.fetch_word(); 6 } else { self.registers.pc += 2; 3 } }
            0xc5 => { self.push_stack(self.registers.read_16(BC)); 4 }
            0xc6 => { self.alu_add(Immediate8); 2 }
            0xc7 => { self.push_stack(self.registers.pc); self.registers.pc = 0x00; 4 }
            0xc8 => { if self.registers.f.zero { self.registers.pc = self.pop_stack(); 5 } else { 2 } }
            0xc9 => { self.registers.pc = self.pop_stack(); 4 }
            0xca => { if self.registers.f.zero { self.jp(); 4 } else { self.registers.pc = self.registers.pc.wrapping_add(2); 3 } }
            0xcb => { self.call_cb() }
            0xcc => { if self.registers.f.zero { self.push_stack(self.registers.pc + 2); self.registers.pc = self.fetch_word(); 6 } else { 3 } }
            0xcd => { self.push_stack(self.registers.pc + 2); self.registers.pc = self.fetch_word(); 6 }
            0xce => { self.alu_adc(Immediate8); 2 }
            0xcf => { self.push_stack(self.registers.pc); self.registers.pc = 0x08; 4 }
            0xd0 => { if !self.registers.f.carry { self.registers.pc = self.pop_stack(); 5 } else { 2 } }
            0xd1 => { let value = self.pop_stack(); self.registers.write_16(DE, value); 3 }
            0xd2 => { if !self.registers.f.carry { self.registers.pc = self.fetch_word(); 4 } else { self.registers.pc += 2; 3 } }
            0xd4 => { if !self.registers.f.carry { self.push_stack(self.registers.pc); self.registers.pc = self.fetch_word(); 6 } else { self.registers.pc += 2; 3 } }
            0xd5 => { self.push_stack(self.registers.read_16(DE)); 4 }
            0xd6 => { self.alu_sub(Immediate8); 2 }
            0xd7 => { self.push_stack(self.registers.pc); self.registers.pc = 0x10; 4 }
            0xd8 => { if self.registers.f.carry { self.registers.pc = self.pop_stack(); 5 } else { 2 } }
            0xd9 => { self.interrupt_master_enable.reti(); self.registers.pc = self.pop_stack(); 4 }
            0xda => { if self.registers.f.carry { self.registers.pc = self.fetch_word(); 4 } else { self.registers.pc += 2; 3 } }
            0xdc => { if self.registers.f.carry { self.push_stack(self.registers.pc + 2); self.registers.pc = self.fetch_word(); 6 } else { 3 } }
            0xde => { self.alu_sbc(Immediate8); 2 }
            0xdf => { self.push_stack(self.registers.pc); self.registers.pc = 0x18; 4 }
            0xe0 => { let address = 0xff00 | self.fetch_byte() as u16; self.bus.write_byte(address, self.registers.a); 3 }
            0xe1 => { let value = self.pop_stack(); self.registers.write_16(HL, value); 3 }
            0xe2 => { self.bus.write_byte(0xff00 | self.registers.c as u16, self.registers.a); 2 }
            0xe5 => { self.push_stack(self.registers.read_16(HL)); 4 }
            0xe6 => { self.alu_and(Immediate8); 2 }
            0xe7 => { self.push_stack(self.registers.pc); self.registers.pc = 0x20; 4 }
            0xe8 => { self.registers.sp = self.alu_add_s8(self.registers.sp); 4 }
            0xe9 => { self.registers.pc = self.registers.read_16(HL); 1 }
            0xea => { let address = self.fetch_word(); self.bus.write_byte(address, self.registers.a); 4 }
            0xee => { self.alu_xor(Immediate8); 2 }
            0xef => { self.push_stack(self.registers.pc); self.registers.pc = 0x28; 4 }
            0xf0 => { let address = 0xff00 | self.fetch_byte() as u16; self.registers.a = self.bus.read_byte(address); 3 }
            0xf1 => { let value = self.pop_stack(); self.registers.write_16(AF, value); 4 }
            0xf2 => { let address = 0xff00 | self.registers.c as u16; self.registers.a = self.bus.read_byte(address); 2 }
            0xf3 => { self.interrupt_master_enable.di(); 1 }
            0xf5 => { self.push_stack(self.registers.read_16(AF)); 4 }
            0xf6 => { self.alu_or(Immediate8); 2 }
            0xf7 => { self.push_stack(self.registers.pc); self.registers.pc = 0x30; 4 }
            0xf8 => { let sum = self.alu_add_s8(self.registers.sp); self.registers.write_16(HL, sum); 3 }
            0xf9 => { self.registers.sp = self.registers.read_16(HL); 2 }
            0xfa => { let address = self.fetch_word(); self.registers.a = self.bus.read_byte(address); 4 }
            0xfb => { self.interrupt_master_enable.ei(); 1 }
            0xfe => { self.alu_cp(Immediate8); 2 }
            0xff => { self.push_stack(self.registers.pc); self.registers.pc = 0x38; 4 }
            _ => panic!("Instruksjon ikke støttet: 0x{:2x}", opcode)
        }
    }
    fn call_cb(&mut self) -> u32 {
        let opcode = self.fetch_byte();
        debug!("Dekoder nå opkode {:#04x} (etter CB-prefiks)", opcode);
        match opcode {
            0x00 => { self.rlc(RegB); 2 }
            0x01 => { self.rlc(RegC); 2 }
            0x02 => { self.rlc(RegD); 2 }
            0x03 => { self.rlc(RegE); 2 }
            0x04 => { self.rlc(RegH); 2 }
            0x05 => { self.rlc(RegL); 2 }
            0x06 => { self.rlc(AddressHL); 4 }
            0x07 => { self.rlc(RegA); 2 }
            0x08 => { self.rrc(RegB); 2 }
            0x09 => { self.rrc(RegC); 2 }
            0x0a => { self.rrc(RegD); 2 }
            0x0b => { self.rrc(RegE); 2 }
            0x0c => { self.rrc(RegH); 2 }
            0x0d => { self.rrc(RegL); 2 }
            0x0e => { self.rrc(AddressHL); 4 }
            0x0f => { self.rrc(RegA); 2 }
            0x10 => { self.rl(RegB); 2 }
            0x11 => { self.rl(RegC); 2 }
            0x12 => { self.rl(RegD); 2 }
            0x13 => { self.rl(RegE); 2 }
            0x14 => { self.rl(RegH); 2 }
            0x15 => { self.rl(RegL); 2 }
            0x16 => { self.rl(AddressHL); 4 }
            0x17 => { self.rl(RegA); 2 }
            0x18 => { self.rr(RegB); 2 }
            0x19 => { self.rr(RegC); 2 }
            0x1a => { self.rr(RegD); 2 }
            0x1b => { self.rr(RegE); 2 }
            0x1c => { self.rr(RegH); 2 }
            0x1d => { self.rr(RegL); 2 }
            0x1e => { self.rr(AddressHL); 4 }
            0x1f => { self.rr(RegA); 2 }
            0x20 => { self.sla(RegB); 2 }
            0x21 => { self.sla(RegC); 2 }
            0x22 => { self.sla(RegD); 2 }
            0x23 => { self.sla(RegE); 2 }
            0x24 => { self.sla(RegH); 2 }
            0x25 => { self.sla(RegL); 2 }
            0x26 => { self.sla(AddressHL); 4 }
            0x27 => { self.sla(RegA); 2 }
            0x28 => { self.sra(RegB); 2 }
            0x29 => { self.sra(RegC); 2 }
            0x2a => { self.sra(RegD); 2 }
            0x2b => { self.sra(RegE); 2 }
            0x2c => { self.sra(RegH); 2 }
            0x2d => { self.sra(RegL); 2 }
            0x2e => { self.sra(AddressHL); 4 }
            0x2f => { self.sra(RegA); 2 }
            0x30 => { self.swap(RegB); 2 }
            0x31 => { self.swap(RegC); 2 }
            0x32 => { self.swap(RegD); 2 }
            0x33 => { self.swap(RegE); 2 }
            0x34 => { self.swap(RegH); 2 }
            0x35 => { self.swap(RegL); 2 }
            0x36 => { self.swap(AddressHL); 4 }
            0x37 => { self.swap(RegA); 2 }
            0x38 => { self.srl(RegB); 2 }
            0x39 => { self.srl(RegC); 2 }
            0x3a => { self.srl(RegD); 2 }
            0x3b => { self.srl(RegE); 2 }
            0x3c => { self.srl(RegH); 2 }
            0x3d => { self.srl(RegL); 2 }
            0x3e => { self.srl(AddressHL); 4 }
            0x3f => { self.srl(RegA); 2 }
            0x40 => { self.bit(RegB, 0); 2 }
            0x41 => { self.bit(RegC, 0); 2 }
            0x42 => { self.bit(RegD, 0); 2 }
            0x43 => { self.bit(RegE, 0); 2 }
            0x44 => { self.bit(RegH, 0); 2 }
            0x45 => { self.bit(RegL, 0); 2 }
            0x46 => { self.bit(AddressHL, 0); 3 }
            0x47 => { self.bit(RegA, 0); 2 }
            0x48 => { self.bit(RegB, 1); 2 }
            0x49 => { self.bit(RegC, 1); 2 }
            0x4a => { self.bit(RegD, 1); 2 }
            0x4b => { self.bit(RegE, 1); 2 }
            0x4c => { self.bit(RegH, 1); 2 }
            0x4d => { self.bit(RegL, 1); 2 }
            0x4e => { self.bit(AddressHL, 1); 3 }
            0x4f => { self.bit(RegA, 1); 2 }
            0x50 => { self.bit(RegB, 2); 2 }
            0x51 => { self.bit(RegC, 2); 2 }
            0x52 => { self.bit(RegD, 2); 2 }
            0x53 => { self.bit(RegE, 2); 2 }
            0x54 => { self.bit(RegH, 2); 2 }
            0x55 => { self.bit(RegL, 2); 2 }
            0x56 => { self.bit(AddressHL, 2); 3 }
            0x57 => { self.bit(RegA, 2); 2 }
            0x58 => { self.bit(RegB, 3); 2 }
            0x59 => { self.bit(RegC, 3); 2 }
            0x5a => { self.bit(RegD, 3); 2 }
            0x5b => { self.bit(RegE, 3); 2 }
            0x5c => { self.bit(RegH, 3); 2 }
            0x5d => { self.bit(RegL, 3); 2 }
            0x5e => { self.bit(AddressHL, 3); 3 }
            0x5f => { self.bit(RegA, 3); 2 }
            0x60 => { self.bit(RegB, 4); 2 }
            0x61 => { self.bit(RegC, 4); 2 }
            0x62 => { self.bit(RegD, 4); 2 }
            0x63 => { self.bit(RegE, 4); 2 }
            0x64 => { self.bit(RegH, 4); 2 }
            0x65 => { self.bit(RegL, 4); 2 }
            0x66 => { self.bit(AddressHL, 4); 3 }
            0x67 => { self.bit(RegA, 4); 2 }
            0x68 => { self.bit(RegB, 5); 2 }
            0x69 => { self.bit(RegC, 5); 2 }
            0x6a => { self.bit(RegD, 5); 2 }
            0x6b => { self.bit(RegE, 5); 2 }
            0x6c => { self.bit(RegH, 5); 2 }
            0x6d => { self.bit(RegL, 5); 2 }
            0x6e => { self.bit(AddressHL, 5); 3 }
            0x6f => { self.bit(RegA, 5); 2 }
            0x70 => { self.bit(RegB, 6); 2 }
            0x71 => { self.bit(RegC, 6); 2 }
            0x72 => { self.bit(RegD, 6); 2 }
            0x73 => { self.bit(RegE, 6); 2 }
            0x74 => { self.bit(RegH, 6); 2 }
            0x75 => { self.bit(RegL, 6); 2 }
            0x76 => { self.bit(AddressHL, 6); 3 }
            0x77 => { self.bit(RegA, 6); 2 }
            0x78 => { self.bit(RegB, 7); 2 }
            0x79 => { self.bit(RegC, 7); 2 }
            0x7a => { self.bit(RegD, 7); 2 }
            0x7b => { self.bit(RegE, 7); 2 }
            0x7c => { self.bit(RegH, 7); 2 }
            0x7d => { self.bit(RegL, 7); 2 }
            0x7e => { self.bit(AddressHL, 7); 3 }
            0x7f => { self.bit(RegA, 7); 2 }
            0x80 => { self.res(RegB, 0); 2 }
            0x81 => { self.res(RegC, 0); 2 }
            0x82 => { self.res(RegD, 0); 2 }
            0x83 => { self.res(RegE, 0); 2 }
            0x84 => { self.res(RegH, 0); 2 }
            0x85 => { self.res(RegL, 0); 2 }
            0x86 => { self.res(AddressHL, 0); 4 }
            0x87 => { self.res(RegA, 0); 2 }
            0x88 => { self.res(RegB, 1); 2 }
            0x89 => { self.res(RegC, 1); 2 }
            0x8a => { self.res(RegD, 1); 2 }
            0x8b => { self.res(RegE, 1); 2 }
            0x8c => { self.res(RegH, 1); 2 }
            0x8d => { self.res(RegL, 1); 2 }
            0x8e => { self.res(AddressHL, 1); 4 }
            0x8f => { self.res(RegA, 1); 2 }
            0x90 => { self.res(RegB, 2); 2 }
            0x91 => { self.res(RegC, 2); 2 }
            0x92 => { self.res(RegD, 2); 2 }
            0x93 => { self.res(RegE, 2); 2 }
            0x94 => { self.res(RegH, 2); 2 }
            0x95 => { self.res(RegL, 2); 2 }
            0x96 => { self.res(AddressHL, 2); 4 }
            0x97 => { self.res(RegA, 2); 2 }
            0x98 => { self.res(RegB, 3); 2 }
            0x99 => { self.res(RegC, 3); 2 }
            0x9a => { self.res(RegD, 3); 2 }
            0x9b => { self.res(RegE, 3); 2 }
            0x9c => { self.res(RegH, 3); 2 }
            0x9d => { self.res(RegL, 3); 2 }
            0x9e => { self.res(AddressHL, 3); 4 }
            0x9f => { self.res(RegA, 3); 2 }
            0xa0 => { self.res(RegB, 4); 2 }
            0xa1 => { self.res(RegC, 4); 2 }
            0xa2 => { self.res(RegD, 4); 2 }
            0xa3 => { self.res(RegE, 4); 2 }
            0xa4 => { self.res(RegH, 4); 2 }
            0xa5 => { self.res(RegL, 4); 2 }
            0xa6 => { self.res(AddressHL, 4); 4 }
            0xa7 => { self.res(RegA, 4); 2 }
            0xa8 => { self.res(RegB, 5); 2 }
            0xa9 => { self.res(RegC, 5); 2 }
            0xaa => { self.res(RegD, 5); 2 }
            0xab => { self.res(RegE, 5); 2 }
            0xac => { self.res(RegH, 5); 2 }
            0xad => { self.res(RegL, 5); 2 }
            0xae => { self.res(AddressHL, 5); 4 }
            0xaf => { self.res(RegA, 5); 2 }
            0xb0 => { self.res(RegB, 6); 2 }
            0xb1 => { self.res(RegC, 6); 2 }
            0xb2 => { self.res(RegD, 6); 2 }
            0xb3 => { self.res(RegE, 6); 2 }
            0xb4 => { self.res(RegH, 6); 2 }
            0xb5 => { self.res(RegL, 6); 2 }
            0xb6 => { self.res(AddressHL, 6); 4 }
            0xb7 => { self.res(RegA, 6); 2 }
            0xb8 => { self.res(RegB, 7); 2 }
            0xb9 => { self.res(RegC, 7); 2 }
            0xba => { self.res(RegD, 7); 2 }
            0xbb => { self.res(RegE, 7); 2 }
            0xbc => { self.res(RegH, 7); 2 }
            0xbd => { self.res(RegL, 7); 2 }
            0xbe => { self.res(AddressHL, 7); 4 }
            0xbf => { self.res(RegA, 7); 2 }
            0xc0 => { self.set(RegB, 0); 2 }
            0xc1 => { self.set(RegC, 0); 2 }
            0xc2 => { self.set(RegD, 0); 2 }
            0xc3 => { self.set(RegE, 0); 2 }
            0xc4 => { self.set(RegH, 0); 2 }
            0xc5 => { self.set(RegL, 0); 2 }
            0xc6 => { self.set(AddressHL, 0); 4 }
            0xc7 => { self.set(RegA, 0); 2 }
            0xc8 => { self.set(RegB, 1); 2 }
            0xc9 => { self.set(RegC, 1); 2 }
            0xca => { self.set(RegD, 1); 2 }
            0xcb => { self.set(RegE, 1); 2 }
            0xcc => { self.set(RegH, 1); 2 }
            0xcd => { self.set(RegL, 1); 2 }
            0xce => { self.set(AddressHL, 1); 4 }
            0xcf => { self.set(RegA, 1); 2 }
            0xd0 => { self.set(RegB, 2); 2 }
            0xd1 => { self.set(RegC, 2); 2 }
            0xd2 => { self.set(RegD, 2); 2 }
            0xd3 => { self.set(RegE, 2); 2 }
            0xd4 => { self.set(RegH, 2); 2 }
            0xd5 => { self.set(RegL, 2); 2 }
            0xd6 => { self.set(AddressHL, 2); 4 }
            0xd7 => { self.set(RegA, 2); 2 }
            0xd8 => { self.set(RegB, 3); 2 }
            0xd9 => { self.set(RegC, 3); 2 }
            0xda => { self.set(RegD, 3); 2 }
            0xdb => { self.set(RegE, 3); 2 }
            0xdc => { self.set(RegH, 3); 2 }
            0xdd => { self.set(RegL, 3); 2 }
            0xde => { self.set(AddressHL, 3); 4 }
            0xdf => { self.set(RegA, 3); 2 }
            0xe0 => { self.set(RegB, 4); 2 }
            0xe1 => { self.set(RegC, 4); 2 }
            0xe2 => { self.set(RegD, 4); 2 }
            0xe3 => { self.set(RegE, 4); 2 }
            0xe4 => { self.set(RegH, 4); 2 }
            0xe5 => { self.set(RegL, 4); 2 }
            0xe6 => { self.set(AddressHL, 4); 4 }
            0xe7 => { self.set(RegA, 4); 2 }
            0xe8 => { self.set(RegB, 5); 2 }
            0xe9 => { self.set(RegC, 5); 2 }
            0xea => { self.set(RegD, 5); 2 }
            0xeb => { self.set(RegE, 5); 2 }
            0xec => { self.set(RegH, 5); 2 }
            0xed => { self.set(RegL, 5); 2 }
            0xee => { self.set(AddressHL, 5); 4 }
            0xef => { self.set(RegA, 5); 2 }
            0xf0 => { self.set(RegB, 6); 2 }
            0xf1 => { self.set(RegC, 6); 2 }
            0xf2 => { self.set(RegD, 6); 2 }
            0xf3 => { self.set(RegE, 6); 2 }
            0xf4 => { self.set(RegH, 6); 2 }
            0xf5 => { self.set(RegL, 6); 2 }
            0xf6 => { self.set(AddressHL, 6); 4 }
            0xf7 => { self.set(RegA, 6); 2 }
            0xf8 => { self.set(RegB, 7); 2 }
            0xf9 => { self.set(RegC, 7); 2 }
            0xfa => { self.set(RegD, 7); 2 }
            0xfb => { self.set(RegE, 7); 2 }
            0xfc => { self.set(RegH, 7); 2 }
            0xfd => { self.set(RegL, 7); 2 }
            0xfe => { self.set(AddressHL, 7); 4 }
            0xff => { self.set(RegA, 7); 2 }
            _ => panic!("CB-instruksjon ikke støttet: 0x{:2x}", opcode)
        }
    }
}