use std::{
    thread,
    time::{Duration, Instant},
};

use super::{LINES_PER_FRAME, PPU, TICKS_PER_LINE, XRES, YRES};

use crate::{
    interrupts::InterruptType,
    io::lcd::{Mode, StatType},
};

impl PPU {
    pub fn ly_increment(&mut self) {
        if self.window_is_visible()
            && self.lcd.ly >= self.lcd.win_y
            && self.lcd.ly < self.lcd.win_y.wrapping_add(YRES)
        {
            self.window_line = self.window_line.wrapping_add(1);
        }
        // println!("incrementing ly {}", self.ly);
        self.lcd.ly = self.lcd.ly.wrapping_add(1);
        // println!("incremented ly {}", self.ly);

        if self.lcd.ly as u8 == self.lcd.ly_compare {
            self.lcd.lcds_lyc_set(true);
            // println!("ly is ly compare");

            if self.lcd.lcds_stat_int(StatType::LYC) {
                // println!("stat is LYC, interrupting");

                if let Some(request_interrupt) = &self.request_interrupt {
                    request_interrupt(InterruptType::LCD_STAT);
                }
            }
            // println!("stat is not LYC");
        } else {
            // println!("ly is not ly compare");
            self.lcd.lcds_lyc_set(false);
        }
    }
    pub fn mode_oam(&mut self) {
        if self.line_ticks >= 80 {
            self.lcd.lcds_mode_set(Mode::XFER);
            self.pf_control.reset_x();
        }
        if self.line_ticks == 1 {
            self.line_entries.clear();
            self.load_line_sprites();
        }
    }
    pub fn mode_xfer(&mut self) {
        // if lcd.ly >= YRES{
        //     return;
        // }
        self.pipeline_process();
        if self.pf_control.pushed_x >= XRES {
            self.pipeline_fifo_reset();
            self.lcd.lcds_mode_set(Mode::HBlank);
            if self.lcd.lcds_stat_int(StatType::HBLANK) {
                if let Some(request_interrupt) = &self.request_interrupt {
                    request_interrupt(InterruptType::LCD_STAT);
                }
            }
        }
    }
    pub fn mode_vblank(&mut self) {
        // println!("mode {}", lcd.lcds_mode() as u8);
        // exit(0);
        if self.line_ticks >= TICKS_PER_LINE {
            self.ly_increment();

            if self.lcd.ly >= LINES_PER_FRAME {
                self.lcd.lcds_mode_set(Mode::OAM);
                self.lcd.ly = 0;
                self.window_line = 0;
            }

            self.line_ticks = 0;
        }
    }

    pub fn mode_hblank(&mut self) {
        if self.line_ticks >= TICKS_PER_LINE {
            self.ly_increment();

            if self.lcd.ly >= YRES {
                self.lcd.lcds_mode_set(Mode::VBlank);

                if let Some(request_interrupt) = &self.request_interrupt {
                    request_interrupt(InterruptType::VBLANK);

                    if self.lcd.lcds_stat_int(StatType::VBLANK) {
                        request_interrupt(InterruptType::LCD_STAT);
                    }
                }

                let sleep_time: Duration;
                let elapsed = self.last_frame_end.elapsed(); //add deduction period compensating the real hardware delay
                if elapsed < self.frame_duration {
                    sleep_time = self.frame_duration - elapsed;
                    thread::sleep(sleep_time);
                }

                self.last_frame_end = Instant::now();
                self.have_update = true;
            } else {
                self.lcd.lcds_mode_set(Mode::OAM);
            }
            self.line_ticks = 0;
        }
    }
}
