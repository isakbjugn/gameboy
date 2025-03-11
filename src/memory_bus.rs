pub struct MemoryBus {
    memory: [u8; 65536] // fra 0x0000 til 0xFFFF
}

impl MemoryBus {
    pub fn new() -> Self {
        Self { memory: [0; 65536] }
    }
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
        // todo: Det er en rekke lokasjoner i minnet, som koder for forskjellige enheter
        // f.eks. MBC, GPU, keypad, timer, lydenhet
    }
    pub fn write_byte(&mut self, address: u16, byte: u8) {
        self.memory[address as usize] = byte;
    }
    pub fn read_word(&self, address: u16) -> u16 {
        (self.read_byte(address) as u16) | ((self.read_byte(address + 1) as u16) << 8)
    }
    pub fn write_word(&mut self, address: u16, word: u16) {
        self.write_byte(address, (word & 0xff) as u8);
        self.write_byte(address + 1, (word >> 8) as u8);
    }
}