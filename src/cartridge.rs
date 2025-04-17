use std::fs::File;
use std::io::Read;
use std::path;
use crate::mbc::MBC;
use crate::mbc::mbc_0::MBC0;
use crate::mbc::mbc_1::MBC1;

pub struct Cartridge {
    header: Vec<u8>,
    pub mbc: Box<dyn MBC>,
}

impl Cartridge {
    pub fn from_path(cartridge_path: path::PathBuf) -> Result<Self, &'static str> {
        let mut data = vec![];
        File::open(&cartridge_path).and_then(|mut f| f.read_to_end(&mut data)).map_err(|_| "Could not read ROM")?;
        let mut header = vec![0; 0x14f - 0x100 + 1];
        header.copy_from_slice(&data[0x0100..=0x014f]);
        
        Ok(Self {
            header,
            mbc: match (data[0x147], data[0x148], data[0x149]) {
                _ if cfg!(feature = "test") => Box::new(MBC0::new(data)),
                (0x00, ..) => Box::new(MBC0::new(data)),
                (0x03, 0x04, 0x02) => Box::new(MBC1::new(data, Some(cartridge_path.with_extension("gbsave")))),
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
}