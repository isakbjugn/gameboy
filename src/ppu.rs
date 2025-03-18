use std::cmp::Ordering;
use arrayvec::ArrayVec;
use bitflags::bitflags;
use itertools::Itertools;

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;
const VIDEO_RAM_SIZE: usize = 0x2000;
const OAM_SIZE: usize = 160;
const SCANLINES: u8 = 154;

bitflags!(
    pub struct Control: u8 {
        const lcd_enable = 0x8;
        const window_tile_map_select = 0x7;
        const window_enable = 0x8;
        const tile_data_select = 0x5;
        const bg_tile_map_select = 0x4;
        const sprite_size = 0x3;
        const sprite_enable = 0x2;
        const bg_window_enable = 0x1;
    }
);

impl Control {
    fn lcd_on(&self) -> bool {
        self.bits() & 0x8 != 0
    }
    fn tall_sprite_mode(&self) -> bool {
        self.bits() & 0x4 != 0
    }
    fn sprite_height(&self) -> u8 {
        if self.tall_sprite_mode() { 16 } else { 8 }
    }
}

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

#[derive(Clone)]
pub struct Sprite {
    x: u8,
    tile_row: u8,
    sprite_number: u8,
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
    t_cycles: u32,
    sprite_buffer: ArrayVec<Sprite, 10>
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
            t_cycles: 0,
            sprite_buffer: ArrayVec::new(),
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
            0x40 => self.set_control(value),
            0x41 => self.set_status(value),
            0x42 => self.vertical_scroll = value,
            0x43 => self.horizontal_scroll = value,
            0x44 => panic!("scanline is read-only"),
            0x45 => self.scanline_compare = value,
            0x47 => self.bg_palette = value,
            0x48 => self.obj_palette_0 = value,
            0x49 => self.obj_palette_1 = value,
            0x4a => self.window_y_position = value,
            0x4b => self.window_x_position = value,
            _ => unreachable!()
        }
    }
    fn set_control(&mut self, value: u8) {
        let lcd_enable_initial_state = self.control.lcd_on();
        self.control = Control::from_bits(value).unwrap();
        if !self.control.lcd_on() && lcd_enable_initial_state {
            // reset PPU
            self.mode = Mode::OAMScan; // todo!("Dette er vel ikke riktig tilstand etter nullstilling?")
            self.scanline = 0;
            self.clear_display()
        }
    }
    fn clear_display(&mut self) {
        self.frame_buffer.iter_mut().for_each(|value| *value = 0xff);
        self.updated = true;
    }
    fn set_status(&mut self, value: u8) {
        self.status = Status::from_bits(value).unwrap();
        // self.mode is read-only
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
    pub fn cycle(&mut self, m_cycles: u32) {
        if !self.control.lcd_on() { return }

        self.t_cycles += m_cycles * 4;

        loop {
            match self.mode {
                Mode::OAMScan if self.t_cycles >= 80 => self.oam_scan(),
                Mode::Drawing if self.t_cycles >= 172 => self.draw(),
                Mode::HorizontalBlank if self.t_cycles >= 204 => self.horizontal_blank(),
                Mode::VerticalBlank if self.t_cycles >= 456 => self.vertical_blank(),
                _ => break,
            }
        }
    }
    fn oam_scan(&mut self) {
        self.sprite_buffer.clear();
        self.sprite_buffer = self.oam.chunks_exact(4)
            .filter_map(|sprite| match sprite {
                &[y, x, tile_index, flags] => {
                    let y = y.wrapping_sub(16);
                    let x = x.wrapping_sub(8);
                    if self.scanline.wrapping_sub(y) < self.control.sprite_height() {
                        Some(Sprite {
                            x: x,
                            tile_row: todo!(),
                            sprite_number: tile_index,
                        })
                    } else {
                        None
                    }
                }
                _ => None
            })
            .take(10)
            .enumerate()
            .sorted_by(Self::sprite_order())
            .map(|(_, sprite)| sprite)
            .collect();
        
        self.t_cycles -= 80;
        self.mode = Mode::Drawing
    }
    fn sprite_order() -> fn(&(usize, Sprite), &(usize, Sprite)) -> Ordering {
        |&(a_index, ref a), &(b_index, ref b)| match a.x.cmp(&b.x) {
            Ordering::Equal => b_index.cmp(&a_index),
            order => order.reverse()
        }
    }
    fn draw(&mut self) {
        //todo!("Write resulting pixels to frame buffer")
        self.t_cycles -= 172;
        self.mode = Mode::HorizontalBlank;
    }
    fn horizontal_blank(&mut self) {
        self.t_cycles -= 204;
        self.scanline += 1;
        self.mode = match self.scanline >= 144 {
            true => Mode::VerticalBlank,
            false => Mode::OAMScan,
        };
    }
    fn vertical_blank(&mut self) {
        self.t_cycles -= 456;
        self.scanline += 1;
        self.scanline %= SCANLINES;
        if self.scanline == 0 {
            self.mode = Mode::OAMScan;
        }
    }
}