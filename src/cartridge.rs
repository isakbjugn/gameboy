use std::fs::File;
use std::io::Read;
use std::path;
use crate::mbc::{MBC, MBC0};

pub struct Cartridge {
    pub mbc: Box<dyn MBC>
}

impl Cartridge {
    pub fn from_path(cartridge_path: path::PathBuf) -> Result<Self, &'static str> {
        let mut data = vec![];
        File::open(&cartridge_path).and_then(|mut f| f.read_to_end(&mut data)).map_err(|_| "Could not read ROM")?;
        
        Ok(Self {
            mbc: Box::new(MBC0::new(data)),
        })
    }
}