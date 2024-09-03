use super::MBC;
use crate::cartridge::cart_info::CartridgeInfo;
use std::path::Path;

pub struct MBC5 {
    data: Vec<u8>,
    rom_size: usize,
    rom_bank_count: usize,
    ram_bank_count: u8,
    ram_size: usize,
    ram_enabled: bool,
    battery: bool,
    rom_banks: Vec<Vec<u8>>,
    ram_banks: Option<Vec<Vec<u8>>>,
    current_rom_bank: usize,
    current_ram_bank: usize,
}

impl MBC5 {
    pub fn from_data(data: Vec<u8>, info: &CartridgeInfo) -> Self {
        let rom_banks = MBC5::create_rom_banks(&data, info);
        let ram_banks = MBC5::create_ram_banks(info);

        MBC5 {
            data,
            rom_size: info.rom_size,
            rom_bank_count: info.rom_bank_count,
            ram_size: info.ram_size,
            ram_bank_count: info.ram_bank_count,
            battery: info.battery,
            current_ram_bank: 0,
            current_rom_bank: 1,
            ram_banks,
            rom_banks,
            ram_enabled: false,
        }
    }

    pub fn load_saved_ram(&mut self, data: Vec<u8>) {
        if !self.battery {
            println!("Non-battery cartridges cannot load saves");
            return;
        }
        if let Some(ram_banks) = &mut self.ram_banks {
            let expected_size = self.ram_size;
            if data.len() != expected_size {
                println!("Warning: Loaded RAM data size does not match expected size");
                return;
            }
            let bank_size = 8 * 1024;
            for (i, bank) in ram_banks.iter_mut().enumerate() {
                let start = i * bank_size;
                let end = start + bank_size;
                bank.copy_from_slice(&data[start..end]);
            }
        }
    }

    fn create_rom_banks(data: &[u8], info: &CartridgeInfo) -> Vec<Vec<u8>> {
        let bank_size = 16 * 1024; // 16 KB per bank
        (0..info.rom_bank_count)
            .map(|i| {
                let bank_start = i * bank_size;
                let bank_end = bank_start + bank_size;
                data[bank_start..bank_end].to_vec()
            })
            .collect()
    }

    fn create_ram_banks(info: &CartridgeInfo) -> Option<Vec<Vec<u8>>> {
        if info.ram_size > 0 {
            Some(vec![vec![0; 8 * 1024]; info.ram_bank_count as usize])
        } else {
            None
        }
    }

    fn set_rom_bank_low(&mut self, value: u8) {
        self.current_rom_bank = (self.current_rom_bank & 0x100) | (value as usize);
    }

    fn set_rom_bank_high(&mut self, value: u8) {
        self.current_rom_bank = (self.current_rom_bank & 0xFF) | ((value as usize & 0x01) << 8);
    }

    fn set_ram_bank(&mut self, value: u8) {
        self.current_ram_bank = (value & 0x0F) as usize;
    }
}

impl MBC for MBC5 {
    fn read(&self, address: usize) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom_banks[0][address],
            0x4000..=0x7FFF => {
                let bank = self.current_rom_bank % self.rom_bank_count;
                let offset = address - 0x4000;
                self.rom_banks
                    .get(bank)
                    .and_then(|b| b.get(offset))
                    .copied()
                    .unwrap_or(0xFF)
            }
            0xA000..=0xBFFF => {
                if !self.ram_enabled {
                    return 0xFF;
                }
                if let Some(ram_banks) = &self.ram_banks {
                    let bank = self.current_ram_bank % self.ram_bank_count as usize;
                    let offset = address - 0xA000;
                    ram_banks
                        .get(bank)
                        .and_then(|b| b.get(offset))
                        .copied()
                        .unwrap_or(0xFF)
                } else {
                    0xFF
                }
            }
            _ => 0xFF,
        }
    }

    fn write(&mut self, address: usize, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram_enabled = (value & 0x0F) == 0x0A,
            0x2000..=0x2FFF => self.set_rom_bank_low(value),
            0x3000..=0x3FFF => self.set_rom_bank_high(value),
            0x4000..=0x5FFF => self.set_ram_bank(value),
            0xA000..=0xBFFF => {
                if !self.ram_enabled {
                    return;
                }
                if let Some(ram_banks) = &mut self.ram_banks {
                    let bank = self.current_ram_bank % self.ram_bank_count as usize;
                    let offset = address - 0xA000;
                    if let Some(cell) = ram_banks.get_mut(bank).and_then(|b| b.get_mut(offset)) {
                        *cell = value;
                    }
                }
            }
            _ => {}
        }
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
