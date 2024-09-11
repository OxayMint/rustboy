use std::path::Path;

use super::cart_info::CartridgeInfo;

pub mod mbc0;
pub mod mbc1;
pub mod mbc2;
pub mod mbc3;
pub mod mbc5;
pub mod mbc6;
pub mod mbc7;

pub trait MBC {
    fn read(&self, address: usize) -> u8;
    fn write(&mut self, address: usize, value: u8);
    fn save_ram(&self);
}

pub enum MBCEnum {
    MBC0(mbc0::MBC0),
    MBC1(mbc1::MBC1),
    MBC2(mbc2::MBC2),
    MBC3(mbc3::MBC3),
    MBC5(mbc5::MBC5),
    MBC6(mbc6::MBC6),
    MBC7(mbc7::MBC7),
}

impl MBC for MBCEnum {
    // Implement the MBC trait methods here, delegating to the inner MBC type
    fn read(&self, address: usize) -> u8 {
        match self {
            MBCEnum::MBC0(mbc) => mbc.read(address),
            MBCEnum::MBC1(mbc) => mbc.read(address),
            MBCEnum::MBC2(mbc) => mbc.read(address),
            MBCEnum::MBC3(mbc) => mbc.read(address),
            MBCEnum::MBC5(mbc) => mbc.read(address),
            MBCEnum::MBC6(mbc) => mbc.read(address),
            MBCEnum::MBC7(mbc) => mbc.read(address),
        }
    }

    fn write(&mut self, address: usize, value: u8) {
        match self {
            MBCEnum::MBC0(mbc) => mbc.write(address, value),
            MBCEnum::MBC1(mbc) => mbc.write(address, value),
            MBCEnum::MBC2(mbc) => mbc.write(address, value),
            MBCEnum::MBC3(mbc) => mbc.write(address, value),
            MBCEnum::MBC5(mbc) => mbc.write(address, value),
            MBCEnum::MBC6(mbc) => mbc.write(address, value),
            MBCEnum::MBC7(mbc) => mbc.write(address, value),
        }
    }
    fn save_ram(&self) {
        match self {
            MBCEnum::MBC0(mbc) => mbc.save_ram(),
            MBCEnum::MBC1(mbc) => mbc.save_ram(),
            MBCEnum::MBC2(mbc) => mbc.save_ram(),
            MBCEnum::MBC3(mbc) => mbc.save_ram(),
            MBCEnum::MBC5(mbc) => mbc.save_ram(),
            MBCEnum::MBC6(mbc) => mbc.save_ram(),
            MBCEnum::MBC7(mbc) => mbc.save_ram(),
        }
    }

    // Implement other methods similarly
}

pub fn create_mbc(data: Vec<u8>, info: &CartridgeInfo) -> MBCEnum {
    println!("mbc{}", info.mbc_index);
    let mbc = match info.mbc_index {
        1 => MBCEnum::MBC1(mbc1::MBC1::from_data(data, &info)),
        2 => MBCEnum::MBC2(mbc2::MBC2::from_data(data, &info)),
        3 => MBCEnum::MBC3(mbc3::MBC3::from_data(data, &info)),
        5 => MBCEnum::MBC5(mbc5::MBC5::from_data(data, &info)),
        6 => MBCEnum::MBC6(mbc6::MBC6::from_data(data, &info)),
        7 => MBCEnum::MBC7(mbc7::MBC7::from_data(data, &info)),
        _ => MBCEnum::MBC0(mbc0::MBC0::from_data(data, &info)),
    };
    mbc
}
