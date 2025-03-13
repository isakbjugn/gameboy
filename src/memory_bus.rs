use crate::bootrom::Bootrom;
use crate::joypad::Joypad;
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
    interrupt_enable_register: u8,
    interrupt_flag: u8,
    joypad: Joypad,
    bootrom: Bootrom,
}

impl MemoryBus {
    pub fn new(cart: Box<dyn MBC>) -> Self {
        Self {
            memory: [0; 65536],
            mbc: cart,
            ppu: PPU::new(),
            work_ram: [0; WORK_RAM_SIZE],
            high_ram: [0; HIGH_RAM_SIZE],
            interrupt_enable_register: 0,
            interrupt_flag: 0,
            joypad: Joypad::new(),
            bootrom: Bootrom::new(),
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
            0xffff => self.interrupt_enable_register,
            _ => unreachable!()
        }
    }
    pub fn io_read_byte(&self, address: u8) -> u8 {
        match address {
            0x00 => self.joypad.read_byte(),
            0x01 ..= 0x02 => panic!("Serial transfer not implemented"),
            0x04 ..= 0x07 => todo!("Implement timer and divider"),
            0x0f => self.interrupt_flag,
            0x10 ..= 0x26 => panic!("Audio not implemented"),
            0x30 ..= 0x3f => panic!("Wave pattern not implemented"),
            0x40 ..= 0x4b => todo!("Implement LCD Control"),
            0x4f => panic!("VRAM Bank Select is CGB feature"),
            _ => unreachable!("Game Boy Color feature")
        }
    }
    pub fn write_byte(&mut self, address: u16, byte: u8) {
        match address {
            0x0000 if self.bootrom.is_active() => (),
            0x0000 ..= 0x7fff => panic!("MBC0 is read-only"),
            0x8000 ..= 0x9fff => self.ppu.write_byte(address, byte),
            0xa000 ..= 0xbfff => panic!("MBC0 is read-only"),
            0xc000 ..= 0xcfff | 0xe000 ..= 0xefff => self.work_ram[address as usize & 0x1fff] = byte,
            0xd000 ..= 0xdfff | 0xf000 ..= 0xfdff => self.work_ram[address as usize & 0x1fff] = byte,
            0xfe00 ..= 0xfe9f => self.ppu.write_oam(address, byte),
            0xfea0 ..= 0xfeff => panic!("Not usable!"),
            0xff00 ..= 0xff7f => self.io_write_byte((address & 0x00ff) as u8, byte),
            0xff80 ..= 0xfffe => self.high_ram[address as usize & 0x007F] = byte,
            0xffff => self.interrupt_enable_register = byte,
            _ => unreachable!()
        }
    }
    pub fn io_write_byte(&mut self, address: u8, byte: u8) {
        match address {
            0x00 => self.joypad.write_byte(byte),
            0x01 ..= 0x02 => panic!("Serial transfer not implemented"),
            0x04 ..= 0x07 => todo!("Implement timer and divider"),
            0x0f => self.interrupt_flag = byte,
            0x10 ..= 0x26 => panic!("Audio not implemented"),
            0x30 ..= 0x3f => panic!("Wave pattern not implemented"),
            0x40 ..= 0x4b => todo!("Implement LCD Control"),
            0x4f => panic!("VRAM Bank Select is CGB feature"),
            _ => unreachable!("Game Boy Color feature")
        }
    }
    pub fn read_word(&self, address: u16) -> u16 {
        (self.read_byte(address) as u16) | ((self.read_byte(address + 1) as u16) << 8)
    }
    pub fn write_word(&mut self, address: u16, word: u16) {
        self.write_byte(address, (word & 0xff) as u8);
        self.write_byte(address + 1, (word >> 8) as u8);
    }
}