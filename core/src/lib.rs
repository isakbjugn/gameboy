pub mod cpu;
pub mod address_bus;
pub mod mbc;
pub mod ppu;
pub mod joypad;
pub mod bootrom;
pub mod timer;
pub mod game_boy;
pub mod cartridge;
pub mod frame_buffer;
pub mod apu;
pub mod battery_save;

pub const SCREEN_WIDTH: u32 = 160;
pub const SCREEN_HEIGHT: u32 = 144;
