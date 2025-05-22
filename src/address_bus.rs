use log::debug;
use crate::apu::APU;
use crate::bootrom::Bootrom;
use crate::cartridge::Cartridge;
use crate::joypad::Joypad;
use crate::ppu::PPU;
use crate::timer::Timer;

const WORK_RAM_SIZE: usize = 0x8000;
const HIGH_RAM_SIZE: usize = 0x7f;

pub struct AddressBus {
    pub cartridge: Cartridge,
    pub ppu: PPU,
    apu: APU,
    work_ram: [u8; WORK_RAM_SIZE],
    high_ram: [u8; HIGH_RAM_SIZE],
    pub interrupt_enable_register: u8,
    pub interrupt_flag: u8,
    pub joypad: Joypad,
    bootrom: Bootrom,
    timer: Timer,
}

impl AddressBus {
    pub fn new(cart: Cartridge) -> Self {
        let mut address_bus = Self {
            cartridge: cart,
            ppu: PPU::new(),
            apu: APU::new(),
            work_ram: [0; WORK_RAM_SIZE],
            high_ram: [0; HIGH_RAM_SIZE],
            interrupt_enable_register: 0,
            interrupt_flag: 0,
            joypad: Joypad::new(),
            bootrom: Bootrom::new(),
            timer: Timer::new(),
        };
        #[cfg(feature = "test")] {
            address_bus.set_initial();
        }
        address_bus
    }
    pub fn cycle(&mut self, m_cycles: u32) -> u32 {
        self.timer.cycle(m_cycles);
        self.interrupt_flag |= self.timer.interrupt;
        self.timer.interrupt = 0;
        
        self.interrupt_flag |= self.joypad.interrupt;
        self.joypad.interrupt = 0;
        
        let t_cycles = 4 * m_cycles;
        self.ppu.cycle(t_cycles);
        self.interrupt_flag |= self.ppu.interrupt;
        self.ppu.interrupt = 0;
        t_cycles
    }
    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            //0x00ff => panic!("Siste instruksjon i bootrom"),
            0x0000 ..= 0x00ff if self.bootrom.is_active() => self.bootrom[address],
            0x0000 ..= 0x7fff => self.cartridge.mbc.read_rom(address),
            0x8000 ..= 0x9fff => self.ppu.read_video_ram(address),
            0xa000 ..= 0xbfff => self.cartridge.mbc.read_ram(address),
            0xc000 ..= 0xcfff | 0xe000 ..= 0xefff => self.work_ram[address as usize & 0x1fff],
            0xd000 ..= 0xdfff | 0xf000 ..= 0xfdff => self.work_ram[address as usize & 0x1fff],
            0xfe00 ..= 0xfe9f => self.ppu.read_oam(address),
            0xfea0 ..= 0xfeff => panic!("Not usable!"),
            0xff00 ..= 0xff7f => self.io_read_byte((address & 0x00ff) as u8),
            0xff80 ..= 0xfffe => self.high_ram[address as usize & 0x007F],
            0xffff => self.interrupt_enable_register,

            _ => { debug!("Minneadresse {:#04x} kan ikke leses fra. Returnerer 0xff.", address); 0xff }
        }
    }
    pub fn io_read_byte(&self, address: u8) -> u8 {
        match address {
            0x00 => self.joypad.read_byte(),
            0x01 ..= 0x02 => panic!("Serial transfer not implemented"),
            0x04 ..= 0x07 => self.timer.read_byte(address),
            0x0f => self.interrupt_flag,
            0x10 ..= 0x26 => self.apu.read_byte(address),
            0x30 ..= 0x3f => self.apu.read_wave_byte(address),
            0x40 ..= 0x4b => self.ppu.read_byte(address),
            0x4f => panic!("VRAM Bank Select is CGB feature"),
            0x50 => panic!("write-only"),
            0x51 ..= 0x70 => panic!("Game Boy Color feature"),
            _ => { debug!("IO-minneadresse {:#04x} kan ikke leses fra. Returnerer 0xff.", address); 0xff }
        }
    }
    pub fn write_byte(&mut self, address: u16, byte: u8) {
        match address {
            0x0000 ..= 0x00ff if self.bootrom.is_active() => (),
            0x0000 ..= 0x7fff => self.cartridge.mbc.write_rom(address, byte),
            0x8000 ..= 0x9fff => self.ppu.write_video_ram(address, byte),
            0xa000 ..= 0xbfff => self.cartridge.mbc.write_ram(address, byte),
            0xc000 ..= 0xcfff | 0xe000 ..= 0xefff => self.work_ram[address as usize & 0x1fff] = byte,
            0xd000 ..= 0xdfff | 0xf000 ..= 0xfdff => self.work_ram[address as usize & 0x1fff] = byte,
            0xfe00 ..= 0xfe9f => self.ppu.write_oam(address, byte),
            0xfea0 ..= 0xfeff => debug!("Not usable!"),
            0xff00 ..= 0xff7f => self.io_write_byte((address & 0x00ff) as u8, byte),
            0xff80 ..= 0xfffe => self.high_ram[address as usize & 0x007F] = byte,
            0xffff => self.interrupt_enable_register = byte,
            _ => debug!("Minneadresse {:#04x} kan ikke skrives til.", address)
        }
    }
    pub fn io_write_byte(&mut self, address: u8, byte: u8) {
        match address {
            0x00 => self.joypad.write_byte(byte),
            0x01 ..= 0x02 => self.write_serial(byte),
            0x04 ..= 0x07 => self.timer.write_byte(address, byte),
            0x0f => self.interrupt_flag = byte,
            0x10 ..= 0x26 => self.apu.write_byte(address, byte),
            0x30 ..= 0x3f => self.apu.write_wave_byte(address, byte),
            0x40 ..= 0x45 | 0x47 ..= 0x4b => self.ppu.write_byte(address, byte),
            0x46 => self.oam_dma(byte),
            0x4f => debug!("VRAM Bank Select is CGB feature"),
            0x50 => self.bootrom.deactivate(),
            0x51 ..= 0x70 => debug!("Game Boy Color feature ved IO-minneadresse 0xff{:02x}", address),
            _ => debug!("IO-minneadresse 0xff{:02x} kan ikke skrives til.", address)
        }
    }
    pub fn read_word(&self, address: u16) -> u16 {
        (self.read_byte(address) as u16) | ((self.read_byte(address + 1) as u16) << 8)
    }
    pub fn write_word(&mut self, address: u16, word: u16) {
        self.write_byte(address, (word & 0xff) as u8);
        self.write_byte(address + 1, (word >> 8) as u8);
    }
    fn write_serial(&self, byte: u8) {
        if cfg!(feature = "test") {
            if let Ok(s) = String::from_utf8(vec![byte]) { print!("{}", s); }
        } else {
            debug!("Serial transfer not implemented");
        }
    }
    pub fn oam_dma(&mut self, value: u8) {
        let base = (value as u16) << 8;
        for i in 0..0xa0 {
            let sprite = self.read_byte(base + i);
            self.ppu.dma_write_oam(i, sprite);
        }
    }
    #[cfg(feature = "test")]
    fn set_initial(&mut self) {
        self.write_byte(0xFF05, 0);
        self.write_byte(0xFF06, 0);
        self.write_byte(0xFF07, 0);
        self.write_byte(0xFF10, 0x80);
        self.write_byte(0xFF11, 0xBF);
        self.write_byte(0xFF12, 0xF3);
        self.write_byte(0xFF14, 0xBF);
        self.write_byte(0xFF16, 0x3F);
        self.write_byte(0xFF16, 0x3F);
        self.write_byte(0xFF17, 0);
        self.write_byte(0xFF19, 0xBF);
        self.write_byte(0xFF1A, 0x7F);
        self.write_byte(0xFF1B, 0xFF);
        self.write_byte(0xFF1C, 0x9F);
        self.write_byte(0xFF1E, 0xFF);
        self.write_byte(0xFF20, 0xFF);
        self.write_byte(0xFF21, 0);
        self.write_byte(0xFF22, 0);
        self.write_byte(0xFF23, 0xBF);
        self.write_byte(0xFF24, 0x77);
        self.write_byte(0xFF25, 0xF3);
        self.write_byte(0xFF26, 0xF1);
        self.write_byte(0xFF40, 0x91);
        self.write_byte(0xFF42, 0);
        self.write_byte(0xFF43, 0);
        self.write_byte(0xFF45, 0);
        self.write_byte(0xFF47, 0xFC);
        self.write_byte(0xFF48, 0xFF);
        self.write_byte(0xFF49, 0xFF);
        self.write_byte(0xFF4A, 0);
        self.write_byte(0xFF4B, 0);
    }
}