use std::{
    ops::AddAssign,
    thread::sleep,
    time::{Duration, SystemTime},
};

use crate::libs::gameboy::{
    cpu::CPU,
    interrupts::InterruptType,
    io::lcd::{Mode, StatType, LCD},
    rendering::Renderer,
};

use super::{ LINES_PER_FRAME, PPU, TICKS_PER_LINE, XRES, YRES};

impl PPU {
    pub fn mode_oam(&mut self, lcd: &mut LCD) {
        if self.line_ticks >= 80 {
            lcd.lcds_mode_set(Mode::XFER);
            self.pf_control.reset_x();
        }
        if self.line_ticks == 1 {
            self.line_entries.clear();
            self.load_line_sprites(lcd);
        }
    }
    pub fn mode_xfer(&mut self, lcd: &mut LCD) {
        // if lcd.ly >= YRES{
        //     return;
        // }
        self.pipeline_process(lcd);
        if self.pf_control.pushed_x >= XRES as usize {
            self.pipeline_fifo_reset();
            lcd.lcds_mode_set(Mode::HBlank);
            if lcd.lcds_stat_int(StatType::HBLANK) {
                CPU::request_interrupt(InterruptType::LCD_STAT);
            }
        }
    }
    pub fn mode_vblank(&mut self, lcd: &mut LCD) {
        // println!("mode {}", lcd.lcds_mode() as u8);
        // exit(0);
        if self.line_ticks >= TICKS_PER_LINE {
            lcd.ly_increment();

            if lcd.ly >= LINES_PER_FRAME {
                lcd.lcds_mode_set(Mode::OAM);
                lcd.ly = 0;
            }

            self.line_ticks = 0;
        }
    }
    pub fn mode_hblank(&mut self, lcd: &mut LCD) {
        if self.line_ticks >= TICKS_PER_LINE {
            lcd.ly_increment();

            if lcd.ly >= YRES {
                lcd.lcds_mode_set(Mode::VBlank);

                CPU::request_interrupt(InterruptType::VBLANK);

                if lcd.lcds_stat_int(StatType::VBLANK) {
                    CPU::request_interrupt(InterruptType::LCD_STAT);
                }

                self.have_update = true;
                // self.prev_frame_time = Renderer::get_ticks();
            } else {
                lcd.lcds_mode_set(Mode::OAM);
            }
            self.line_ticks = 0;
        }
    }
}
