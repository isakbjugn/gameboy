use crate::cpu::CPU;
use crate::cpu::execute::Address;
use crate::cpu::registers::Reg8::{A, B, C, D, E, H, L};
use crate::cpu::registers::Reg16::{BC, DE, HL, SP};

impl CPU {
    pub fn call(&mut self) -> u32 {
        let opcode = self.fetch_byte();
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
            0x20 => { if !self.registers.f.zero { self.jr(); 3 } else { 2 } }
            0x21 => { let word = self.fetch_word(); self.registers.write_16(HL, word); 3 }
            0x22 => { self.bus.write_byte(self.registers.hli(), self.registers.a); 2 }
            0x23 => { self.inc_16(HL); 2 }
            0x24 => { self.inc(H); 1 }
            0x25 => { self.dec(H); 1 }
            0x26 => { self.registers.h = self.fetch_byte(); 2 }
            0x27 => { self.daa(); 1 }
            0x28 => { if self.registers.f.zero { self.jr(); 3 } else { 2 } }
            0x29 => { self.add_16(HL); 2 }
            0x2a => { self.registers.a = self.bus.read_byte(self.registers.hli()); 2 }
            0x2b => { self.dec_16(HL); 2 }
            0x2c => { self.inc(L); 1 }
            0x2d => { self.dec(L); 1 }
            0x2e => { self.registers.l = self.fetch_byte(); 2 }
            0x2f => { self.cpl(); 1 }
            0x30 => { if !self.registers.f.carry { self.jr(); 3 } else { 2 } }
            0x31 => { self.registers.sp = self.fetch_word(); 3 }
            0x32 => { self.bus.write_byte(self.registers.hld(), self.registers.a); 2 }
            0x33 => { self.inc_16(SP); 2 }
            0x34 => { self.inc_addr(Address::HL); 3 }
            0x35 => { self.dec_addr(Address::HL); 3 }
            0x36 => { let d8 = self.fetch_byte(); self.bus.write_byte(self.registers.read_16(HL), d8); 3 }
            0x37 => { self.registers.f.scf(); 1 }
            0x38 => { if self.registers.f.carry { self.jr(); 3 } else { 2 } }
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
            _ => todo!("Instruksjonen er ikke støttet!")
        }
    }
}