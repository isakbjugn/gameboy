pub struct MemoryBus {
    memory: [u8; 0xFFFF]
}

impl MemoryBus {
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    pub fn write_byte(&mut self, address: u16, byte: u8) {

    }
}