use std::{
    any::{Any, TypeId},
    fs,
};
mod cart_info;
mod mbc;
use cart_info::CartridgeInfo;
use mbc::{create_mbc, MBCEnum, MBC};
pub struct Cartridge {
    pub info: CartridgeInfo,
    mbc: MBCEnum,
    // pub data: Vec<u8>,
    // pub rom_banks: Vec<Vec<u8>>,
    // pub ram_banks: Vec<Vec<u8>>,
    // pub current_rom_bank: usize,
    // pub current_ram_bank: usize,
    // pub ram_enabled: bool,
    // save_path: String,
}

impl Cartridge {
    pub fn from_path(path: &str) -> Result<Cartridge, String> {
        let cart_data = Cartridge::cart_load(&path);
        if let Some(unwrapped_data) = cart_data {
            match CartridgeInfo::from_data(path, &unwrapped_data) {
                Ok(info) => {
                    let mbc = create_mbc(unwrapped_data, &info);

                    let cartridge = Cartridge { info, mbc };

                    Ok(cartridge)
                }
                Err(err) => Err(err),
            }
        } else {
            Err("Couldn't load ROM file".to_string())
        }
    }

    fn cart_load(path: &str) -> Option<Vec<u8>> {
        match fs::read(path) {
            Ok(data) => Some(data),
            Err(_) => None,
        }
    }

    pub fn save_ram(&self) {
        self.mbc.save_ram();
    }

    pub fn read(&self, address: usize) -> u8 {
        self.mbc.read(address)
    }
    pub fn write(&mut self, address: usize, value: u8) {
        self.mbc.write(address, value)
    }
}
// fn get_mbc(cartridge_data: Vec<u8>, info: &CartridgeInfo) -> Box<MBC> {
//     match info.mbc_index {
//         1 => Box::new(MBC1::from_data(cartridge_data)),
//         2 => Box::new(MBC2::from_data(cartridge_data)),
//         3 => Box::new(MBC3::from_data(cartridge_data)),
//         4 => Box::new(MBC4::from_data(cartridge_data)),
//         5 => Box::new(MBC5::from_data(cartridge_data)),
//         _ => Box::new(MBC0::from_data(cartridge_data)),
//     }
// }
