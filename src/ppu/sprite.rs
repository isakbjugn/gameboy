use std::cmp::Ordering;
use bitflags::bitflags;

bitflags! {
    #[derive(Clone)]
    pub struct SpriteFlags: u8 {
        const obj_to_bg_priority = 1 << 7;
        const y_flip = 1 << 6;
        const x_flip = 1 << 5;
        const palette = 1 << 4;
    }
}

#[derive(Clone)]
pub struct Sprite {
    pub y: u8,
    pub x: u8,
    pub tile_index: u8,
    pub flags: SpriteFlags,
}