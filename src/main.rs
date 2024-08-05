use std::{env, path::Path};
mod emulator;
mod libs;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: program <path_to_rom>");
    }

    let path = &args[1];
    if !Path::new(path).exists() {
        println!("File does not exist: {}", path);
    }
    // Initialize the emulator
    let mut emulator = emulator::Emulator::new(&args[1]);
    println!("{}", emulator.gb_engine.memory.cart.info.to_string());
    emulator.start()
}
