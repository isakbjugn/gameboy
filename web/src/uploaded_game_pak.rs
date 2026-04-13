use gameboy_core::game_pak::GamePak;

pub struct UploadedGamePak {
    data: Vec<u8>,
}

impl UploadedGamePak {
    pub fn new() -> Self {
        let data = include_bytes!("../../roms/links_awakening.gb");
        Self { data: Vec::from(data) }
    }
}

impl GamePak for UploadedGamePak {
    fn read_rom(&self) -> Vec<u8> {
        self.data.clone()
    }
}
