use sdl2::pixels::Color;

use super::{fetch_state::FetchState, PPU, XRES, YRES};

impl PPU {
    pub fn pipeline_fifo_reset(&mut self) {
        while !self.pf_control.pixel_fifo.is_empty() {
            self.pixel_fifo_pop();
        }
    }
    pub fn pipeline_fifo_add(&mut self) -> bool {
        if self.pf_control.pixel_fifo.len() > 8 {
            return false;
        }
        let x: i32 = (self.pf_control.fetch_x as i32)
            .wrapping_sub(8 - (self.lcd.scroll_x.wrapping_rem(8) as i32));

        for i in 0..8 {
            let bit = 7 - i;

            let b1 = self.pf_control.bgw_fetch_data[1];
            let b2 = self.pf_control.bgw_fetch_data[2];
            let low = b1 >> bit & 1;
            let hi = (b2 >> bit & 1) << 1;
            let col_index = (hi | low) as usize;
            let mut col = self.lcd.bg_colors[col_index];

            if self.lcd.lcdc_bgw_enabled() {
                col = self.lcd.bg_colors[0];
            }

            if self.lcd.lcdc_obj_enabled() {
                col = self.fetch_sprite_pixels(col, hi | low);
            }
            if x >= 0 {
                self.pixel_fifo_push(col);
                self.pf_control.fifo_x += 1;
            }
        }

        return true;
    }

