use std::{
    ops::{BitAnd, BitOr, BitOrAssign},
    sync::Mutex,
};

use sdl2::pixels::Color;

use crate::libs::gameboy::{cpu::CPU, dma::DMA, interrupts::InterruptType};

pub static COLORS: [Color; 4] = [
    //E1F8CF
    Color::RGB(0xE1, 0xF8, 0xCF),
    //87C06C
    Color::RGB(0x87, 0xC0, 0x6C),
    //2F6850
    Color::RGB(0x2F, 0x68, 0x50),
    //071821
    Color::RGB(0x07, 0x18, 0x21),
];
// Color::WHITE,
// Color::RGB(0xAA, 0xAA, 0xAA),
// Color::RGB(0x55, 0x55, 0x55),
// Color::RGB(0x00, 0x00, 0x00),

pub enum StatType {
    HBLANK = (1 << 3),
    VBLANK = (1 << 4),
    OAM = (1 << 5),
    LYC = (1 << 6),
}

impl BitOr<u8> for StatType {
    type Output = u8;

    fn bitor(self, rhs: u8) -> Self::Output {
        (self as u8) | rhs
    }
}

impl BitOr<StatType> for u8 {
    type Output = u8;

    fn bitor(self, rhs: StatType) -> Self::Output {
        self | (rhs as u8)
    }
}
impl BitAnd<StatType> for u8 {
    type Output = u8;

    fn bitand(self, rhs: StatType) -> Self::Output {
        self & (rhs as u8)
    }
}
impl BitOrAssign<StatType> for u8 {
    fn bitor_assign(&mut self, rhs: StatType) {
        *self |= rhs as u8;
    }
}

pub struct LCD {
    pub lcdc: u8,             //ff40
    pub lcds: u8,             //ff41
    pub scroll_y: u8,         //ff42
    pub scroll_x: u8,         //ff43
    pub ly: u8,               //ff44
    pub ly_compare: u8,       //ff45
    pub dma_address: u8,      //ff46
    pub bg_pallete: u8,       //ff47
    pub obj_pallete: [u8; 2], //ff48-ff49
    pub win_y: u8,            //ff4a
    pub win_x: u8,            //ff4b
    pub bg_colors: [Color; 4],
    pub obj0_colors: [Color; 4],
    pub obj1_colors: [Color; 4],
    pub dma: DMA, //ff46
}

impl LCD {
    pub fn new() -> Self {
        return LCD {
            lcdc: 0x91,
            lcds: 0,
            scroll_y: 0,
            scroll_x: 0,
            ly_compare: 0,
            dma_address: 0,
            bg_pallete: 0xFC,
            obj_pallete: [0xFF, 0xFF],
            dma: DMA::new(),
            ly: 0,
            win_y: 0,
            win_x: 0,
            bg_colors: COLORS.clone(),
            obj0_colors: COLORS.clone(),
            obj1_colors: COLORS.clone(),
        };
    }
    // 0: BG & Window enable / priority [Different meaning in CGB Mode]: 0 = Off; 1 = On
    pub fn lcdc_bgw_enabled(&self) -> bool {
        return (self.lcdc & 1) == 0;
    }
    // 1: OBJ enable: 0 = Off; 1 = On
    pub fn lcdc_obj_enabled(&self) -> bool {
        return (self.lcdc >> 1) & 1 > 0;
    }
    // 2: OBJ size: 0 = 8×8; 1 = 8×16
    pub fn lcdc_obj_double_size(&self) -> bool {
        return (self.lcdc >> 2) & 1 > 0;
    }
    pub fn lcdc_obj_height(&self) -> u8 {
        return self.lcdc_obj_double_size() as u8 * 8 + 8;
    }
    // 3: BG tile map area: 0 = 9800–9BFF; 1 = 9C00–9FFF
    pub fn lcdc_bg_map_area(&self) -> usize {
        return if (self.lcdc >> 3) & 1 > 0 {
            0x9C00
            // (0x9C00, 0x9FFF)
        } else {
            0x9800
            // (0x9800, 0x9BFF)
        };
    }
    // 4: BG & Window tile data area: 0 = 8800–97FF; 1 = 8000–8FFF
    pub fn lcdc_bg_data_area(&self) -> usize {
        return if (self.lcdc >> 4) & 1 > 0 {
            0x8000
            // (0x8000, 0x8FFF)
        } else {
            0x8800
            // (0x8800, 0x97FF)
        };
    }
    // 5: Window enable: 0 = Off; 1 = On
    pub fn lcdc_window_enabled(&self) -> bool {
        return (self.lcdc >> 5) & 1 > 0;
    }
    //    6: Window tile map area: 0 = 9800–9BFF; 1 = 9C00–9FFF
    pub fn lcdc_window_tile_map_area(&self) -> usize {
        return if (self.lcdc >> 6) & 1 > 0 {
            0x9C00
        } else {
            0x9800
        };
    }
    // 7: LCD & PPU enable: 0 = Off; 1 = On
    pub fn lcdc_ppu_enabled(&self) -> bool {
        return (self.lcdc >> 7) & 1 > 0;
    }

