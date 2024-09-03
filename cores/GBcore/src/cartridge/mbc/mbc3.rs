use std::fs;

use super::MBC;
use crate::cartridge::cart_info::CartridgeInfo;

pub struct MBC3 {
    data: Vec<u8>,
    rom_bank_count: usize,
    ram_bank_count: u8,
    rom_banks: Vec<Vec<u8>>,
    ram_banks: Option<Vec<Vec<u8>>>,
    rtc_registers: [u8; 5], // 5 registers for RTC
    current_rom_bank: usize,
    current_ram_bank: usize,
    ram_enabled: bool,
    rtc_latched: bool,
    rtc_selected: bool,
    rtc_enabled: bool,
}

impl MBC3 {
    pub fn from_data(data: Vec<u8>, info: &CartridgeInfo) -> Self {
        let rom_banks = MBC3::create_rom_banks(&data, info);
        let ram_banks = MBC3::create_ram_banks(info);

        MBC3 {
            data,
            rom_bank_count: info.rom_bank_count,
            ram_bank_count: info.ram_bank_count,
            rom_banks,
            ram_banks,
            rtc_registers: [0; 5],
            current_rom_bank: 1, // MBC3 uses ROM bank 1 as the starting bank
            current_ram_bank: 0,
            ram_enabled: false,
            rtc_latched: false,
            rtc_selected: false,
            rtc_enabled: false,
        }
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

    // Latch the RTC (for reading)
    fn latch_rtc(&mut self) {
        // Here we would normally capture the current time and store it in rtc_registers,
        // but we'll keep it simple for now and just simulate it.
        self.rtc_latched = true;
    }

    // Handle the RTC registers (read/write)
    fn handle_rtc(&self, address: usize) -> u8 {
        let index = address - 0xA000;
        self.rtc_registers[index.min(4)] // Return the corresponding RTC register
    }

    fn write_rtc(&mut self, address: usize, value: u8) {
        let index = address - 0xA000;
        if index < 5 {
            self.rtc_registers[index] = value;
        }
    }
}

impl MBC for MBC3 {
    fn read(&self, address: usize) -> u8 {
        match address {
            // ROM reading
            0x0000..=0x3FFF => self.rom_banks[0][address], // Fixed bank 0
            0x4000..=0x7FFF => {
                let bank_index = self.current_rom_bank;
                self.rom_banks[bank_index][address - 0x4000]
            }
            // RAM or RTC reading
            0xA000..=0xBFFF => {
                if self.ram_enabled {
                    if self.rtc_selected {
                        return self.handle_rtc(address); // Handle RTC read
                    } else if let Some(ref ram_banks) = self.ram_banks {
                        let bank_index =
                            self.current_ram_bank.min(self.ram_bank_count as usize - 1);
                        return ram_banks[bank_index][address - 0xA000];
                    }
                }
                0xFF // Default value when RAM/RTC disabled
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
                let bank_number = (value & 0x7F) as usize;
                self.current_rom_bank = if bank_number == 0 { 1 } else { bank_number };
                // Bank 0 treated as 1
            }
            // RAM bank number / RTC register select (0x4000 - 0x5FFF)
            0x4000..=0x5FFF => {
                if value <= 0x03 {
                    self.current_ram_bank = value as usize;
                    self.rtc_selected = false;
                } else if value >= 0x08 && value <= 0x0C {
                    self.rtc_selected = true;
                    self.rtc_enabled = true;
                }
            }
            // Latch clock data (0x6000 - 0x7FFF)
            0x6000..=0x7FFF => {
                if value == 1 {
                    self.latch_rtc();
                }
            }
            // RAM or RTC writing
            0xA000..=0xBFFF => {
                if self.ram_enabled {
                    if self.rtc_selected {
                        self.write_rtc(address, value); // Handle RTC write
                    } else if let Some(ref mut ram_banks) = self.ram_banks {
                        let bank_index =
                            self.current_ram_bank.min(self.ram_bank_count as usize - 1);
                        ram_banks[bank_index][address - 0xA000] = value;
                    }
                }
            }
            _ => {} // Unhandled addresses
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
