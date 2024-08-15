use std::{env, path::Path};

use libs::gameboy::GameBoyEngine;

mod emulator;
mod libs;
#[macro_use]
extern crate lazy_static;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: program <path_to_rom>");
    }
    let path = &args[1];

    // let path = "/Users/fgoja/dev/rust/rustboy/roms/dmg-acid2.gb";
    // let path = "/Users/fgoja/dev/rust/rustboy/roms/drmario.gb";
    // let path = "/Users/fgoja/dev/rust/rustboy/roms/tetris.gb";
    // let path = "/Users/fgoja/dev/rust/rustboy/roms/asteroids.gb";
    if !Path::new(path).exists() {
        println!("File does not exist: {}", path);
    }
    // Initialize the emulator
    let mut emulator = GameBoyEngine::new(path);
    // println!("{}", emulator.gb_engine.memory.cart.info.to_string());
    emulator.start();
}
