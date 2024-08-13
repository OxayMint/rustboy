use std::{ops::AddAssign, process::exit, thread::sleep, time::Duration};

use crate::libs::gameboy::{
    cpu::CPU,
    interrupts::InterruptType,
    io::lcd::{Mode, StatType, LCD},
    rendering::Renderer,
};

use super::{have_update, LINES_PER_FRAME, PPU, TICKS_PER_LINE, YRES};

impl PPU {
    pub fn mode_oam(&mut self, lcd: &mut LCD) {
        if self.line_ticks >= 80 {
            lcd.lcds_mode_set(Mode::XFER);
        }
    }
    pub fn mode_xfer(&mut self, lcd: &mut LCD) {
        if self.line_ticks >= 80 + 172 {
            lcd.lcds_mode_set(Mode::HBlank);
        }
    }
    pub fn mode_vblank(&mut self, lcd: &mut LCD) {
        // println!("mode {}", lcd.lcds_mode() as u8);
        // exit(0);
        if self.line_ticks >= TICKS_PER_LINE.into() {
            lcd.ly_increment();
            if lcd.ly >= LINES_PER_FRAME {
                lcd.lcds_mode_set(Mode::OAM);
                lcd.ly = 0;
            }
            self.line_ticks = 0;
        }
    }
    pub fn mode_hblank(&mut self, lcd: &mut LCD) {
        // println!("mode {}", lcd.lcds_mode() as u8);
        if self.line_ticks >= TICKS_PER_LINE.into() {
            lcd.ly_increment();

            if lcd.ly >= YRES {
                lcd.lcds_mode_set(Mode::VBlank);
                CPU::request_interrupt(InterruptType::VBLANK);
                if lcd.lcds_stat_int(StatType::VBLANK) > 0 {
                    CPU::request_interrupt(InterruptType::LCD_STAT);
                }
                self.current_frame = self.current_frame.wrapping_add(1);

                // calc FPS...
                let end = Renderer::get_ticks();
                let frame_time = end - self.prev_frame_time;

                if frame_time < self.target_frame_time {
                    sleep(Duration::from_millis(self.target_frame_time - frame_time))
                }
                // unsafe { sleep(1) };
                self.prev_frame_time = Renderer::get_ticks();
                if self.prev_frame_time - self.start_timer >= 1000 {
                    let fps = self.frame_count;
                    self.start_timer = end;
                    self.frame_count = 0;

                    println!("FPS: {}", fps);
                }

                (*have_update.lock().unwrap()) = true;
                self.prev_frame_time = end;
            } else {
                lcd.lcds_mode_set(Mode::OAM);
            }
            self.line_ticks = 0;
        }
    }
}
