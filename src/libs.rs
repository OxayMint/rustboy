#[path = "engine/gameboy_engine.rs"]
pub mod gameboy;

pub trait SetBytes {
    fn from_pair(low: u8, hi: u8) -> u16;
    fn set_low(&mut self, value: u8);
    fn set_high(&mut self, value: u8);
    fn separate_bytes(&mut self) -> (u8, u8);
}

impl SetBytes for u16 {
    fn set_low(&mut self, value: u8) {
        *self &= !0xff;
        *self |= value as u16;
    }
    fn set_high(&mut self, value: u8) {
        *self &= !0xff00;
        *self |= (value as u16) << 8;
    }

    fn separate_bytes(&mut self) -> (u8, u8) {
        return (*self as u8, (*self >> 8) as u8);
    }
    fn from_pair(low: u8, hi: u8) -> u16 {
        let mut res: u16 = 0;
        res.set_low(low);
        res.set_high(hi);
        return res;
    }
}
