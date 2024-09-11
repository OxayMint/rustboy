use super::MBC;
use crate::cartridge::cart_info::CartridgeInfo;
use std::{
    env, fs,
    ops::{AddAssign, SubAssign},
    path::{Path, PathBuf},
};
static SAVE_SKIPS: u8 = 20;
pub struct MBC1 {
    data: Vec<u8>,
    rom_size: usize,
    rom_bank_count: usize,
    ram_bank_count: u8,
    ram_size: usize,
    ram_enabled: bool,
    battery: bool,
    banking_mode: u8,
    ram_banking: bool,
    rom_banks: Vec<Vec<u8>>,
    ram_banks: Option<Vec<Vec<u8>>>,
    current_rom_bank_index: usize,
    current_ram_bank_index: usize,
    rom_bank_mask: usize,
    upper_bits: usize,
    save_path: String,
    current_save_skips: u8,
}

impl MBC1 {
    pub fn from_data(data: Vec<u8>, info: &CartridgeInfo) -> Self {
        let rom_banks = MBC1::create_rom_banks(&data, info);
        let ram_banks = MBC1::create_ram_banks(info);
        let rom_bank_mask =
            ((info.rom_bank_count - 1).next_power_of_two() - 1).min(info.rom_bank_count - 1);

        let mut mbc = MBC1 {
            data,
            rom_size: info.rom_size,
            rom_bank_count: info.rom_bank_count,
            ram_size: info.ram_size,
            ram_bank_count: info.ram_bank_count,
            battery: info.battery,
            current_ram_bank_index: 0,
            current_rom_bank_index: 1,
            banking_mode: 0,
            ram_banking: false,
            ram_banks,
            rom_banks,
            rom_bank_mask,
            ram_enabled: false,
            upper_bits: 0,
            save_path: Self::get_save_path(info),
            current_save_skips: 0,
        };
        mbc.load_saved_ram();
        return mbc;
    }
    fn get_save_path(info: &CartridgeInfo) -> String {
        let file_path = Path::new(&info.path);
        let file_stem = file_path.file_stem().unwrap().to_str().unwrap();
        let save_path = Path::new(&info.path)
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join(format!("saves/{}.sav", file_stem));

        return save_path.to_str().unwrap().to_string();
    }

    fn create_rom_banks(data: &[u8], info: &CartridgeInfo) -> Vec<Vec<u8>> {
        let mut banks = Vec::new();
        for i in 0..info.rom_bank_count {
            let start = i * 0x4000;
            let end = start + 0x4000;
            banks.push(data[start..end].to_vec());
        }
        banks
    }

    fn create_ram_banks(info: &CartridgeInfo) -> Option<Vec<Vec<u8>>> {
        if info.ram_size == 0 {
            return None;
        }

        Some(vec![vec![0; 0x2000]; info.ram_bank_count as usize])
    }
    // let ram_data = .create_ram_save();
    // if let Err(e) = fs::write(&self.save_path, ram_data) {
    //     eprintln!("Failed to save RAM: {}", e);
    // }
    pub fn load_saved_ram(&mut self) {
        if !self.battery {
            println!("Non-battery cartridges cannot be loaded");
            return;
        }
        if !self.ram_banks.is_some() {
            println!("No ram to load");
            return;
        }
        if let Ok(data) = fs::read(&self.save_path) {
            let mut ram_banks_inner = self.ram_banks.take().unwrap();
            let bank_size = ram_banks_inner[0].len();

            for (i, bank) in ram_banks_inner.iter_mut().enumerate() {
                let start = i * bank_size;
                let end = start + bank_size;
                if end <= data.len() {
                    bank.copy_from_slice(&data[start..end]);
                }
            }

            self.ram_banks = Some(ram_banks_inner);
        } else {
            println!("No file to load")
        }
    }
}

impl MBC for MBC1 {
    fn read(&self, address: usize) -> u8 {
        match address {
            // ROM reading
            0x0000..=0x3FFF => self.rom_banks[0][address], // Fixed bank 0
            0x4000..=0x7FFF => {
                let bank_index = self.current_rom_bank_index & self.rom_bank_mask;
                self.rom_banks[bank_index][address - 0x4000]
            }
            // RAM reading
            0xA000..=0xBFFF => {
                if self.ram_enabled {
                    if let Some(ref ram_banks) = self.ram_banks {
                        let bank_index = self
                            .current_ram_bank_index
                            .min(self.ram_bank_count as usize - 1);
                        return ram_banks[bank_index][address - 0xA000];
                    }
                }
                0xFF // Default return value when RAM is disabled or non-existent
            }
            _ => 0xFF, // Unmapped addresses return 0xFF
        }
    }

