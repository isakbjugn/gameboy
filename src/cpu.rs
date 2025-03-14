mod decode;
mod execute;

use crate::cartridge::Cartridge;
use crate::memory_bus::MemoryBus;
use crate::registers::Registers;

pub struct CPU {
    registers: Registers,
    pc: u16,
    pub bus: MemoryBus,
    is_halted: bool,
}

impl CPU {
    pub fn new(cartridge_name: &str) -> Result<Self, &'static str> {
        let cartridge = Cartridge::from_path(format!("roms/{}", cartridge_name).into())?;
        
        Ok(Self {
            registers: Registers::new(),
            pc: 0,
            bus: MemoryBus::new(cartridge),
            is_halted: false,
        })
    }
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
