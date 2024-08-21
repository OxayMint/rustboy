use std::collections::LinkedList;

use sdl2::pixels::Color;

use super::fetch_state::FetchState;

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
