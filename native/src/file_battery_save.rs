use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use gameboy_core::battery_save::BatterySave;

struct FileBatterySave {
    battery_save_path: PathBuf
}

impl BatterySave for FileBatterySave {
    fn load(&self, ram: &mut [u8]) {
        if let Ok(mut file) = File::open(self.battery_save_path.as_ref().unwrap()) {
            file.read_exact(ram).expect("Failed to read battery data")
        }
    }

    fn save(&self, data: &[u8]) {
        File::create(self.battery_save_path.as_ref().unwrap())
            .and_then(|mut file| file.write_all(data))
            .expect("Failed to save battery data");
    }
}