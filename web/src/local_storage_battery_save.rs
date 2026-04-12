use log::{error, info};
use gameboy_core::battery_save::BatterySave;

pub struct LocalStorageBatterySave {
    local_storage_key: String,
}

impl LocalStorageBatterySave {
    pub fn new(game_title: &str) -> Self {
        Self { local_storage_key: String::from(game_title) }
    }
}

impl BatterySave for LocalStorageBatterySave {
    fn load(&self, ram: &mut [u8]) {
        info!("Laster lagret spill fra LocalStorage (nøkkel: {})", self.local_storage_key);
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

        match local_storage.get_item(&self.local_storage_key) {
            Ok(Some(value)) => {
                info!("Fant lagret data ({} bytes)", value.len());
                ram.copy_from_slice(value.as_bytes());
            }
            Ok(None) => {
                info!("Ingen lagret data funnet for nøkkel: {}", self.local_storage_key);
            }
            Err(err) => {
                error!("Kunne ikke lese fra LocalStorage: {:?}", err);
            }
        }
    }

    fn save(&self, data: &[u8]) {
        info!("Lagrer spill til LocalStorage (nøkkel: {})", self.local_storage_key);
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        local_storage.set_item(&self.local_storage_key, &String::from_utf8(Vec::from(data)).unwrap()).expect("Klarte ikke å lagre spill til LocalStorage");
    }
}
