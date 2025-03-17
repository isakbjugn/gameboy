use bitflags::bitflags;

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;
const VIDEO_RAM_SIZE: usize = 0x2000;
const OAM_SIZE: usize = 160;

bitflags!(
    pub struct Control: u8 {
        const lcd_ppu_enable = 0x8;
        const window_tile_map_area = 0x7;
        const window_enable = 0x8;
        const bg_window_tile_data_area = 0x5;
        const bg_tile_map_area = 0x4;
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
    frame_buffer: Vec<u8>,
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
    oam: [u8; OAM_SIZE],
    updated: bool,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            video_ram: [0; VIDEO_RAM_SIZE],
            frame_buffer: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
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
            oam: [0; OAM_SIZE],
            updated: false,
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
        match address {
            0x40 => self.control = Control::from_bits(value).unwrap(),
            0x41 => self.set_status(value),
            0x42 => self.vertical_scroll = value,
            0x43 => self.horizontal_scroll = value,
            0x44 => self.scanline = value,
            0x45 => self.scanline_compare = value,
            0x47 => self.bg_palette = value,
            0x48 => self.obj_palette_0 = value,
            0x49 => self.obj_palette_1 = value,
            0x4a => self.window_y_position = value,
            0x4b => self.window_x_position = value,
            _ => unreachable!()
        }
    }
    fn set_status(&mut self, value: u8) {
        self.status = Status::from_bits(value).unwrap();
        self.mode = match value & 0x3 {
            0x0 => Mode::HorizontalBlank,
            0x1 => Mode::VerticalBlank,
            0x2 => Mode::Drawing,
            0x3 => Mode::OAMScan,
            _ => unreachable!()
        }
    }
    pub fn read_oam(&self, address: u16) -> u8 {
        match self.mode {
            Mode::Drawing | Mode::OAMScan => 0xff,
            _ => self.oam[address as usize - 0xfe00]
        }
    }
    pub fn write_oam(&mut self, address: u16, value: u8) {
        match self.mode {
            Mode::Drawing | Mode::OAMScan => (),
            _ => self.oam[address as usize - 0xfe00] = value,
        }
    }
    pub fn dma_write_oam(&mut self, address: u16, sprite: u8) {
        self.oam[address as usize] = sprite;
    }
    pub fn check_and_reset_updated(&mut self) -> bool {
        let result = self.updated;
        self.updated = false;
        result
    }
    pub fn read_frame_buffer(&mut self) -> &[u8] {
        &self.frame_buffer
    }
}