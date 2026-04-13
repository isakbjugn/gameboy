use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use gameboy_core::game_pak::GamePak;

pub struct FileGamePak {
    data: Vec<u8>,
}

impl FileGamePak {
    pub fn new(cartridge_path: PathBuf) -> Self {
        let mut cartridge_data = vec![];
        File::open(&cartridge_path).and_then(|mut f| f.read_to_end(&mut cartridge_data)).expect("Could not read ROM");
        Self { data: cartridge_data }
    }
}

impl GamePak for FileGamePak {
    fn read_rom(&self) -> Vec<u8> {
        self.data.clone()
    }
}
