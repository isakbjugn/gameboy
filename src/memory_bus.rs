pub struct MemoryBus {
    memory: [u8; 65536] // fra 0x0000 til 0xFFFF
}

impl MemoryBus {
    pub fn new() -> Self {
        Self { memory: [0; 65536] }
    }
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    pub fn write_byte(&mut self, address: u16, byte: u8) {
        self.memory[address as usize] = byte;
    }
}