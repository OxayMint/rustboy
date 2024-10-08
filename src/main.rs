use std::{env, path::Path};
use GBcore::GBCore;
// use crate::

// #[macro_use]
// extern crate lazy_static;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: program <path_to_rom>");
        return;
    }
    let path = &args[1];
    // let path = "/Users/fgoja/dev/rust/rustboy/roms/tetris.gb";
    if !Path::new(path).exists() {
        println!("File does not exist: {}", path);
        return;
    }
    // Initialize the emulator
    let mut emulator = GBCore::new();
    emulator.start(path);
}
