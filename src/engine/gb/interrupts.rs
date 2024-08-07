pub enum InterruptType {
    VBLANK = 1,
    LCD_STAT = 2,
    TIMER = 4,
    SERIAL = 8,
    JOYPAD = 16,
}

impl Clone for InterruptType {
    fn clone(&self) -> InterruptType {
        match self {
            InterruptType::VBLANK => InterruptType::VBLANK,
            InterruptType::LCD_STAT => InterruptType::LCD_STAT,
            InterruptType::TIMER => InterruptType::LCD_STAT,
            InterruptType::SERIAL => InterruptType::SERIAL,
            InterruptType::JOYPAD => InterruptType::JOYPAD,
        }
    }
}
