
pub struct PPU;

impl PPU {
    pub fn new() -> Self { Self }
    pub fn read_byte(&self, address: u16) -> u8 {
        todo!()
    }
    pub fn write_byte(&mut self, address: u16, value: u8) {
        todo!()
    }
    pub fn read_oam(&self, address: u16) -> u8 {
        todo!()
    }
    pub fn write_oam(&self, address: u16, value: u8) {
        todo!()
    }
}