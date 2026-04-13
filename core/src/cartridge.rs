use crate::battery_save::BatterySave;
use crate::game_pak::GamePak;
use crate::mbc::MBC;
use crate::mbc::mbc_0::MBC0;
use crate::mbc::mbc_1::MBC1;
use crate::mbc::mbc_3::MBC3;

pub struct Cartridge {
    header: Vec<u8>,
    pub mbc: Box<dyn MBC>,
}

impl Cartridge {
    pub fn from_game_pak(game_pak: Box<dyn GamePak>, battery_save: Option<Box<dyn BatterySave>>) -> Result<Self, &'static str> {
        let data = game_pak.read_rom();
        let header = game_pak.read_header();
        
        Ok(Self {
            header,
            mbc: match (data[0x147], data[0x148], data[0x149]) {
                _ if cfg!(feature = "test") => Box::new(MBC0::new(data)),
                (0x00, ..) => Box::new(MBC0::new(data)),
                (0x03, 0x04, 0x02) => Box::new(MBC1::new(data, battery_save)),
                (0x13, 0x05, 0x03) => Box::new(MBC3::new(data, battery_save)),
                (mbc, rom_size, ram_size) => {
                    panic!("Støtter ikke denne MBC-en:\nMBC: {:#04x}\nROM size: {:#04x}\nRAM size: {:#04x}", mbc, rom_size, ram_size)
                },
            }
        })
    }
    pub fn title(&self) -> String {
        const TITLE_START: usize = 0x0134 - 0x0100;
        const TITLE_END: usize = 0x0143 - 0x0100;
        
        String::from_utf8(self.header[TITLE_START..=TITLE_END].to_owned()).unwrap()
    }
    pub fn manual_save(&self) {
        self.mbc.manual_save()
    }
}