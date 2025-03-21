mod decode;
mod execute;
mod flags_register;
mod registers;
mod interrupt_master_enable;

use crate::cartridge::Cartridge;
use crate::address_bus::AddressBus;
use registers::Registers;
use crate::cpu::interrupt_master_enable::InterruptMasterEnable;

pub struct CPU {
    registers: Registers,
    pub bus: AddressBus,
    is_halted: bool,
    interrupt_master_enable: InterruptMasterEnable,
}

impl CPU {
    pub fn new(cartridge_name: &str) -> Result<Self, &'static str> {
        let cartridge = Cartridge::from_path(format!("roms/{}", cartridge_name).into())?;
        
        Ok(Self {
            registers: Registers::new(),
            bus: AddressBus::new(cartridge),
            is_halted: false,
            interrupt_master_enable: InterruptMasterEnable::new(),
        })
    }
    pub fn cycle(&mut self) -> u32 {
        let m_cycles = self.call();
        self.bus.cycle(m_cycles * 4);
        m_cycles
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
    fn pop_sp(&mut self) -> u16 {
        let lower_byte = self.bus.read_byte(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);
        let upper_byte = self.bus.read_byte(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);
        u16::from_le_bytes([lower_byte, upper_byte])
    }
}
