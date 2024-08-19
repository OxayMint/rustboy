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

    //     Byte 3 — Attributes/Flags

    // Priority: 0 = No, 1 = BG and Window colors 1–3 are drawn over this OBJ
    // Y flip: 0 = Normal, 1 = Entire OBJ is vertically mirrored
    // X flip: 0 = Normal, 1 = Entire OBJ is horizontally mirrored
    // DMG palette [Non CGB Mode only]: 0 = OBP0, 1 = OBP1
    // Bank [CGB Mode Only]: 0 = Fetch tile from VRAM bank 0, 1 = Fetch tile from VRAM bank 1
    // CGB palette [CGB Mode Only]: Which of OBP0–7 to use
    pub fn x_flipped(&self) -> bool {
        self.attributes >> 5 & 1 > 0
    }
    pub fn y_flipped(&self) -> bool {
        self.attributes >> 6 & 1 > 0
    }
    pub fn draw_under_bg(&self) -> bool {
        self.attributes >> 7 & 1 > 0
    }
    pub fn palette(&self) -> u8 {
        self.attributes >> 4 & 1
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
    pub line_x: u8,
    pub pushed_x: u8,
    pub fetch_x: u8,
    pub bgw_fetch_data: [u8; 3],
    pub fetch_entry_data: [u8; 6], //oam data.: usize,
    pub map_y: u8,
    pub map_x: u8,
    pub tile_y: u8,
    pub fifo_x: u8,
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
