mod decode;
mod execute;
mod flags_register;
mod registers;

use crate::cartridge::Cartridge;
use crate::address_bus::AddressBus;
use registers::Registers;

pub struct CPU {
    registers: Registers,
    pub bus: AddressBus,
    is_halted: bool,
}

impl CPU {
    pub fn new(cartridge_name: &str) -> Result<Self, &'static str> {
        let cartridge = Cartridge::from_path(format!("roms/{}", cartridge_name).into())?;
        
        Ok(Self {
            registers: Registers::new(),
            bus: AddressBus::new(cartridge),
            is_halted: false,
        })
    }
    fn cycle(&mut self) -> u32 {
        self.call()
    }
    fn fetch_byte(&mut self) -> u8 {
        let byte = self.bus.read_byte(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        byte
    }
    fn fetch_word(&mut self) -> u16 {
        let word = self.bus.read_word(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);
        word
    }
}
