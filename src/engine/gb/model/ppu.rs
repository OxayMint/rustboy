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
