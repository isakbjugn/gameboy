use crate::mbc::MBC;

enum BankingMode {
    Simple = 0,
    Advanced = 1,
}

pub struct MBC1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_banks: usize,
    ram_banks: usize,
    ram_enable: bool,
    rom_bank_number: usize,
    ram_bank_number: usize,
    banking_mode_select: BankingMode,
}

impl MBC1 {
    pub fn new(data: Vec<u8>) -> Self {
        let rom_banks = 32;
        let ram_banks = 16;
        let ram_size = ram_banks * 0x2000;

        Self {
            rom: data,
            ram: vec![0; ram_size],
            rom_banks: rom_banks,
            ram_banks: ram_banks,
            ram_enable: false,
            rom_bank_number: 1,
            ram_bank_number: 0,
            banking_mode_select: BankingMode::Simple,
        }
    }
}

impl MBC for MBC1 {
    fn read_rom(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3fff => match self.banking_mode_select {
                BankingMode::Simple => self.rom[address as usize],
                BankingMode::Advanced => {
                    let bank_number = self.rom_bank_number & 0b11100000;
                    self.rom[(bank_number * 0x4000) | address as usize]
                }
            },
            0x4000..=0x7fff => {
                let bank_number = self.rom_bank_number & 0b00011111;
                self.rom[(bank_number * 0x4000) | (address & 0x3fff) as usize]
            }
            _ => panic!("Invalid ROM address"),
        }
    }
    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enable { return 0xff; }

        let bank_number = match self.banking_mode_select {
            BankingMode::Simple => 0,
            BankingMode::Advanced => self.ram_bank_number & 0b00000011,
        };
        self.ram[(bank_number * 0x2000) | (address & 0x1fff) as usize]
    }
    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0x0000 ..= 0x1fff => self.ram_enable = matches!(value & 0x0f, 0x0a),
            0x2000 ..= 0x3fff => self.rom_bank_number = match value {
                0x00 => 0x01,
                _ => (value & 0b00011111) as usize,
            },
            0x4000 ..= 0x5fff => self.ram_bank_number = (value & 0b00000011) as usize,
            0x6000 ..= 0x7fff => {
                self.banking_mode_select = match value & 0b00000001 {
                    0 => BankingMode::Simple,
                    1 => BankingMode::Advanced,
                    _ => unreachable!()
                }
            }
            _ => panic!("Invalid ROM address"),
        }
    }
    fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enable { return }

        let bank_number = match self.banking_mode_select {
            BankingMode::Simple => 0,
            BankingMode::Advanced => self.ram_bank_number & 0b00000011,
        };
        self.ram[(bank_number * 0x2000) | (address & 0x1fff) as usize] = value;
    }
}
