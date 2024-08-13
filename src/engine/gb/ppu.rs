// use crate::libs::;

#[path = "model/ppu.rs"]
pub mod ppu_models;
#[path = "ppu_sm.rs"]
pub mod ppu_sm;

use std::{ops::AddAssign, sync::Mutex};

use ppu_models::*;

lazy_static! {
    static ref PPU_INSTANCE: Mutex<PPU> = Mutex::new(PPU::new());
}

use super::io::lcd::{Mode, LCD_INSTANCE};

pub static LINES_PER_FRAME: u16 = 154;
pub static TICKS_PER_LINE: u16 = 456;
pub static YRES: u16 = 144;
pub static XRES: u32 = 160;
pub static have_update: Mutex<bool> = Mutex::new(false);
pub struct PPU {
    pub oam_ram: [OamEntry; 40],
    pub vram: [u8; 0x2000],
    pub current_frame: u32,
    pub line_ticks: u16,
    pub video_buffer: [u32; 144 * 160],

    pub target_frame_time: u64,
    pub prev_frame_time: u64,
    pub start_timer: u64,
    pub frame_count: u64,
    // = 1000 / 60
    // = 0
    // = 0
    // = 0
    // pixel_fifo_context pfc;

    // u8 line_sprite_count; //0 to 10 sprites.
    // oam_line_entry *line_sprites; //linked list of current sprites on line.
    // oam_line_entry line_entry_array[10]; //memory to use for list.

    // u8 fetched_entry_count;
    // oam_entry fetched_entries[3]; //entries fetched during pipeline.

    // u32 current_frame;
    // u32 line_ticks;
    // u32 *video_buffer;
}

impl PPU {
    pub fn new() -> PPU {
        let mut lcd = LCD_INSTANCE.lock().unwrap();
        lcd.lcds_mode_set(Mode::OAM);
        drop(lcd);
        PPU {
            oam_ram: [OamEntry::empty(); 40],
            vram: [0; 0x2000],
            current_frame: 0,
            line_ticks: 0,
            video_buffer: [0; 144 * 160],

            target_frame_time: 1000 / 60,
            prev_frame_time: 0,
            start_timer: 0,
            frame_count: 0,
        }
    }

    pub fn tick() {
        // println!("tick");
        let mut ppu = PPU_INSTANCE.lock().unwrap();
        ppu._tick();
    }
    fn _tick(&mut self) {
        // println!("_tick");
        self.line_ticks.add_assign(1);
        let mut lcd = LCD_INSTANCE.lock().unwrap();
        // println!("ppu tick. lcds mode : {}", lcd.lcds_mode() as u8);
        match lcd.lcds_mode() {
            Mode::OAM => self.mode_oam(&mut lcd),
            Mode::HBlank => self.mode_hblank(&mut lcd),
            Mode::VBlank => self.mode_vblank(&mut lcd),
            Mode::XFER => self.mode_xfer(&mut lcd),
        }
        // match Bus::
        // let lcd = Bus::
        // match self
    }

    pub fn have_update() -> bool {
        // return true;
        let mut update = have_update.lock().unwrap();
        if *update {
            *update = false;
            true
        } else {
            false
        }
    }

    pub fn oam_write(&mut self, address: usize, value: u8) {
        let adjusted_address = if address >= 0xFE00 {
            address.wrapping_sub(0xFE00)
        } else {
            address
        } as usize;

        if adjusted_address < 160 {
            // 40 entries * 4 bytes each = 160 bytes
            let entry_index = adjusted_address / 4;
            let byte_index = adjusted_address % 4;

            match byte_index {
                0 => self.oam_ram[entry_index].y = value,
                1 => self.oam_ram[entry_index].x = value,
                2 => self.oam_ram[entry_index].tile_idx = value,
                3 => self.oam_ram[entry_index].attributes = value,
                _ => unreachable!(),
            }
        }
    }
    pub fn oam_read(&self, address: usize) -> u8 {
        let adjusted_address = if address >= 0xFE00 {
            address.wrapping_sub(0xFE00)
        } else {
            address
        } as usize;

        if adjusted_address < 160 {
            // 40 entries * 4 bytes each = 160 bytes
            let entry_index = adjusted_address / 4;
            let byte_index = adjusted_address % 4;

            return match byte_index {
                0 => self.oam_ram[entry_index].y,
                1 => self.oam_ram[entry_index].x,
                2 => self.oam_ram[entry_index].tile_idx,
                3 => self.oam_ram[entry_index].attributes,
                _ => 0,
            };
        }
        0
    }
    pub fn oam_get_entry_by_index(&self, index: usize) -> &OamEntry {
        return &self.oam_ram[index];
    }
    // pub fn oam_get_entry_by_address(&self, index: usize) -> &OamEntry {
    //     return &self.oam_ram[index];
    // }
    pub fn vram_write(&mut self, address: usize, value: u8) {
        self.vram[address - 0x8000] = value;
    }
    pub fn vram_read(&self, address: usize) -> u8 {
        self.vram[address - 0x8000]
    }
}
