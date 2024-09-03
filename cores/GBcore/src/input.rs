use std::sync::Mutex;

// lazy_static! {
//     pub static ref INPUT_INSTANCE: Mutex<Input> = Mutex::new(Input::new());
// }
#[derive(Clone, PartialEq)]
pub struct Input {
    pub A: bool,
    pub B: bool,
    pub Select: bool,
    pub Start: bool,
    pub Right: bool,
    pub Left: bool,
    pub Up: bool,
    pub Down: bool,
}

impl Input {
    pub fn new() -> Self {
        Input {
            A: false,
            B: false,
            Select: false,
            Start: false,
            Right: false,
            Left: false,
            Up: false,
            Down: false,
        }
    }
}
pub struct InputManager {
    pub d_pad_mode: bool,
    pub button_mode: bool,
    pub last_input: Input,
}
impl InputManager {
    pub fn new() -> Self {
        InputManager {
            d_pad_mode: false,
            button_mode: false,
            last_input: Input::new(),
        }
    }

    pub fn set_mode(&mut self, mode: u8) {
        self.button_mode = mode & 0x20 == 0;
        self.d_pad_mode = mode & 0x10 == 0;
    }

    pub fn gamepad_get_output(&self) -> u8 {
        let mut output = 0xCF;

        if self.button_mode {
            if self.last_input.Start {
                output &= !(1 << 3);
            }
            if self.last_input.Select {
                output &= !(1 << 2);
            }
            if self.last_input.A {
                output &= !1;
            }
            if self.last_input.B {
                output &= !(1 << 1);
            }
        }

        if self.d_pad_mode {
            if self.last_input.Left {
                output &= !(1 << 1);
            }
            if self.last_input.Right {
                output &= !1;
            }
            if self.last_input.Up {
                output &= !(1 << 2);
            }
            if self.last_input.Down {
                output &= !(1 << 3);
            }
        }

        return output;
    }
}
