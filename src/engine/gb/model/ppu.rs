use std::collections::LinkedList;

use sdl2::pixels::Color;

#[derive(Debug, Default, Clone, Copy)]
pub struct OamEntry {
    pub y: u8,
    pub x: u8,
    pub tile_idx: u8,
    pub attributes: u8,
}
impl OamEntry {
    pub fn from_u32(value: u32) -> Self {
        OamEntry {
            y: (value & 0xFF) as u8,
            x: ((value >> 8) & 0xFF) as u8,
            tile_idx: ((value >> 16) & 0xFF) as u8,
            attributes: ((value >> 24) & 0xFF) as u8,
        }
    }
    pub fn empty() -> OamEntry {
        OamEntry {
            y: 0,
            x: 0,
            tile_idx: 0,
            attributes: 0,
        }
    }
}

pub enum FetchState {
    TILE,
    DATA0,
    DATA1,
    SLEEP,
    PUSH,
}

pub struct PixelFifo {
    pub cur_fetch_state: FetchState,
    pub pixel_fifo: LinkedList<Color>,
    pub line_x: usize,
    pub pushed_x: usize,
    pub fetch_x: usize,
    pub bgw_fetch_data: [u8; 3],
    pub fetch_entry_data: [u8; 6], //oam data.: usize,
    pub map_y: usize,
    pub map_x: usize,
    pub tile_y: usize,
    pub fifo_x: usize,
}
impl PixelFifo {
    pub fn new() -> Self {
        PixelFifo {
            cur_fetch_state: FetchState::TILE,
            pixel_fifo: LinkedList::new(),
            line_x: 0,
            pushed_x: 0,
            fetch_x: 0,
            bgw_fetch_data: [0; 3],
            fetch_entry_data: [0; 6],
            map_y: 0,
            map_x: 0,
            tile_y: 0,
            fifo_x: 0,
        }
    }

    pub fn reset_x(&mut self) {
        self.cur_fetch_state = FetchState::TILE;
        self.line_x = 0;
        self.fetch_x = 0;
        self.pushed_x = 0;
        self.fifo_x = 0;
    }
}
