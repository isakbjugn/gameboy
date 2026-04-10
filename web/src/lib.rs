use wasm_bindgen::prelude::*;

use gameboy_core::{SCREEN_HEIGHT, SCREEN_WIDTH};

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn inform() {
    alert(&format!("Game Boy-ens skjerm er {} px høy og {} px bred", SCREEN_HEIGHT, SCREEN_WIDTH));
}
