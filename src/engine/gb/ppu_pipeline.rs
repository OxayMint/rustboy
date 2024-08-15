use std::{
    ops::{AddAssign, Div, DivAssign, Mul, SubAssign},
    process::exit,
};

use sdl2::pixels::Color;

use crate::libs::gameboy::{
    bus::Bus,
    io::lcd::{self, COLORS, LCD},
};

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

            let hi = ((self.pf_control.bgw_fetch_data[1] >> bit) & 1) << 1;
            let low = ((self.pf_control.bgw_fetch_data[2] >> bit) & 1);
            let mut col = COLORS[(hi | low) as usize];

            if !lcd.lcdc_bgw_enabled() {
                col = lcd.bg_colors[0];
            }
            if x >= 0 {
                self.pixel_fifo_push(col);
                self.pf_control.fifo_x.add_assign(1);
            }
        }

        return true;
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
                if lcd.lcdc_bgw_enabled() {
                    let bg_map_start = lcd.lcdc_bg_map_area();
                    self.pf_control.bgw_fetch_data[0] = Bus::read8(
                        bg_map_start
                            + (self.pf_control.map_x.wrapping_div(8))
                            + ((self.pf_control.map_y.wrapping_div(8)).wrapping_mul(32)),
                    );
                    let bg_data_start = lcd.lcdc_bg_data_area();
                    if bg_data_start == 0x8800 {
                        self.pf_control.bgw_fetch_data[0] =
                            self.pf_control.bgw_fetch_data[0].wrapping_add(128);
                        // println!(
                        //     "bg_map_start: {bg_map_start:04X}, bg_data_start: {bg_data_start:04X}, map_x: {}, map_y: {}, fetch_id: {:04X}",
                        //     self.pf_control.map_x,self.pf_control.map_y,self.pf_control.bgw_fetch_data[0]
                        // );
                    }
                }

                self.pf_control.cur_fetch_state = FetchState::DATA0;
                self.pf_control.fetch_x.add_assign(8);
            }
            super::FetchState::DATA0 => {
                let idx = lcd.lcdc_bg_data_area().wrapping_add(
                    ((self.pf_control.bgw_fetch_data[0] as usize).wrapping_mul(16))
                        .wrapping_add(self.pf_control.tile_y),
                );
                self.pf_control.bgw_fetch_data[1] = Bus::read8(idx);

                // pipeline_load_sprite_data(0);

                self.pf_control.cur_fetch_state = FetchState::DATA1;
            }
            super::FetchState::DATA1 => {
                let map_start = lcd.lcdc_bg_map_area();
                let data_start = lcd.lcdc_bg_data_area();
                self.pf_control.bgw_fetch_data[2] = Bus::read8(
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

                // pipeline_load_sprite_data(1);

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
        // println!(
        //     "fetch_x: {}, scroll_x: {}",
        //     self.pf_control.fetch_x, lcd.scroll_x
        // );
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
}
