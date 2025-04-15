mod control;
mod status;
mod mode;
mod sprite;

use std::cmp::Ordering;
use std::collections::HashMap;
use arrayvec::ArrayVec;
use itertools::Itertools;
use log::debug;
use crate::ppu::control::Control;
use crate::ppu::mode::Mode;
use crate::ppu::sprite::{Sprite, SpriteFlags};
use crate::ppu::status::Status;

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;
const VIDEO_RAM_SIZE: usize = 0x2000;
const OAM_SIZE: usize = 160;
const SCANLINES: u8 = 154;

pub struct Pixel {
    color: u8,
    palette: u8,
    background_priority: bool,
}

pub struct PPU {
    video_ram: [u8; VIDEO_RAM_SIZE],
    frame_buffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],
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
    sprite_buffer: ArrayVec<Sprite, 10>,
    pub interrupt: u8,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            video_ram: [0; VIDEO_RAM_SIZE],
            frame_buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
            control: Control::from_bits_truncate(0),
            status: Status::from_bits_truncate(0),
            mode: Mode::HorizontalBlank,
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
            interrupt: 0,
        }
    }
    pub fn read_byte(&self, address: u8) -> u8 {
        match address {
            0x40 => self.control.bits(),
            0x41 => self.read_status(),
            0x42 => self.vertical_scroll,
            0x43 => self.horizontal_scroll,
            0x44 => self.scanline, // 0x90 for å kjøre Blargg-tester uten skjerm
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
            0x44 => debug!("scanline is read-only"),
            0x45 => { self.scanline_compare = value; self.check_scanline_interrupt() },
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
            self.mode = Mode::HorizontalBlank;
            self.scanline = 0;
            self.clear_display()
        }
    }
    fn clear_display(&mut self) {
        self.frame_buffer.iter_mut().for_each(|value| *value = 0xff);
        self.updated = true;
    }
    fn read_status(&self) -> u8 {
        self.status.bits()
            | if self.scanline == self.scanline_compare { 1 << 2 } else { 0 }
            | self.mode.bits()
    }
    fn set_status(&mut self, value: u8) {
        self.status = Status::from_bits_truncate(value & 0b01111000);
        // self.lyc_equals_ly is read-only
        // self.mode is read-only
    }
    pub fn read_video_ram(&self, address: u16) -> u8 {
        match self.mode {
            Mode::Drawing => 0xff,
            _ => self.video_ram[address as usize & 0x1FFF]
        }
    }
    pub fn write_video_ram(&mut self, address: u16, value: u8) {
        match self.mode {
            Mode::Drawing => (),
            _ => self.video_ram[address as usize & 0x1FFF] = value,
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
    pub fn cycle(&mut self, t_cycles: u32) {
        if !self.control.lcd_on() { return }
        
        self.t_cycles += t_cycles;

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
                        // todo!("Tror egentlig ikke vi skal lagre Sprite slik de lagres i OAM, men heller x, tile_row og sprite_number")
                        Some(Sprite {
                            y,
                            x,
                            tile_index,
                            flags: SpriteFlags::from_bits(flags).unwrap(),
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
        let line_start = SCREEN_WIDTH * self.scanline as usize;
        let line_end = line_start + SCREEN_WIDTH;
        let mut pixels = [0; SCREEN_WIDTH];
        let mut bg_priority = [false; SCREEN_WIDTH];

        if self.control.contains(Control::bg_window_enable) {
            self.fetch_background_pixels().into_iter()
                .for_each(|(key, Pixel { color, palette, background_priority })| {
                    pixels[key] = self.color_from_palette(color, palette);
                    bg_priority[key] = background_priority;
                })
        }
        if self.control.contains(Control::window_enable) && self.control.contains(Control::bg_window_enable) && self.window_y_position <= self.scanline {
            self.fetch_window_pixels().into_iter()
                .for_each(|(key, Pixel { color, palette, background_priority })| {
                    pixels[key] = self.color_from_palette(color, palette);
                    bg_priority[key] = background_priority;
                })
        }
        if self.control.contains(Control::sprite_enable) {
            self.fetch_sprites_pixels().into_iter()
                .for_each(|(key, Pixel { color, palette, background_priority })| {
                if !(background_priority && bg_priority[key]) {
                    pixels[key] = self.color_from_palette(color, palette);
                }
            })
        }
        
        self.frame_buffer[line_start..line_end].copy_from_slice(&pixels);
        self.updated = true;

        self.t_cycles -= 172;
        self.mode = Mode::HorizontalBlank;
        if self.status.contains(Status::mode_0_int_select) {
            self.interrupt |= 1 << 1;
        }
    }
    fn fetch_background_pixels(&self) -> HashMap<usize, Pixel>  {
        let mut pixels = HashMap::new();
        let y = self.scanline.wrapping_add(self.vertical_scroll);
        let row = (y / 8) as usize;

        for i in 0..SCREEN_WIDTH {
            let x = (i as u8).wrapping_add(self.horizontal_scroll);
            let col = (x / 8) as usize;

            let tile_number = self.video_ram[(self.control.bg_map_mask() | (row * 32 + col)) & 0x1fff];
            let tile_data_base = self.control.tile_data_base_from_tile_number(tile_number);
            let line = ((y % 8) * 2) as usize;
            let tile_data_low = self.video_ram[(tile_data_base + line) & 0x1fff];
            let tile_data_high = self.video_ram[(tile_data_base + line + 1) & 0x1fff];
            let color = self.pixel_color_from_bits(tile_data_low, tile_data_high, x);

            pixels.insert(i, Pixel {
                color,
                palette: self.bg_palette,
                background_priority: color != 0x00
            });
        }
        pixels
    }
    fn color_from_palette(&self, pixel: u8, palette: u8) -> u8 {
        let pixel_value = pixel & 0b11;
        let shift_amount = pixel_value * 2;
        (palette >> shift_amount) & 0b11
    }
    fn pixel_color_from_bits(&self, tile_data_low: u8, tile_data_high: u8, x: u8) -> u8 {
        // mest signifikante bit i en u8 er pikselen lengst til venstre. Må derfor snu x
        let x_bit = 7 - (x % 8);
        (((tile_data_high >> x_bit) & 1) << 1) | ((tile_data_low >> x_bit) & 1)
    }
    fn fetch_window_pixels(&self) -> HashMap<usize, Pixel>   {
        let mut pixels = HashMap::new();
        let y = self.scanline - self.window_y_position;
        let row = (y / 8) as usize;
        let start_x = self.window_x_position.wrapping_sub(7) as usize;

        for x in start_x..SCREEN_WIDTH {
            let col = x / 8;
            let tile_number = self.video_ram[(self.control.window_map_mask() | (row * 32 + col)) & 0x1fff];
            let tile_data_base = self.control.tile_data_base_from_tile_number(tile_number);
            let line = ((y % 8) * 2) as usize;
            let tile_data_low = self.video_ram[(tile_data_base + line) & 0x1fff];
            let tile_data_high = self.video_ram[(tile_data_base + line + 1) & 0x1fff];
            let color = self.pixel_color_from_bits(tile_data_low, tile_data_high, x as u8);

            pixels.insert(x, Pixel {
                color,
                palette: self.bg_palette,
                background_priority: color != 0x00,
            });
        }

        pixels
    }
    fn fetch_sprites_pixels(&self) -> HashMap<usize, Pixel> {
        let mut pixels = HashMap::new();
        for sprite in self.sprite_buffer.iter() {
            let tile_line = match sprite.flags.contains(SpriteFlags::y_flip) {
                false => self.scanline.wrapping_sub(sprite.y),
                true => self.control.sprite_height() - self.scanline.wrapping_sub(sprite.y) - 1,
            } * 2;
            let tile_data_low = self.video_ram[(((sprite.tile_index as u16) << 4) | (tile_line as u16)) as usize];
            let tile_data_high = self.video_ram[(((sprite.tile_index as u16) << 4) | (tile_line as u16 + 1u16)) as usize];

            for x in 0..8 {
                let sprite_x = sprite.x.wrapping_add(x);
                if sprite_x > SCREEN_WIDTH as u8 { continue }
                
                let bit = match sprite.flags.contains(SpriteFlags::x_flip) {
                    false => x,
                    true => 7 - x,
                };
                
                match self.pixel_color_from_bits(tile_data_low, tile_data_high, bit) {
                    0x00 => continue,
                    color => {
                        let palette = if sprite.flags.contains(SpriteFlags::palette) { self.obj_palette_1 } else { self.obj_palette_0 };

                        pixels.entry(sprite_x as usize).or_insert(Pixel {
                            color,
                            palette,
                            background_priority: sprite.flags.contains(SpriteFlags::obj_to_bg_priority),
                        });
                    }
                }
            }
        }
        pixels
    }
    fn horizontal_blank(&mut self) {
        self.t_cycles -= 204;
        self.scanline += 1;
        self.check_scanline_interrupt();
        self.mode = match self.scanline >= 144 {
            true => {
                self.interrupt |= 1;
                if self.status.contains(Status::mode_1_int_select) {
                    self.interrupt |= 1 << 1;
                }
                Mode::VerticalBlank
            },
            false => Mode::OAMScan,
        };
    }
    fn check_scanline_interrupt(&mut self) {
        if self.status.contains(Status::lyc_select) && self.scanline == self.scanline_compare {
            self.interrupt |= 1 << 1;
        }
    }
    fn vertical_blank(&mut self) {
        self.t_cycles -= 456;
        self.scanline += 1;
        self.scanline %= SCANLINES;
        if self.scanline == 0 {
            self.mode = Mode::OAMScan;
            if self.status.contains(Status::mode_2_int_select) {
                self.interrupt |= 1 << 1;
            }
        }
    }
}