use crate::cpu::CPU;
use crate::registers::Reg8::{B, C};
use crate::registers::Reg16::BC;

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
            0x06 => { let byte = self.fetch_byte(); self.registers.write_8(B, byte); 2 }
            0x07 => { self.rlca(); 1 }
            0x08 => { let address = self.fetch_word(); self.bus.write_word(address, self.sp); 5 }
            0x09 => { self.add_16(BC); 2 }
            0x0a => { self.registers.a = self.bus.read_byte(self.registers.read_16(BC)); 2 }
            0x0b => { self.dec_16(BC); 2 }
            0x0c => { self.inc(C); 1 }
            0x0d => { self.dec(C); 1 }
            0x0e => { let byte = self.fetch_byte(); self.registers.c = byte; 2 }
            _ => todo!("Instruksjonen er ikke støttet!")
        }
    }
}