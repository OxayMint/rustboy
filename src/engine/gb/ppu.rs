// use crate::libs::;

#[path = "model/ppu.rs"]
pub mod ppu_models;
#[path = "ppu_pipeline.rs"]
pub mod ppu_pipeline;
#[path = "ppu_sm.rs"]
pub mod ppu_sm;
use std::time::{Duration, Instant};

use ppu_models::*;
use sdl2::pixels::Color;

use super::io::lcd::{Mode, COLORS, LCD};

pub static LINES_PER_FRAME: u8 = 154;
pub static TICKS_PER_LINE: u16 = 456;
pub static YRES: u8 = 144;
pub static XRES: u8 = 160;
pub struct PPU {
    pub oam_ram: [OamEntry; 40],
    pub vram: [u8; 0x2000],
    pub window_line: u16,
    pub lcd: LCD,
    pub line_entries: Vec<OamEntry>,
    pub fetched_entries: Vec<OamEntry>,
    pub line_ticks: u16,
    pub video_buffer: [Color; 144 * 160],
    pub pf_control: PixelFifo,
    pub have_update: bool,
    pub last_frame_end: Instant,
    pub frame_duration: Duration,
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            oam_ram: [OamEntry::empty(); 40],
            vram: [0; 0x2000],
            lcd: LCD::new(),
            line_ticks: 0,
            window_line: 0,
            line_entries: vec![],
            fetched_entries: vec![],
            video_buffer: [COLORS[0]; 144 * 160],
            pf_control: PixelFifo::new(),
            have_update: false,
            last_frame_end: Instant::now(),
            frame_duration: Duration::from_secs_f64(1.0 / 60.0),
        }
    }

    pub fn tick(&mut self) {
        self.line_ticks = self.line_ticks.wrapping_add(1);
        // println!(
        //     "ppu tick. mode: {:?}, tick: {}",
        //     lcd.lcds_mode(),
        //     self.line_ticks
        // );
        match self.lcd.lcds_mode() {
            Mode::OAM => self.mode_oam(),
            Mode::HBlank => self.mode_hblank(),
            Mode::VBlank => self.mode_vblank(),
            Mode::XFER => self.mode_xfer(),
        }
        // match Bus::
        // let lcd = Bus::
        // match self
    }

    pub fn have_update(&mut self) -> bool {
        if self.have_update {
            self.have_update = false;
            true
        } else {
            false
        }
    }

    pub fn oam_write(&mut self, address: usize, value: u8) {
        if (address == 0xfe00 && value > 0) {
            println!("dfsdf")
        }
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
        // println!("reading vram: {address:04X}");
        self.vram[address - 0x8000]
    }

    pub fn get_video_buffer(&self) -> Vec<Color> {
        Vec::from(self.video_buffer)
    }
}
