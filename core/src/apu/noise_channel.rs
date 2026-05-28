
#[derive(Default)]
pub struct NoiseChannel;

impl NoiseChannel {
    pub fn read_byte(&self, address: u8) -> u8 {
        0
    }
    pub fn write_byte(&self, address: u8, value: u8) {

    }
}