    fn fetch_sprite_pixels(&self, col: Color, bg_col_index: u8) -> Color {
        let mut result_color: Color = col;
        for (index, entry) in self.fetched_entries.iter().enumerate() {
            let cur_x_pos = self.pf_control.fifo_x as i32;

            let obj_x = (entry.x as i32 - 8) + (self.lcd.scroll_x as i32 % 8);
            if obj_x + 8 < cur_x_pos {
                continue;
            }
            let offset = cur_x_pos - obj_x;
            if offset > 7 || offset < 0 {
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
                    self.lcd.obj0_colors[col_idx]
                } else {
                    self.lcd.obj1_colors[col_idx]
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
    pub fn pipeline_fetch(&mut self) {
        match self.pf_control.cur_fetch_state {
            FetchState::TILE => {
                self.fetched_entries.clear();

                if !self.lcd.lcdc_bgw_enabled() {
                    let bg_map_start = self.lcd.lcdc_bg_map_area();
                    self.pf_control.bgw_fetch_data[0] = self.vram_read(
                        bg_map_start
                            + (self.pf_control.map_x as usize / 8)
                            + ((self.pf_control.map_y as usize / 8) * 32),
                    );
                    if self.lcd.lcdc_bg_data_area() == 0x8800 {
                        self.pf_control.bgw_fetch_data[0] =
                            self.pf_control.bgw_fetch_data[0].wrapping_add(128);
                    }
                    self.pipeline_load_window_tile();
                }

                if self.lcd.lcdc_obj_enabled() && !self.line_entries.is_empty() {
                    self.pipeline_load_sprite_tile();
                }

                self.pf_control.cur_fetch_state = FetchState::DATA0;
                self.pf_control.fetch_x += 8;
            }
            FetchState::DATA0 => {
                let idx = self.lcd.lcdc_bg_data_area()
                    + ((self.pf_control.bgw_fetch_data[0] as usize) * 16)
                    + self.pf_control.tile_y as usize;

                self.pf_control.bgw_fetch_data[1] = self.vram_read(idx);

                self.pipeline_load_sprite_data(0);

                self.pf_control.cur_fetch_state = FetchState::DATA1;
            }
            FetchState::DATA1 => {
                let data_start = self.lcd.lcdc_bg_data_area();
                self.pf_control.bgw_fetch_data[2] = self.vram_read(
                    data_start
                        + ((self.pf_control.bgw_fetch_data[0] as usize) * 16)
                        + self.pf_control.tile_y as usize
                        + 1,
                );

                // if data_start == 0x8800 {
                // println!(
                //     "bg_map_start: {map_start:04X}, bg_data_start: {data_start:04X}, map_x: {:03}, map_y: {:03}, fetch_id: {:04X}, data[1]: {:02X}, data[2]: {:02X}",
                //     self.pf_control.map_x,self.pf_control.map_y,self.pf_control.bgw_fetch_data[0],self.pf_control.bgw_fetch_data[1],self.pf_control.bgw_fetch_data[2],
                // );
                // }

                self.pipeline_load_sprite_data(1);

                self.pf_control.cur_fetch_state = FetchState::SLEEP;
            }
            FetchState::SLEEP => {
                self.pf_control.cur_fetch_state = FetchState::PUSH;
            }
            FetchState::PUSH => {
                if self.pipeline_fifo_add() {
                    self.pf_control.cur_fetch_state = FetchState::TILE;
                }
            }
        }
    }

    pub fn pipeline_process(&mut self) {
        self.pf_control.map_y = self.lcd.ly.wrapping_add(self.lcd.scroll_y);
        self.pf_control.map_x = self.pf_control.fetch_x.wrapping_add(self.lcd.scroll_x);
        self.pf_control.tile_y = (self
            .lcd
            .scroll_y
            .wrapping_add(self.pf_control.map_y)
            .wrapping_rem(8))
        .wrapping_mul(2);

        // if self.pf_control.map_y >= 256 {
        //     self.pf_control.map_y.sub_assign(256);
        // }
        if self.line_ticks & 1 == 0 {
            self.pipeline_fetch();
        }
        self.pipeline_push_pixel();
    }

    pub fn pipeline_push_pixel(&mut self) {
        if self.pf_control.pixel_fifo.len() > 8 {
            if let Some(pixel) = self.pixel_fifo_pop() {
                if self.pf_control.line_x >= self.lcd.scroll_x.wrapping_rem(8) {
                    let idx =
                        self.pf_control.pushed_x as usize + (self.lcd.ly as usize * XRES as usize);
                    self.video_buffer[idx] = pixel;
                    self.pf_control.pushed_x += 1;
                }
            }
            self.pf_control.line_x += 1;
        }
    }

    pub fn pipeline_load_sprite_tile(&mut self) {
        for entry in &self.line_entries {
            let obj_x = entry.x as i32 - 8 + self.lcd.scroll_x as i32 % 8;
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

    fn pipeline_load_sprite_data(&mut self, offset: usize) {
        let cur_y = self.lcd.ly;
        let obj_height = self.lcd.lcdc_obj_height();
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

    pub fn load_line_sprites(&mut self) {
        let ly = self.lcd.ly;
        let obj_height = self.lcd.lcdc_obj_height();
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

    // fn pipeline_load_window_tile(&mut self) {
    //     if !self.window_is_visible() {
    //         return;
    //     }
    //     // let win_y = self.lcd.win_y as usize;
    //     // let win_x = self.lcd.win_x as usize;
    //     // let y_res = YRES as usize;
    //     // let x_res = XRES as usize;

    //     //TODO maybe switch yres and xres below
    //     if self.pf_control.fetch_x.wrapping_add(7) >= self.lcd.win_x
    //         && self.pf_control.fetch_x < self.lcd.win_x.wrapping_add(XRES + 14)
    //     {
    //         // let ly = self.lcd.ly as usize;
    //         if self.lcd.ly >= self.lcd.win_y && self.lcd.ly < self.lcd.win_y.wrapping_add(YRES) {
    //             let window_tile_y = self.window_line / 8;
    //             self.pf_control.bgw_fetch_data[0] = self.vram_read(
    //                 self.lcd.lcdc_window_tile_map_area()
    //                     + ((self.pf_control.fetch_x as usize + 7 - self.lcd.win_x as usize) / 8)
    //                     + (window_tile_y * 32) as usize,
    //             );

    //             if self.lcd.lcdc_bg_data_area() == 0x8800 {
    //                 self.pf_control.bgw_fetch_data[0] =
    //                     self.pf_control.bgw_fetch_data[0].wrapping_add(128);
    //             }
    //         }
    //     }
    // }

    fn pipeline_load_window_tile(&mut self) {
        if !self.window_is_visible() {
            return;
        }
        // let win_y = self.lcd.win_y as usize;
        // let win_x = self.lcd.win_x as usize;
        // let y_res = YRES as usize;
        // let x_res = XRES as usize;

        //TODO maybe switch yres and xres below
        if self.pf_control.fetch_x.wrapping_add(7) >= self.lcd.win_x
            && self.pf_control.fetch_x < self.lcd.win_x.wrapping_add(YRES + 14)
        {
            // let ly = self.lcd.ly as usize;
            if self.lcd.ly >= self.lcd.win_y && self.lcd.ly < self.lcd.win_y.wrapping_add(XRES) {
                let window_tile_y = self.window_line / 8;
                self.pf_control.bgw_fetch_data[0] = self.vram_read(
                    self.lcd.lcdc_window_tile_map_area()
                        + ((self.pf_control.fetch_x as usize + 7 - self.lcd.win_x as usize) / 8)
                        + (window_tile_y * 32) as usize,
                );

                if self.lcd.lcdc_bg_data_area() == 0x8800 {
                    self.pf_control.bgw_fetch_data[0] =
                        self.pf_control.bgw_fetch_data[0].wrapping_add(128);
                }
            }
        }
    }
    pub fn window_is_visible(&self) -> bool {
        return self.lcd.lcdc_window_enabled()
            // && self.lcd.win_x <= 129
            && self.lcd.win_x <= 166
            // && self.lcd.win_y <= 129
            && self.lcd.win_y < YRES;
    }
}
