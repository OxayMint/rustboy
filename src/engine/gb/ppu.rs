// use crate::libs::;

#[path = "model/ppu.rs"]
pub mod ppu_models;
use std::ops::SubAssign;

use ppu_models::*;
// OAM_Entry
pub struct PPU {
    pub oam_ram: [OamEntry; 40],
    pub vram: [u8; 0x2000],
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
        PPU {
            oam_ram: [OamEntry::empty(); 40],
            vram: [0; 0x2000],
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