    /*
    6 LYC int select (Read/Write): If set, selects the LYC == LY condition for the STAT interrupt.
    5 Mode 2 int select (Read/Write): If set, selects the Mode 2 condition for the STAT interrupt.
    4 Mode 1 int select (Read/Write): If set, selects the Mode 1 condition for the STAT interrupt.
    3 Mode 0 int select (Read/Write): If set, selects the Mode 0 condition for the STAT interrupt.
    2 LYC == LY (Read-only): Set when LY contains the same value as LYC; it is constantly updated.
    1 - 0 PPU mode (Read-only): Indicates the PPU’s current status.
     */

    pub fn lcds_mode(&self) -> Mode {
        return unsafe { ::std::mem::transmute(self.lcds & 0b11) };
    }
    pub fn lcds_mode_set(&mut self, mode: Mode) {
        // print!("setting lcds mode from {}", self.lcds as u8);
        self.lcds &= !0b11;
        self.lcds |= mode;
        // println!(" to {}", self.lcds as u8);
    }
    // pub fn lyc_is_ly(&self) -> bool {
    //     return self.lcds >> 2 & 1 > 0;
    // }
    pub fn lcds_lyc_set(&mut self, on: bool) {
        if on {
            self.lcds |= 1 << 2;
        } else {
            self.lcds &= !(1 << 2)
        }
        // println!("lyc is set to {}", on);
        // println!("lcds is {}", self.lcds);
    }

    pub fn lcds_stat_int(&self, src: StatType) -> bool {
        return (self.lcds & src) > 0;
    }
    pub fn dma_active(&self) -> bool {
        return self.dma.active;
    }

    pub fn read(&self, address: usize) -> u8 {
        match address {
            0xff40 => self.lcdc,
            0xff41 => self.lcds,
            0xff42 => self.scroll_y,
            0xff43 => self.scroll_x,
            0xff44 => self.ly as u8,
            0xff45 => self.ly_compare,
            0xff46 => self.dma_address,
            0xff47 => self.bg_pallete,
            0xff48 => self.obj_pallete[0],
            0xff49 => self.obj_pallete[1],
            0xff4a => self.win_y,
            0xff4b => self.win_x,
            _ => 0,
        }
    }

    pub fn write(&mut self, address: usize, value: u8) {
        match address {
            0xff40 => self.lcdc = value,
            0xff41 => self.lcds = value,
            0xff42 => self.scroll_y = value,
            0xff43 => self.scroll_x = value,
            0xff44 => self.ly = value,
            0xff45 => self.ly_compare = value,
            0xff46 => {
                self.dma_address = value;
                self.dma.start(value);
            }
            0xff47 => {
                self.bg_pallete = value;
                self.update_palette(value, 0)
            }
            0xff48 => {
                self.obj_pallete[0] = value;
                self.update_palette(value & !0b11, 1);
            }
            0xff49 => {
                self.obj_pallete[1] = value;
                self.update_palette(value & !0b11, 2);
            }
            0xff4a => self.win_y = value,
            0xff4b => self.win_x = value,
            _ => {}
        }
    }
    pub fn update_palette(&mut self, data: u8, palette: u8) {
        // let colors = match palette {
        //     1 => &mut self.obj0_colors,
        //     2 => &mut self.obj1_colors,
        //     _ => &mut self.bg_colors,
        // };
        // for i in 0..3 {
        //     colors[i] = COLORS[(data >> (i * 2)) as usize & 0b11];
        // }
        match palette {
            1 => {
                self.obj0_colors[0] = COLORS[data as usize & 0b11];
                self.obj0_colors[1] = COLORS[(data >> 2) as usize & 0b11];
                self.obj0_colors[2] = COLORS[(data >> 4) as usize & 0b11];
                self.obj0_colors[3] = COLORS[(data >> 6) as usize & 0b11];
            }
            2 => {
                self.obj1_colors[0] = COLORS[data as usize & 0b11];
                self.obj1_colors[1] = COLORS[(data >> 2) as usize & 0b11];
                self.obj1_colors[2] = COLORS[(data >> 4) as usize & 0b11];
                self.obj1_colors[3] = COLORS[(data >> 6) as usize & 0b11];
            }
            _ => {
                self.bg_colors[0] = COLORS[data as usize & 0b11];
                self.bg_colors[1] = COLORS[(data >> 2) as usize & 0b11];
                self.bg_colors[2] = COLORS[(data >> 4) as usize & 0b11];
                self.bg_colors[3] = COLORS[(data >> 6) as usize & 0b11];
            }
        };
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum Mode {
    HBlank = 0b00,
    VBlank = 0b01,
    OAM = 0b10,
    XFER = 0b11,
}

impl BitOr<u8> for Mode {
    type Output = u8;

    fn bitor(self, rhs: u8) -> Self::Output {
        (self as u8) | rhs
    }
}

impl BitOr<Mode> for u8 {
    type Output = u8;

    fn bitor(self, rhs: Mode) -> Self::Output {
        self | (rhs as u8)
    }
}
impl BitOrAssign<Mode> for u8 {
    fn bitor_assign(&mut self, rhs: Mode) {
        *self |= rhs as u8;
    }
}
