use crate::cpu::CPU;
use crate::registers::Reg8::{B, C, D, E};
use crate::registers::Reg16::{BC, DE};

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
            0x08 => { let address = self.fetch_word(); self.bus.write_word(address, self.sp); 5 }
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
            _ => todo!("Instruksjonen er ikke støttet!")
        }
    }
}