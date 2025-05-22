use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use crate::mbc::MBC;

pub struct MBC3 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_banks: usize,
    ram_banks: usize,
    ram_enable: bool,
    rom_bank_number: usize,
    ram_bank_number: usize,
    has_battery: bool,
    battery_save_path: Option<PathBuf>,
}

impl MBC3 {
    pub fn new(data: Vec<u8>, battery_save_path: Option<PathBuf>) -> Self {
        let rom_banks = 64;
        let ram_banks = 4;
        let ram_size = ram_banks * 0x2000;
        let has_battery = data[0x147] == 0x10 || data[0x147] == 0x13;
        
        Self {
            rom: data,
            ram: {
                let mut ram = vec![0; ram_size];
                if has_battery && battery_save_path.is_some() {
                    if let Ok(mut file) = File::open(battery_save_path.as_ref().unwrap()) {
                        file.read_exact(&mut ram).expect("Failed to read battery data");
                    }
                }
                ram
            },
            rom_banks: rom_banks,
            ram_banks: ram_banks,
            ram_enable: false,
            rom_bank_number: 1,
            ram_bank_number: 0,
            has_battery: has_battery,
            battery_save_path: battery_save_path,            
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
                    _ => panic!("Invalid RAM bank number"),
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
        if self.has_battery && self.battery_save_path.is_some() {
            File::create(self.battery_save_path.as_ref().unwrap())
                .and_then(|mut file| file.write_all(&self.ram))
                .expect("Failed to save battery data");
        }
    }
}