use crate::battery_save::BatterySave;
use crate::cpu::CPU;
use crate::joypad::JoypadKey;

pub struct GameBoy {
    cpu: CPU,
}

impl GameBoy {
    pub fn new(cartridge_data: Vec<u8>, battery_save: Option<Box<dyn BatterySave>>) -> Result<Box<Self>, &'static str> {
        Ok(Box::new(Self {
            cpu: CPU::new(cartridge_data, battery_save)?,
        }))
    }
    pub fn emulate(&mut self) -> u32 {
        let m_cycles = self.cpu.cycle();
        self.cpu.bus.cycle(m_cycles)
    }
    pub fn updated_frame_buffer(&mut self) -> Option<Vec<u8>> {
        match self.cpu.bus.ppu.check_and_reset_updated() {
            true => Some(self.cpu.bus.ppu.read_frame_buffer().to_vec()),
            false => None
        }
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
    pub fn manual_save(&self) {
        self.cpu.bus.cartridge.manual_save()
    }
}