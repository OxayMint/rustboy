use lazy_static::lazy_static;

use super::bus::Bus;

pub struct EmuDebug {
    dbg_msg_bytes: Vec<u8>,
    should_print: bool,
}

impl EmuDebug {
    pub fn new() -> Self {
        EmuDebug {
            dbg_msg_bytes: Vec::new(),
            should_print: false,
        }
    }

    pub fn update(&mut self) {
        if Bus::read(0xFF02) == 0b10000001 {
            // print!("Emu Debug: ");
            let character = Bus::read8(0xFF01);
            self.dbg_msg_bytes.push(character);
            Bus::write8(0xFF02, 0);
            print!("{}", character as char);
            self.should_print = true;
            return;
        }
        if self.should_print {
            // self.print();
            self.should_print = false;
        }
    }

    pub fn print(&mut self) {
        // if self.dbg_msg_bytes.len() > 0 {
        //     println!(
        //         "EmuDebug: {}",
        //         String::from_utf8(self.dbg_msg_bytes.to_vec()).unwrap()
        //     );
        // }
    }
}

lazy_static! {
    pub static ref EMU_DEBUG: Mutex<EmuDebug> = Mutex::new(EmuDebug::new());
}
