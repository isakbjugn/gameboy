use bitflags::bitflags;

bitflags!(
    pub struct Control: u8 {
        const lcd_enable = 1 << 7;
        const window_tile_map_select = 1 << 6;
        const window_enable = 1 << 5;
        const tile_data_select = 1 << 4;
        const bg_tile_map_select = 1 << 3;
        const sprite_size = 1 << 2;
        const sprite_enable = 1 << 1;
        const bg_window_enable = 1;
    }
);

impl Control {
    pub fn lcd_on(&self) -> bool {
        self.bits() >> 7 != 0
    }
    fn tall_sprite_mode(&self) -> bool {
        self.contains(Control::sprite_size)
    }
    pub fn sprite_height(&self) -> u8 {
        if self.tall_sprite_mode() { 16 } else { 8 }
    }
    pub fn bg_map_mask(&self) -> usize {
        match self.contains(Control::bg_tile_map_select) {
            true => 0x1c00,
            false => 0x1800,
        }
    }
    pub fn window_map_mask(&self) -> usize {
        match self.contains(Control::window_tile_map_select) {
            true => 0x1c00,
            false => 0x1800,
        }
    }
    pub fn tile_data_base_from_tile_number(&self, tile_number: u8) -> usize {
        match self.contains(Control::tile_data_select) {
            true => tile_number as usize * 16,
            false => 0x1000u16.wrapping_add_signed(tile_number as i8 as i16 * 16) as usize,
        }
    }
}