use crate::cartridge::cart_info::CartridgeInfo;

use super::MBC;

pub struct MBC2 {
    data: Vec<u8>,
}

impl MBC2 {
    pub fn from_data(data: Vec<u8>, info: &CartridgeInfo) -> Self {
        return MBC2 { data: data };
    }
}
impl MBC for MBC2 {
    fn read(&self, address: usize) -> u8 {
        todo!()
    }

    fn write(&mut self, address: usize, value: u8) {
        todo!()
    }
    fn save_ram(&self) {
        // if !self.battery {
        //     println!("Non-battery cartridges cannot be saved");
        //     return;
        // }

        // if !self.ram_banks.is_some() {
        //     println!("No ram to load");
        //     return;
        // }
        // let save_data: Vec<u8> = self
        //     .ram_banks
        //     .as_ref()
        //     .map(|ram_banks| ram_banks.iter().flatten().copied().collect())
        //     .unwrap();
        // if let Err(e) = fs::write(&self.save_path, save_data) {
        //     eprintln!("Failed to save RAM: {}", e);
        // }
    }
}
