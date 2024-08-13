use std::ops::{BitOr, BitOrAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum InterruptType {
    VBLANK = 1,
    LCD_STAT = 2,
    TIMER = 4,
    SERIAL = 8,
    JOYPAD = 16,
}

impl BitOr<u8> for InterruptType {
    type Output = u8;

    fn bitor(self, rhs: u8) -> Self::Output {
        (self as u8) | rhs
    }
}

impl BitOr<InterruptType> for u8 {
    type Output = u8;

    fn bitor(self, rhs: InterruptType) -> Self::Output {
        self | (rhs as u8)
    }
}
impl BitOrAssign<InterruptType> for u8 {
    fn bitor_assign(&mut self, rhs: InterruptType) {
        *self |= rhs as u8;
    }
}
