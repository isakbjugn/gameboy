use crate::mbc::MBC;
use crate::ppu::PPU;

const WORK_RAM_SIZE: usize = 0x8000;
const HIGH_RAM_SIZE: usize = 0x7f;

pub struct MemoryBus {
    memory: [u8; 65536], // fra 0x0000 til 0xFFFF
    mbc: Box<dyn MBC>,
    ppu: PPU,
    work_ram: [u8; WORK_RAM_SIZE],
    high_ram: [u8; HIGH_RAM_SIZE],
    interrupt: u8,
}

impl MemoryBus {
    pub fn new(cart: Box<dyn MBC>) -> Self {
        Self {
            memory: [0; 65536],
            mbc: cart,
            ppu: PPU::new(),
            work_ram: [0; WORK_RAM_SIZE],
            high_ram: [0; HIGH_RAM_SIZE],
            interrupt: 0,
        }
    }
    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000 ..= 0x7fff => self.mbc.read_rom(address),
            0x8000 ..= 0x9fff => self.ppu.read_byte(address),
            0xa000 ..= 0xbfff => todo!("Have not implemented extra RAM at 0xA000 to 0xBFFF"),
            0xc000 ..= 0xcfff | 0xe000 ..= 0xefff => self.work_ram[address as usize & 0x1fff],
            0xd000 ..= 0xdfff | 0xf000 ..= 0xfdff => self.work_ram[address as usize & 0x1fff],
            0xfe00 ..= 0xfe9f => self.ppu.read_oam(address),
            0xfea0 ..= 0xfeff => panic!("Not usable!"),
            0xff00 ..= 0xff7f => self.io_read_byte((address & 0x00ff) as u8),
            0xff80 ..= 0xfffe => self.high_ram[address as usize & 0x007F],
            0xffff => self.interrupt,
            _ => unreachable!()
        }
    }
    pub fn io_read_byte(&self, address: u8) -> u8 {
        todo!()
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