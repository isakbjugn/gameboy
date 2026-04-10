use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use gameboy_core::battery_save::BatterySave;

pub struct FileBatterySave {
    battery_save_path: PathBuf
}

impl FileBatterySave {
    pub fn new(cartridge_path: PathBuf) -> Self {
        Self { battery_save_path: cartridge_path.with_extension("gbsave")}
    }
}

impl BatterySave for FileBatterySave {
    fn load(&self, ram: &mut [u8]) {
        if let Ok(mut file) = File::open(self.battery_save_path.clone()) {
            file.read_exact(ram).expect("Failed to read battery data")
        }
    }

    fn save(&self, data: &[u8]) {
        File::create(self.battery_save_path.clone())
            .and_then(|mut file| file.write_all(data))
            .expect("Failed to save battery data");
    }
}