use crate::cpu::CPU;
use crate::joypad::JoypadKey;

pub struct GameBoy {
    cpu: CPU,
}

impl GameBoy {
    pub fn new(cartridge_name: &str) -> Result<Box<Self>, &'static str> {
        Ok(Box::new(Self {
            cpu: CPU::new(cartridge_name)?,
        }))
    }
    pub fn emulate(&mut self) {
        
    }
    pub fn title(&self) -> String {
        self.cpu.bus.cartridge.title()
    }
    pub fn key_down(&mut self, key: JoypadKey) {
        self.cpu.bus.joypad.key_down(key)
    }
    pub fn key_up(&mut self, key: JoypadKey) {
        self.cpu.bus.joypad.key_up(key)
    }
}