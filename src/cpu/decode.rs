use crate::cpu::CPU;

impl CPU {
    pub fn call(&mut self) -> u32 {
        let opcode = self.fetch_byte();
        match opcode {
            0x00 => { 1 },
            0x01 => { let word = self.fetch_word(); self.registers.set_bc(word); 3 }
            0x02 => { self.bus.write_byte(self.registers.get_bc(), self.registers.a); 2 }
            _ => todo!("Instruksjonen er ikke støttet!")
        }
    }
}