mod decode;
mod execute;

use crate::memory_bus::MemoryBus;
use crate::registers::Registers;

struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus,
    is_halted: bool,
}

impl CPU {
    fn cycle(&mut self) -> u32 {
        self.call()
    }
    fn fetch_byte(&mut self) -> u8 {
        let byte = self.bus.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }
    fn fetch_word(&mut self) -> u16 {
        let word = self.bus.read_word(self.pc);
        self.pc = self.pc.wrapping_add(2);
        word
    }
}
