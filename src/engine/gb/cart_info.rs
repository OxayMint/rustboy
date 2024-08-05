//from pandocs
/*
MBC Timing Issues
Among Nintendo MBCs, only the MBC5 is guaranteed by Nintendo to support
the tighter timing of CGB Double Speed Mode. There have been rumours that older MBCs
(like MBC1-3) wouldn’t be fast enough in that mode. If so, it might be nevertheless possible
to use Double Speed during periods which use only code and data which is located in internal RAM.
Despite the above, a self-made MBC1-EPROM card appears to work stable and fine even in Double Speed Mode.
*/
pub struct CartridgeInfo {
    pub cart_type: u8,
    pub title: String,
    pub licensee: String,
    pub rom_size: usize,
    pub rom: bool,
    pub mbc_index: u8, // if 0 then this is non-MBC rom.
    pub ram: bool,
    pub battery: bool,
    pub mmm_01: bool,
    pub timer: bool,
    pub huc_index: u8,
    pub bandai_tama: bool,
    pub pocket_camera: bool,
    pub sensor: bool,
    pub rumble: bool,
}

impl CartridgeInfo {
    pub fn to_string(&self) -> String {
        if self.cart_type == 0x00 {
            return "ROM ONLY".to_string();
        }
        let mut items: Vec<String> = vec![];

        if self.mbc_index > 0 {
            let st = format!("MBC{}", self.mbc_index);
            items.push(st);
        }
        if self.huc_index > 0 {
            let st = format!("HuC{}", self.huc_index);
            items.push(st);
        }

        if self.mmm_01 {
            items.push("MMM01".to_string());
        }

        if self.rom {
            items.push("ROM".to_string());
        }

        if self.sensor {
            items.push("SENSOR".to_string());
        }
        if self.rumble {
            items.push("RUMBLE".to_string());
        }

        if self.timer {
            items.push("TIMER".to_string());
        }
        if self.ram {
            items.push("RAM".to_string());
        }
        if self.battery {
            items.push("BATTERY".to_string());
        }
        if self.pocket_camera {
            items.push("POCKET CAMERA".to_string());
        }
        if self.bandai_tama {
            items.push("BANDAI TAMA5".to_string());
        }
        return format!(
            "Title: {0},\nLicensee: ({1})\nROM size: {4} KB\nCart type: {2:0X} ({3})",
            self.title,
            self.licensee,
            self.cart_type,
            items.join("+"),
            self.rom_size / 1024
        );
    }
}
