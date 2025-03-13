
pub trait MBC {
    fn read_rom(&self, address: u16) -> u8;
}