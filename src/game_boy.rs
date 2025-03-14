use crate::cpu::CPU;

pub struct GameBoy {
    cpu: CPU,
}

impl GameBoy {
    pub fn new(cartridge_name: &str) -> Result<Self, &'static str> {
        Ok(Self {
            cpu: CPU::new(cartridge_name)?,
        })
    }
    pub fn emulate(&mut self) {
        
    }
    pub fn title(&self) -> String {
        self.cpu.bus.cartridge.title()
    }
}