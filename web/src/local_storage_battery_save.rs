use base64::prelude::*;
use log::error;
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
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

        match local_storage.get_item(&self.local_storage_key) {
            Ok(Some(value)) => {
                let decoded = BASE64_STANDARD.decode(value).unwrap();
                ram.copy_from_slice(&decoded);
            }
            Ok(None) => {}
            Err(error) => {
                error!("Klarte ikke å lese lagret spill fra LocalStorage: {:?}", error);
            }
        }
    }

    fn save(&self, data: &[u8]) {
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        let encoded = BASE64_STANDARD.encode(data);
        if let Err(error) = local_storage.set_item(&self.local_storage_key, &encoded) {
            error!("Klarte ikke å lagre spill til LocalStorage: {:?}", error)
        };
    }
}