    fn write(&mut self, address: usize, value: u8) {
        match address {
            // RAM enable/disable (0x0000 - 0x1FFF)
            0x0000..=0x1FFF => {
                self.ram_enabled = (value & 0x0F) == 0x0A;
            }
            // ROM bank number (0x2000 - 0x3FFF)
            0x2000..=0x3FFF => {
                let bank_number = (value & 0x1F) as usize;
                let bank_number = if bank_number == 0 { 1 } else { bank_number }; // Bank 0 treated as 1
                self.current_rom_bank_index = (self.current_rom_bank_index & !0x1F) | bank_number;
            }
            // RAM bank number / upper bits of ROM bank number (0x4000 - 0x5FFF)
            0x4000..=0x5FFF => {
                let upper_bits = (value & 0x03) as usize;
                self.upper_bits = upper_bits;
                if self.banking_mode == 1 {
                    self.current_ram_bank_index = upper_bits;
                } else {
                    self.current_rom_bank_index =
                        (self.current_rom_bank_index & 0x1F) | (upper_bits << 5);
                }
            }
            // Banking mode select (0x6000 - 0x7FFF)
            0x6000..=0x7FFF => {
                self.banking_mode = value & 0b1;
            }
            // RAM writing (0xA000 - 0xBFFF)
            0xA000..=0xBFFF => {
                if self.ram_enabled {
                    if let Some(ref mut ram_banks) = self.ram_banks {
                        let bank_index = self
                            .current_ram_bank_index
                            .min(self.ram_bank_count as usize - 1);
                        ram_banks[bank_index][address - 0xA000] = value;
                    }
                    if !self.battery {
                        return;
                    }
                    self.current_save_skips += 1;
                    if self.current_save_skips >= SAVE_SKIPS {
                        self.save_ram();
                        self.current_save_skips = 0;
                    }
                }
            }
            _ => {} // Unhandled addresses
        }
    }

    fn save_ram(&self) {
        if !self.battery {
            println!("Non-battery cartridges cannot be saved");
            return;
        }

        if !self.ram_banks.is_some() {
            println!("No ram to load");
            return;
        }
        let save_data: Vec<u8> = self
            .ram_banks
            .as_ref()
            .map(|ram_banks| ram_banks.iter().flatten().copied().collect())
            .unwrap();
        if let Err(e) = fs::write(&self.save_path, save_data) {
            eprintln!("Failed to save RAM: {}", e);
        }
    }
}
// use super::MBC;
// use crate::cartridge::cart_info::CartridgeInfo;
// use std::path::Path;

// pub struct MBC1 {
//     data: Vec<u8>,
//     rom_size: usize,
//     rom_bank_count: usize,
//     ram_bank_count: u8,
//     ram_size: usize,
//     ram_enabled: bool,
//     battery: bool,
//     banking_mode: u8,
//     ram_banking: bool,
//     rom_banks: Vec<Vec<u8>>,
//     ram_banks: Option<Vec<Vec<u8>>>,
//     current_rom_bank_index: usize,
//     current_ram_bank_index: usize,
//     rom_bank_mask: usize,
//     upper_bits: usize,
// }

// impl MBC1 {
//     pub fn from_data(data: Vec<u8>, info: &CartridgeInfo) -> Self {
//         let rom_banks = MBC1::create_rom_banks(&data, info);
//         let ram_banks = MBC1::create_ram_banks(info);
//         let rom_bank_mask =
//             ((info.rom_bank_count - 1).next_power_of_two() - 1).min(info.rom_bank_count - 1);

//         MBC1 {
//             data,
//             rom_size: info.rom_size,
//             rom_bank_count: info.rom_bank_count,
//             ram_size: info.ram_size,
//             ram_bank_count: info.ram_bank_count,
//             battery: info.battery,
//             current_ram_bank_index: 0,
//             current_rom_bank_index: 1,
//             banking_mode: 0,
//             ram_banking: false,
//             ram_banks,
//             rom_banks,
//             rom_bank_mask,
//             ram_enabled: false,
//             upper_bits: 0,
//         }
//     }

//     pub fn create_ram_save(&self) -> Option<Vec<u8>> {
//         if !self.battery {
//             println!("Non-battery cartridges cannot be saved");
//             return None;
//         }
//         self.ram_banks
//             .as_ref()
//             .map(|ram_banks| ram_banks.iter().flatten().copied().collect())
//     }

