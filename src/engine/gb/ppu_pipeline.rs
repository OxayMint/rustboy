use std::{
    ops::{AddAssign, Div, DivAssign, Mul, SubAssign},
    process::exit,
};

use sdl2::pixels::Color;

use crate::libs::gameboy::io::lcd::{self, COLORS, LCD};

use super::{FetchState, PPU, XRES};

impl PPU {
    pub fn pipeline_fifo_reset(&mut self) {
        while !self.pf_control.pixel_fifo.is_empty() {
            self.pixel_fifo_pop();
        }
    }
    pub fn pipeline_fifo_add(&mut self, lcd: &LCD) -> bool {
        if self.pf_control.pixel_fifo.len() > 8 {
            return false;
        }
        let x: i32 = (self.pf_control.fetch_x as i32)
            .wrapping_sub(8 - (lcd.scroll_x.wrapping_rem(8) as i32));

        for i in 0..8 {
            let bit = 7 - i;

            let b1 = self.pf_control.bgw_fetch_data[1];
            let b2 = self.pf_control.bgw_fetch_data[2];
            let low = b1 >> bit & 1;
            let hi = (b2 >> bit & 1) << 1;

            let mut col = COLORS[(hi | low) as usize];

            if lcd.lcdc_bgw_enabled() {
                col = lcd.bg_colors[0];
            }

            if lcd.lcdc_obj_enabled() {
                col = self.fetch_sprite_pixels(col, hi | low, lcd);
            }
            if x >= 0 {
                self.pixel_fifo_push(col);
                self.pf_control.fifo_x.add_assign(1);
            }
        }

        return true;
    }

    fn fetch_sprite_pixels(&self, col: Color, bg_col_index: u8, lcd: &LCD) -> Color {
        let mut result_color: Color = col;
        for (index, entry) in self.fetched_entries.iter().enumerate() {
            let obj_x = ((entry.x - 8) + (lcd.scroll_x % 8)) as usize;
            if obj_x + 8 < self.pf_control.fifo_x {
                continue;
            }
            let offset = self.pf_control.fifo_x.wrapping_sub(obj_x);
            if offset > 7 {
                continue;
            }
            let bit = if entry.x_flipped() {
                offset
            } else {
                7 - offset
            };

            let b1 = self.pf_control.fetch_entry_data[index * 2];
            let b2 = self.pf_control.fetch_entry_data[(index * 2) + 1];
            let low = b1 >> bit & 1;
            let hi = (b2 >> bit & 1) << 1;
            let col_idx = (hi | low) as usize;
            if col_idx == 0 {
                //transparent pixel
                continue;
            }
            if !entry.draw_under_bg() || bg_col_index == 0 {
                result_color = if entry.palette() == 0 {
                    lcd.obj0_colors[col_idx]
                } else {
                    lcd.obj1_colors[col_idx]
                };
                if col_idx > 0 {
                    break;
                }
            }
        }
        result_color
    }

    pub fn pixel_fifo_push(&mut self, color: Color) {
        self.pf_control.pixel_fifo.push_back(color);
    }
    pub fn pixel_fifo_pop(&mut self) -> Option<Color> {
        self.pf_control.pixel_fifo.pop_front()
    }
    pub fn pipeline_fetch(&mut self, lcd: &mut LCD) {
        match self.pf_control.cur_fetch_state {
            super::FetchState::TILE => {
                self.fetched_entries.clear();

                if !lcd.lcdc_bgw_enabled() {
                    let bg_map_start = lcd.lcdc_bg_map_area();
                    self.pf_control.bgw_fetch_data[0] = self.vram_read(
                        bg_map_start
                            + (self.pf_control.map_x.wrapping_div(8))
                            + ((self.pf_control.map_y.wrapping_div(8)).wrapping_mul(32)),
                    );
                    if lcd.lcdc_bg_data_area() == 0x8800 {
                        self.pf_control.bgw_fetch_data[0] =
                            self.pf_control.bgw_fetch_data[0].wrapping_add(128);
                    }
                }

                if lcd.lcdc_obj_enabled() && !self.line_entries.is_empty() {
                    self.pipeline_load_sprite_tile(lcd);
                }

                self.pf_control.cur_fetch_state = FetchState::DATA0;
                self.pf_control.fetch_x.add_assign(8);
            }
            super::FetchState::DATA0 => {
                let idx = lcd.lcdc_bg_data_area().wrapping_add(
                    ((self.pf_control.bgw_fetch_data[0] as usize).wrapping_mul(16))
                        .wrapping_add(self.pf_control.tile_y),
                );
                self.pf_control.bgw_fetch_data[1] = self.vram_read(idx);

                self.pipeline_load_sprite_data(0, lcd);

                self.pf_control.cur_fetch_state = FetchState::DATA1;
            }
            super::FetchState::DATA1 => {
                let data_start = lcd.lcdc_bg_data_area();
                self.pf_control.bgw_fetch_data[2] = self.vram_read(
                    data_start.wrapping_add(
                        ((self.pf_control.bgw_fetch_data[0] as usize).wrapping_mul(16))
                            .wrapping_add(self.pf_control.tile_y + 1),
                    ),
                );

                // if data_start == 0x8800 {
                // println!(
                //     "bg_map_start: {map_start:04X}, bg_data_start: {data_start:04X}, map_x: {:03}, map_y: {:03}, fetch_id: {:04X}, data[1]: {:02X}, data[2]: {:02X}",
                //     self.pf_control.map_x,self.pf_control.map_y,self.pf_control.bgw_fetch_data[0],self.pf_control.bgw_fetch_data[1],self.pf_control.bgw_fetch_data[2],
                // );
                // }

                self.pipeline_load_sprite_data(1, lcd);

                self.pf_control.cur_fetch_state = FetchState::SLEEP;
            }
            super::FetchState::SLEEP => {
                self.pf_control.cur_fetch_state = FetchState::PUSH;
            }
            super::FetchState::PUSH => {
                if self.pipeline_fifo_add(lcd) {
                    self.pf_control.cur_fetch_state = FetchState::TILE;
                }
            }
        }
    }

