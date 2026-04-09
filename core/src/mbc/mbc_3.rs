use crate::battery_save::BatterySave;
use crate::mbc::MBC;

pub struct MBC3 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_banks: usize,
    ram_banks: usize,
    ram_enable: bool,
    rom_bank_number: usize,
    ram_bank_number: usize,
    battery_save: Option<Box<dyn BatterySave>>,
}

impl MBC3 {
    pub fn new(data: Vec<u8>, battery_save: Option<Box<dyn BatterySave>>) -> Self {
        let rom_banks = 64;
        let ram_banks = 4;
        let ram_size = ram_banks * 0x2000;
        let has_battery = data[0x147] == 0x10 || data[0x147] == 0x13;
        
        Self {
            rom: data,
            ram: {
                let mut ram = vec![0; ram_size];
                if has_battery && let Some(ref battery_save) = battery_save {
                    battery_save.load(&mut ram);
                }
                ram
            },
            rom_banks: rom_banks,
            ram_banks: ram_banks,
            ram_enable: false,
            rom_bank_number: 1,
            ram_bank_number: 0,
            battery_save: battery_save,
        }
    }
}

impl MBC for MBC3 {
    fn read_rom(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3fff => self.rom[address as usize],
            0x4000..=0x7fff => self.rom[0x4000 * self.rom_bank_number + (address as usize - 0x4000)],
            _ => panic!("Invalid ROM address"),
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enable { return 0xff };
        
        match address {
            0xa000..=0xbfff => self.ram[0x2000 * self.ram_bank_number + (address as usize - 0xa000)],
            _ => panic!("Invalid RAM address"),
        }
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0x000..=0x1fff => self.ram_enable = value & 0x0a == 0x0a,
            0x2000..=0x3fff => {
                let bank_number = value & 0x3f;
                if bank_number == 0 {
                    self.rom_bank_number = 1;
                } else {
                    self.rom_bank_number = bank_number as usize;
                }
            }
            0x4000..=0x5fff => {
                match value {
                    0x00..=0x03 => self.ram_bank_number = value as usize,
                    0x04..=0x07 => panic!("Invalid ROM address"),
                    0x08..=0x0c => {} // do nothing, does not support RTC at the moment
                    _ => {} // invalid RAM bank number, but no worries
                }
            }
            0x6000..=0x7fff => {} // RTC Data Latch, RTC not implemented
            _ => panic!("Invalid ROM address"),
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enable { return }
        
        match address {
            0xa000..=0xbfff => self.ram[0x2000 * self.ram_bank_number + (address as usize - 0xa000)] = value,
            _ => panic!("Invalid RAM address"),
        }
    }
}

impl Drop for MBC3 {
    fn drop(&mut self) {
        if let Some(ref battery_save) = self.battery_save {
            battery_save.save(&self.ram)
        }
    }
}