use crate::cartridge::cart_info::CartridgeInfo;

use super::MBC;

pub struct MBC0 {
    data: Vec<u8>,
}
impl MBC0 {
    pub fn from_data(data: Vec<u8>, info: &CartridgeInfo) -> Self {
        return MBC0 { data: data };
    }
}
impl MBC for MBC0 {
    fn read(&self, address: usize) -> u8 {
        return self.data[address];
    }

    fn write(&mut self, address: usize, value: u8) {
        self.data[address] = value;
    }
    fn save_ram(&self) {}
}
