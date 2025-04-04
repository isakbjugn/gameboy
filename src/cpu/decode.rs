use log::debug;
use crate::cpu::CPU;
use crate::cpu::execute::Address;
use crate::cpu::read_write::Operand::{RegA, RegB, RegC, RegD, RegE, RegH, RegL, AddressHL, Immediate8};
use crate::cpu::registers::Reg8::{A, B, C, D, E, H, L};
use crate::cpu::registers::Reg16::{AF, BC, DE, HL, SP};

impl CPU {
    pub fn call(&mut self) -> u32 {
        let opcode = self.fetch_byte();
        debug!("Dekoder nå opkode {:#04x}", opcode);
        match opcode {
            0x00 => { 1 }
            0x01 => { let word = self.fetch_word(); self.registers.write_16(BC, word); 3 }
            0x02 => { self.bus.write_byte(self.registers.read_16(BC), self.registers.a); 2 }
            0x03 => { self.inc_16(BC); 2 }
            0x04 => { self.inc(B); 1 }
            0x05 => { self.dec(B); 1 }
            0x06 => { self.registers.b = self.fetch_byte(); 2 }
            0x07 => { self.rlca(); 1 }
            0x08 => { let address = self.fetch_word(); self.bus.write_word(address, self.registers.sp); 5 }
            0x09 => { self.add_16(BC); 2 }
            0x0a => { self.registers.a = self.bus.read_byte(self.registers.read_16(BC)); 2 }
            0x0b => { self.dec_16(BC); 2 }
            0x0c => { self.inc(C); 1 }
            0x0d => { self.dec(C); 1 }
            0x0e => { self.registers.c = self.fetch_byte(); 2 }
            0x0f => { self.rrca(); 1 }
            0x10 => { panic!("STOP") }
            0x11 => { let word = self.fetch_word(); self.registers.write_16(DE, word); 3 }
            0x12 => { self.bus.write_byte(self.registers.read_16(DE), self.registers.a); 2 }
            0x13 => { self.inc_16(DE); 2 }
            0x14 => { self.inc(D); 1 }
            0x15 => { self.dec(D); 1 }
            0x16 => { self.registers.d = self.fetch_byte(); 2 }
            0x17 => { self.rla(); 1 }
            0x18 => { self.jr(); 3 }
            0x19 => { self.add_16(DE); 2 }
            0x1a => { self.registers.a = self.bus.read_byte(self.registers.read_16(DE)); 2 }
            0x1b => { self.dec_16(DE); 2 }
            0x1c => { self.inc(E); 1 }
            0x1d => { self.dec(E); 1 }
            0x1e => { self.registers.e = self.fetch_byte(); 2 }
            0x1f => { self.rra(); 1 }
            0x20 => { if !self.registers.f.zero { self.jr(); 3 } else { self.registers.pc += 1; 2 } }
            0x21 => { let word = self.fetch_word(); self.registers.write_16(HL, word); 3 }
            0x22 => { self.bus.write_byte(self.registers.hli(), self.registers.a); 2 }
            0x23 => { self.inc_16(HL); 2 }
            0x24 => { self.inc(H); 1 }
            0x25 => { self.dec(H); 1 }
            0x26 => { self.registers.h = self.fetch_byte(); 2 }
            0x27 => { self.daa(); 1 }
            0x28 => { if self.registers.f.zero { self.jr(); 3 } else { self.registers.pc += 1; 2 } }
            0x29 => { self.add_16(HL); 2 }
            0x2a => { self.registers.a = self.bus.read_byte(self.registers.hli()); 2 }
            0x2b => { self.dec_16(HL); 2 }
            0x2c => { self.inc(L); 1 }
            0x2d => { self.dec(L); 1 }
            0x2e => { self.registers.l = self.fetch_byte(); 2 }
            0x2f => { self.cpl(); 1 }
            0x30 => { if !self.registers.f.carry { self.jr(); 3 } else { self.registers.pc += 1; 2 } }
            0x31 => { self.registers.sp = self.fetch_word(); 3 }
            0x32 => { self.bus.write_byte(self.registers.hld(), self.registers.a); 2 }
            0x33 => { self.inc_16(SP); 2 }
            0x34 => { self.inc_addr(Address::HL); 3 }
            0x35 => { self.dec_addr(Address::HL); 3 }
            0x36 => { let d8 = self.fetch_byte(); self.bus.write_byte(self.registers.read_16(HL), d8); 3 }
            0x37 => { self.registers.f.scf(); 1 }
            0x38 => { if self.registers.f.carry { self.jr(); 3 } else { self.registers.pc += 1; 2 } }
            0x39 => { self.add_16(SP); 2 }
            0x3a => { self.registers.a = self.bus.read_byte(self.registers.hld()); 2 }
            0x3b => { self.dec_16(SP); 2 }
            0x3c => { self.inc(A); 1 }
            0x3d => { self.dec(A); 1 }
            0x3e => { self.registers.a = self.fetch_byte(); 2 }
            0x3f => { self.ccf(); 1 }
            0x40 => { 1 }
            0x41 => { self.registers.b = self.registers.c; 1 }
            0x42 => { self.registers.b = self.registers.d; 1 }
            0x43 => { self.registers.b = self.registers.e; 1 }
            0x44 => { self.registers.b = self.registers.h; 1 }
            0x45 => { self.registers.b = self.registers.l; 1 }
            0x46 => { self.registers.b = self.bus.read_byte(self.registers.read_16(HL)); 2 }
            0x47 => { self.registers.b = self.registers.a; 1 }
            0x48 => { self.registers.c = self.registers.b; 1 }
            0x49 => { 1 }
            0x4a => { self.registers.c = self.registers.d; 1 }
            0x4b => { self.registers.c = self.registers.e; 1 }
            0x4c => { self.registers.c = self.registers.h; 1 }
            0x4d => { self.registers.c = self.registers.l; 1 }
            0x4e => { self.registers.c = self.bus.read_byte(self.registers.read_16(HL)); 2 }
            0x4f => { self.registers.c = self.registers.a; 1 }
            0x50 => { self.registers.d = self.registers.b; 1 }
            0x51 => { self.registers.d = self.registers.c; 1 }
            0x52 => { 1 }
            0x53 => { self.registers.d = self.registers.e; 1 }
            0x54 => { self.registers.d = self.registers.h; 1 }
            0x55 => { self.registers.d = self.registers.l; 1 }
            0x56 => { self.registers.d = self.bus.read_byte(self.registers.read_16(HL)); 2 }
            0x57 => { self.registers.d = self.registers.a; 1 }
            0x58 => { self.registers.e = self.registers.b; 1 }
            0x59 => { self.registers.e = self.registers.c; 1 }
            0x5a => { self.registers.e = self.registers.d; 1 }
            0x5b => { 1 }
            0x5c => { self.registers.e = self.registers.h; 1 }
            0x5d => { self.registers.e = self.registers.l; 1 }
            0x5e => { self.registers.e = self.bus.read_byte(self.registers.read_16(HL)); 2 }
            0x5f => { self.registers.e = self.registers.a; 1 }
            0x60 => { self.registers.h = self.registers.b; 1 }
            0x61 => { self.registers.h = self.registers.c; 1 }
            0x62 => { self.registers.h = self.registers.d; 1 }
            0x63 => { self.registers.h = self.registers.e; 1 }
            0x64 => { 1 }
            0x65 => { self.registers.h = self.registers.l; 1 }
            0x66 => { self.registers.h = self.bus.read_byte(self.registers.read_16(HL)); 2 }
            0x67 => { self.registers.h = self.registers.a; 1 }
            0x68 => { self.registers.l = self.registers.b; 1 }
            0x69 => { self.registers.l = self.registers.c; 1 }
            0x6a => { self.registers.l = self.registers.d; 1 }
            0x6b => { self.registers.l = self.registers.e; 1 }
            0x6c => { self.registers.l = self.registers.h; 1 }
            0x6d => { 1 }
            0x6e => { self.registers.l = self.bus.read_byte(self.registers.read_16(HL)); 2 }
            0x6f => { self.registers.l = self.registers.a; 1 }
            0x70 => { self.bus.write_byte(self.registers.read_16(HL), self.registers.b); 2 }
            0x71 => { self.bus.write_byte(self.registers.read_16(HL), self.registers.c); 2 }
            0x72 => { self.bus.write_byte(self.registers.read_16(HL), self.registers.d); 2 }
            0x73 => { self.bus.write_byte(self.registers.read_16(HL), self.registers.e); 2 }
            0x74 => { self.bus.write_byte(self.registers.read_16(HL), self.registers.h); 2 }
            0x75 => { self.bus.write_byte(self.registers.read_16(HL), self.registers.l); 2 }
            0x76 => { self.is_halted = true; 1 }
            0x77 => { self.bus.write_byte(self.registers.read_16(HL), self.registers.a); 2 }
            0x78 => { self.registers.a = self.registers.b; 1 }
            0x79 => { self.registers.a = self.registers.c; 1 }
            0x7a => { self.registers.a = self.registers.d; 1 }
            0x7b => { self.registers.a = self.registers.e; 1 }
            0x7c => { self.registers.a = self.registers.h; 1 }
            0x7d => { self.registers.a = self.registers.l; 1 }
            0x7e => { self.registers.a = self.bus.read_byte(self.registers.read_16(HL)); 2 }
            0x7f => { 1 }
            0x80 => { self.alu_add(self.registers.b); 1 }
            0x81 => { self.alu_add(self.registers.c); 1 }
            0x82 => { self.alu_add(self.registers.d); 1 }
            0x83 => { self.alu_add(self.registers.e); 1 }
            0x84 => { self.alu_add(self.registers.h); 1 }
            0x85 => { self.alu_add(self.registers.l); 1 }
            0x86 => { let byte = self.bus.read_byte(self.registers.read_16(HL)); self.alu_add(byte); 2 }
            0x87 => { self.alu_add(self.registers.a); 1 }
            0x88 => { self.alu_adc(self.registers.b); 1 }
            0x89 => { self.alu_adc(self.registers.c); 1 }
            0x8a => { self.alu_adc(self.registers.d); 1 }
            0x8b => { self.alu_adc(self.registers.e); 1 }
            0x8c => { self.alu_adc(self.registers.h); 1 }
            0x8d => { self.alu_adc(self.registers.l); 1 }
            0x8e => { let byte = self.bus.read_byte(self.registers.read_16(HL)); self.alu_adc(byte); 2 }
            0x8f => { self.alu_adc(self.registers.a); 1 }
            0x90 => { self.alu_sub(self.registers.b); 1 }
            0x91 => { self.alu_sub(self.registers.c); 1 }
            0x92 => { self.alu_sub(self.registers.d); 1 }
            0x93 => { self.alu_sub(self.registers.e); 1 }
            0x94 => { self.alu_sub(self.registers.h); 1 }
            0x95 => { self.alu_sub(self.registers.l); 1 }
            0x96 => { let byte = self.bus.read_byte(self.registers.read_16(HL)); self.alu_sub(byte); 2 }
            0x97 => { self.alu_sub(self.registers.a); 1 }
            0x98 => { self.alu_sbc(self.registers.b); 1 }
            0x99 => { self.alu_sbc(self.registers.c); 1 }
            0x9a => { self.alu_sbc(self.registers.d); 1 }
            0x9b => { self.alu_sbc(self.registers.e); 1 }
            0x9c => { self.alu_sbc(self.registers.h); 1 }
            0x9d => { self.alu_sbc(self.registers.l); 1 }
            0x9e => { let byte = self.bus.read_byte(self.registers.read_16(HL)); self.alu_sbc(byte); 2 }
            0x9f => { self.alu_sbc(self.registers.a); 1 }
            0xa0 => { self.alu_and(self.registers.b); 1 }
            0xa1 => { self.alu_and(self.registers.c); 1 }
            0xa2 => { self.alu_and(self.registers.d); 1 }
            0xa3 => { self.alu_and(self.registers.e); 1 }
            0xa4 => { self.alu_and(self.registers.h); 1 }
            0xa5 => { self.alu_and(self.registers.l); 1 }
            0xa6 => { let byte = self.bus.read_byte(self.registers.read_16(HL)); self.alu_and(byte); 2 }
            0xa7 => { self.alu_and(self.registers.a); 1 }
            0xa8 => { self.alu_and(self.registers.b); 1 }
            0xa9 => { self.alu_xor(self.registers.c); 1 }
            0xaa => { self.alu_xor(self.registers.d); 1 }
            0xab => { self.alu_xor(self.registers.e); 1 }
            0xac => { self.alu_xor(self.registers.h); 1 }
            0xad => { self.alu_xor(self.registers.l); 1 }
            0xae => { let byte = self.bus.read_byte(self.registers.read_16(HL)); self.alu_xor(byte); 2 }
            0xaf => { self.alu_xor(self.registers.a); 1 }
            0xb0 => { self.alu_or(B); 1 }
            0xb1 => { self.alu_or(C); 1 }
            0xb2 => { self.alu_or(D); 1 }
            0xb3 => { self.alu_or(E); 1 }
            0xb4 => { self.alu_or(H); 1 }
            0xb5 => { self.alu_or(L); 1 }
            0xb6 => { let value = self.bus.read_byte(self.registers.read_16(HL)); self.alu_or_val(value); 2 }
            0xb7 => { self.alu_or(A); 1 }
            0xb8 => { self.alu_cp(self.registers.b); 1 }
            0xb9 => { self.alu_cp(self.registers.c); 1 }
            0xba => { self.alu_cp(self.registers.d); 1 }
            0xbb => { self.alu_cp(self.registers.e); 1 }
            0xbc => { self.alu_cp(self.registers.h); 1 }
            0xbd => { self.alu_cp(self.registers.c); 1 }
            0xbe => { let byte = self.bus.read_byte(self.registers.read_16(HL)); self.alu_cp(byte); 2 }
            0xbf => { self.alu_cp(self.registers.a); 1 }
            0xc0 => { if !self.registers.f.zero { self.registers.pc = self.pop_stack(); 5 } else { 2 } }
            0xc1 => { let value = self.pop_stack(); self.registers.write_16(BC, value); 3 }
            0xc2 => { if !self.registers.f.zero { self.registers.pc = self.fetch_word(); 4 } else { self.registers.pc += 2; 3 } }
            0xc3 => { self.registers.pc = self.fetch_word(); 4 }
            0xc4 => { if !self.registers.f.zero { self.push_stack(self.registers.pc + 2); self.registers.pc = self.fetch_word(); 6 } else { self.registers.pc += 2; 3 } }
            0xc5 => { self.push_stack(self.registers.read_16(BC)); 4 }
            0xc6 => { let value = self.fetch_byte(); self.alu_add(value); 2 }
            0xc7 => { self.push_stack(self.registers.pc); self.registers.pc = 0x00; 4 }
            0xc8 => { if self.registers.f.zero { self.registers.pc = self.pop_stack(); 5 } else { 2 } }
            0xc9 => { self.registers.pc = self.pop_stack(); 4 }
            0xca => { if self.registers.f.zero { self.jp(); 4 } else { self.registers.pc = self.registers.pc.wrapping_add(2); 3 } }
            0xcb => { self.call_cb() }
            0xcc => { if self.registers.f.zero { self.push_stack(self.registers.pc + 2); self.registers.pc = self.fetch_word(); 6 } else { 3 } }
            0xcd => { self.push_stack(self.registers.pc + 2); self.registers.pc = self.fetch_word(); 6 }
            0xce => { let byte = self.fetch_byte(); self.alu_adc(byte); 2 }
            0xcf => { self.push_stack(self.registers.pc); self.registers.pc = 0x08; 4 }
            0xd0 => { if !self.registers.f.carry { self.registers.pc = self.pop_stack(); 5 } else { 2 } }
            0xd1 => { let value = self.pop_stack(); self.registers.write_16(DE, value); 3 }
            0xd2 => { if !self.registers.f.carry { self.registers.pc = self.fetch_word(); 4 } else { self.registers.pc += 2; 3 } }
            0xd4 => { if !self.registers.f.carry { self.push_stack(self.registers.pc); self.registers.pc = self.fetch_word(); 6 } else { self.registers.pc += 2; 3 } }
            0xd5 => { self.push_stack(self.registers.read_16(DE)); 4 }
            0xd6 => { let byte = self.fetch_byte(); self.alu_sub(byte); 2 }
            0xd7 => { self.push_stack(self.registers.pc); self.registers.pc = 0x10; 4 }
            0xd8 => { if self.registers.f.carry { self.registers.pc = self.pop_stack(); 5 } else { 2 } }
            0xd9 => { self.interrupt_master_enable.reti(); self.registers.pc = self.pop_stack(); 4 }
            0xda => { if self.registers.f.carry { self.registers.pc = self.fetch_word(); 4 } else { self.registers.pc += 2; 3 } }
            0xdc => { if self.registers.f.carry { self.push_stack(self.registers.pc + 2); self.registers.pc = self.fetch_word(); 6 } else { 3 } }
            0xde => { let byte = self.fetch_byte(); self.alu_sbc(byte); 1 }
            0xdf => { self.push_stack(self.registers.pc); self.registers.pc = 0x18; 4 }
            0xe0 => { let address = 0xff00 | self.fetch_byte() as u16; self.bus.write_byte(address, self.registers.a); 3 }
            0xe1 => { let value = self.pop_stack(); self.registers.write_16(HL, value); 3 }
            0xe2 => { self.bus.write_byte(0xff00 | self.registers.c as u16, self.registers.a); 2 }
            0xe5 => { self.push_stack(self.registers.read_16(HL)); 4 }
            0xe6 => { let byte = self.fetch_byte(); self.alu_and(byte); 2 }
            0xe7 => { self.push_stack(self.registers.pc); self.registers.pc = 0x20; 4 }
            0xe8 => { self.registers.sp = self.alu_add_s8(self.registers.sp); 4 }
            0xe9 => { self.registers.pc = self.registers.read_16(HL); 1 }
            0xea => { let address = self.fetch_word(); self.bus.write_byte(address, self.registers.a); 4 }
            0xee => { let byte = self.fetch_byte(); self.alu_xor(byte); 2 }
            0xef => { self.push_stack(self.registers.pc); self.registers.pc = 0x28; 4 }
            0xf0 => { let address = 0xff00 | self.fetch_byte() as u16; self.registers.a = self.bus.read_byte(address); 3 }
            0xf1 => { let value = self.pop_stack(); self.registers.write_16(AF, value); 4 }
            0xf2 => { let address = 0xff00 | self.registers.c as u16; self.registers.a = self.bus.read_byte(address); 2 }
            0xf3 => { self.interrupt_master_enable.di(); 1 }
            0xf5 => { self.push_stack(self.registers.read_16(AF)); 4 }
            0xf6 => { let byte = self.fetch_byte(); self.alu_or_val(byte); 2 }
            0xf7 => { self.push_stack(self.registers.pc); self.registers.pc = 0x30; 4 }
            0xf8 => { let sum = self.alu_add_s8(self.registers.sp); self.registers.write_16(HL, sum); 3 }
            0xf9 => { self.registers.sp = self.registers.read_16(HL); 2 }
            0xfa => { let address = self.fetch_word(); self.registers.a = self.bus.read_byte(address); 4 }
            0xfb => { self.interrupt_master_enable.ei(); 1 }
            0xfe => { let value = self.fetch_byte(); self.alu_cp(value); 2 }
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
            
            0x37 => { self.registers.a = self.alu_swap(self.registers.a); 2 }
            0x38 => { self.alu_srl(B); 2 }
            
            0x7c => { self.alu_bit(self.registers.h, 7); 2 }
            
            0xd1 => { self.registers.c |= 0x2; 2 }
            _ => panic!("CB-instruksjon ikke støttet: 0x{:2x}", opcode)
        }
    }
}