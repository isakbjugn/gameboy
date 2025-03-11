use crate::cpu::CPU;
use crate::registers::Reg16::BC;

impl CPU {
    pub fn call(&mut self) -> u32 {
        let opcode = self.fetch_byte();
        match opcode {
            0x00 => { 1 },
            0x01 => { let word = self.fetch_word(); self.registers.write_16(BC, word); 3 }
            0x02 => { self.bus.write_byte(self.registers.read_16(BC), self.registers.a); 2 }
            0x03 => { self.inc_16(BC); 2 }
            _ => todo!("Instruksjonen er ikke støttet!")
        }
    }
}