//     pub fn load_saved_ram(&mut self, data: Vec<u8>) {
//         if !self.battery {
//             println!("Non-battery cartridges cannot load saves");
//             return;
//         }
//         if let Some(ram_banks) = &mut self.ram_banks {
//             let expected_size = self.ram_size;
//             if data.len() != expected_size {
//                 println!("Warning: Loaded RAM data size does not match expected size");
//                 return;
//             }
//             let bank_size = 8 * 1024;
//             for (i, bank) in ram_banks.iter_mut().enumerate() {
//                 let start = i * bank_size;
//                 let end = start + bank_size;
//                 bank.copy_from_slice(&data[start..end]);
//             }
//         }
//     }

//     fn create_rom_banks(data: &[u8], info: &CartridgeInfo) -> Vec<Vec<u8>> {
//         let bank_size = 16 * 1024; // 16 KB per bank
//         (0..info.rom_bank_count)
//             .map(|i| {
//                 let bank_start = i * bank_size;
//                 let bank_end = bank_start + bank_size;
//                 data[bank_start..bank_end].to_vec()
//             })
//             .collect()
//     }

//     fn create_ram_banks(info: &CartridgeInfo) -> Option<Vec<Vec<u8>>> {
//         if info.ram_size > 0 {
//             Some(vec![vec![0; 8 * 1024]; info.ram_bank_count as usize])
//         } else {
//             None
//         }
//     }

//     fn set_rom_bank(&mut self, bank: u8) {
//         let mut bank = bank as usize;
//         if bank == 0 || bank == 0x20 || bank == 0x40 || bank == 0x60 {
//             bank += 1;
//         }
//         bank &= self.rom_bank_mask;
//         if self.banking_mode == 0 {
//             bank |= self.upper_bits;
//         }
//         self.current_rom_bank_index = bank;
//     }

//     fn set_ram_bank(&mut self, bank: u8) {
//         if self.banking_mode == 1 {
//             self.current_ram_bank_index = (bank & 0x03) as usize;
//         } else {
//             self.upper_bits = (bank as usize & 0x03) << 5;
//         }
//     }

//     fn set_banking_mode(&mut self, mode: u8) {
//         self.banking_mode = mode & 1;
//         if self.banking_mode == 0 {
//             self.current_ram_bank_index = 0;
//             self.current_rom_bank_index = (self.current_rom_bank_index & 0x1F) | self.upper_bits;
//         } else {
//             self.upper_bits = self.current_rom_bank_index & 0x60;
//             self.current_rom_bank_index &= 0x1F;
//         }
//     }
// }

// impl MBC for MBC1 {
//     fn read(&self, address: usize) -> u8 {
//         match address {
//             0x0000..=0x3FFF => self.rom_banks[0][address],
//             0x4000..=0x7FFF => {
//                 let bank = self.current_rom_bank_index;
//                 let offset = address - 0x4000;
//                 self.rom_banks
//                     .get(bank)
//                     .and_then(|b| b.get(offset))
//                     .copied()
//                     .unwrap_or(0xFF)
//             }
//             0xA000..=0xBFFF => {
//                 if !self.ram_enabled {
//                     return 0xFF;
//                 }
//                 if let Some(ram_banks) = &self.ram_banks {
//                     let bank = self.current_ram_bank_index;
//                     let offset = address - 0xA000;
//                     ram_banks
//                         .get(bank)
//                         .and_then(|b| b.get(offset))
//                         .copied()
//                         .unwrap_or(0xFF)
//                 } else {
//                     0xFF
//                 }
//             }
//             _ => 0xFF,
//         }
//     }

//     fn write(&mut self, address: usize, value: u8) {
//         match address {
//             0x0000..=0x1FFF => self.ram_enabled = (value & 0x0F) == 0x0A,
//             0x2000..=0x3FFF => self.set_rom_bank(value),
//             0x4000..=0x5FFF => self.set_ram_bank(value),
//             0x6000..=0x7FFF => self.set_banking_mode(value),
//             0xA000..=0xBFFF => {
//                 if !self.ram_enabled {
//                     return;
//                 }
//                 if let Some(ram_banks) = &mut self.ram_banks {
//                     let bank = self.current_ram_bank_index;
//                     let offset = address - 0xA000;
//                     if let Some(cell) = ram_banks.get_mut(bank).and_then(|b| b.get_mut(offset)) {
//                         *cell = value;
//                     }
//                 }
//             }
//             _ => {}
//         }
//     }
// }