    pub fn pipeline_process(&mut self, lcd: &mut LCD) {
        self.pf_control.map_y = lcd.ly as usize + lcd.scroll_y as usize;
        self.pf_control.map_x = self.pf_control.fetch_x as usize + lcd.scroll_x as usize;
        if self.pf_control.map_x > 255 {
            self.pf_control.map_x.sub_assign(255);
        }
        self.pf_control.tile_y =
            (lcd.scroll_y.wrapping_add(self.pf_control.map_y as u8) as usize % 8) * 2;

        if self.pf_control.map_y >= 256 {
            self.pf_control.map_y.sub_assign(256);
        }
        if self.line_ticks & 1 == 0 {
            self.pipeline_fetch(lcd);
        }
        self.pipeline_push_pixel(lcd);
    }

    pub fn pipeline_push_pixel(&mut self, lcd: &mut LCD) {
        if self.pf_control.pixel_fifo.len() > 8 {
            if let Some(pixel) = self.pixel_fifo_pop() {
                if self.pf_control.line_x >= (lcd.scroll_x as usize % 8) {
                    let idx = self.pf_control.pushed_x + (lcd.ly as usize * XRES as usize);
                    self.video_buffer[idx] = pixel;
                    self.pf_control.pushed_x += 1;
                }
            }
            self.pf_control.line_x += 1;
        }
    }
    pub fn pipeline_load_sprite_tile(&mut self, lcd: &mut LCD) {
        for entry in &self.line_entries {
            let obj_x = entry.x as i32 - 8 + lcd.scroll_x as i32 % 8;
            let cur_x = self.pf_control.fetch_x as i32;
            if (obj_x >= cur_x && obj_x < cur_x + 8)
                || (obj_x + 8 >= cur_x && obj_x + 8 < cur_x + 8)
            {
                self.fetched_entries.push(*entry);
                if self.fetched_entries.len() == 3 {
                    break;
                }
            }
        }
    }

    fn pipeline_load_sprite_data(&mut self, offset: usize, lcd: &mut LCD) {
        let cur_y = lcd.ly;
        let obj_height = lcd.lcdc_obj_height();
        for (index, entry) in self.fetched_entries.iter().enumerate() {
            let mut ty = (cur_y + 16 - entry.y) * 2;
            if entry.y_flipped() {
                ty = (obj_height * 2) - 2 - ty;
            }
            let mut tile_idx = entry.tile_idx;
            if obj_height == 16 {
                tile_idx &= !(1);
            }
            self.pf_control.fetch_entry_data[(index * 2) + offset] =
                self.vram_read(0x8000 + (tile_idx as usize * 16) + ty as usize + offset);
        }
    }

    pub fn load_line_sprites(&mut self, lcd: &mut LCD) {
        let ly = lcd.ly;
        let obj_height = lcd.lcdc_obj_height();
        self.line_entries.clear();
        for entry in self.oam_ram {
            if entry.x == 0 {
                //not visible
                continue;
            }
            if self.line_entries.len() >= 10 {
                break;
            }
            // if (entry.y >= ly && entry.y + ) {}
            if entry.y <= ly + 16 && entry.y + obj_height > ly + 16 {
                self.line_entries.push(entry);
            }
        }
        self.line_entries.sort_by(|a, b| a.x.cmp(&b.x))
    }
}
