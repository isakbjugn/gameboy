use bitflags::bitflags;

const VIDEO_RAM_SIZE: usize = 0x2000;

bitflags!(
    pub struct Control: u8 {
        const lcd_ppu_enable = 0x8;
        const window_tile_map_area = 0x7;
        const window_enable = 0x8;
        const bg_window_tile_data_area = 0x5;
        const bg_tile_map_area = 0x43;
        const obj_size = 0x3;
        const obj_enable = 0x2;
        const bg_window_enable = 0x1;
    }
);

bitflags!(
    struct Status: u8 {
        const lyc_select = 0x7;
        const mode_2_int_select = 0x6;
        const mode_1_int_select = 0x5;
        const mode_0_int_select = 0x4;
        const lyc_equals_ly = 0x3;
    }
);

pub enum Mode {
    HorizontalBlank,
    VerticalBlank,
    Drawing,
    OAMScan,
}

impl Mode {
    pub fn bits(&self) -> u8 {
        match self {
            Mode::HorizontalBlank => 0,
            Mode::VerticalBlank => 1,
            Mode::Drawing => 2,
            Mode::OAMScan => 3,
        }
    }
}

pub struct PPU {
    pub video_ram: [u8; VIDEO_RAM_SIZE],
    control: Control,
    status: Status,
    mode: Mode,
    vertical_scroll: u8,
    horizontal_scroll: u8,
    scanline: u8,
    scanline_compare: u8,
    bg_palette: u8,
    obj_palette_0: u8,
    obj_palette_1: u8,
    window_y_position: u8,
    window_x_position: u8,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            video_ram: [0; VIDEO_RAM_SIZE],
            control: Control::from_bits(0).unwrap(),
            status: Status::from_bits(0).unwrap(),
            mode: Mode::OAMScan,
            vertical_scroll: 0,
            horizontal_scroll: 0,
            scanline: 0,
            scanline_compare: 0,
            bg_palette: 0,
            obj_palette_0: 0,
            obj_palette_1: 0,
            window_y_position: 0,
            window_x_position: 0,
        }
    }
    pub fn read_byte(&self, address: u8) -> u8 {
        match address {
            0x40 => self.control.bits(),
            0x41 => self.status.bits() | self.mode.bits(),
            0x42 => self.vertical_scroll,
            0x43 => self.horizontal_scroll,
            0x44 => self.scanline,
            0x45 => self.scanline_compare,
            0x46 => 0, // write-only
            0x47 => self.bg_palette,
            0x48 => self.obj_palette_0,
            0x49 => self.obj_palette_1,
            0x4a => self.window_y_position,
            0x4b => self.window_x_position,
            _ => unreachable!()
        }
    }
    pub fn write_byte(&mut self, address: u8, value: u8) {
        todo!()
    }
    pub fn read_oam(&self, address: u16) -> u8 {
        todo!()
    }
    pub fn write_oam(&self, address: u16, value: u8) {
        todo!()
    }
}