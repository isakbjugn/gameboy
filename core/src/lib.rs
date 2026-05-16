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
pub const CPU_CLOCK_SPEED: u32 = 4_194_304;
pub const CPU_CYCLES_PER_FRAME: u32 = 70224;
pub const NANOSECONDS_PER_FRAME: u64 = 16_742_006;

#[cfg(feature = "sound")]
pub const AUDIO_SAMPLE_RATE: u32 = 48_000;
