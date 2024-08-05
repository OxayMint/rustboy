#[path = "cart_info.rs"]
mod cart_info;
use std::{collections::HashMap, fs};

use cart_info::CartridgeInfo;

pub struct Cartridge {
    pub info: CartridgeInfo,
    pub data: Vec<u8>,
}

impl Cartridge {
    pub fn from_path(path: &str) -> Result<Cartridge, String> {
        let cart_data = Cartridge::cart_load(path);
        if cart_data.is_some() {
            let unwrapped_data = cart_data.unwrap();
            match Cartridge::get_info(&unwrapped_data) {
                Ok(info) => Ok(Cartridge {
                    info: info,
                    data: unwrapped_data,
                }),
                Err(err) => Err(err),
            }
        } else {
            Err("Couldn't load ROM file".to_string())
        }
    }
    fn cart_load(path: &str) -> Option<Vec<u8>> {
        // Load the ROM file into memory
        match fs::read(path) {
            Ok(data) => Some(data),
            Err(_) => None,
        }
    }

    pub fn read(&self, address: usize) -> u8 {
        // return &0;
        return self.data[address.clone()];
    }
    pub fn write(&self, address: usize, value: u8) {}

    fn check_header_checksum(card: &[u8]) -> bool {
        let mut checksum: u8 = 0;
        for n in &card[0x0134..0x014d] {
            checksum = checksum.wrapping_sub(*n).wrapping_sub(1);
        }
        return checksum == card[0x014D];
    }

    fn get_info(card: &[u8]) -> Result<cart_info::CartridgeInfo, String> {
        match Cartridge::check_header_checksum(card) {
            true => println!("Checksum ok"),
            false => println!("Checksum wrong"),
        }
        let new_licensee_tuples: [(u16, &str); 64] = [
            (0x3030, "None"),
            (0x3031, "Nintendo Research & Development 1"),
            (0x3038, "Capcom"),
            (0x3133, "EA (Electronic Arts)"),
            (0x3138, "Hudson Soft"),
            (0x3139, "B-AI"),
            (0x3230, "KSS"),
            (0x3232, "Planning Office WADA"),
            (0x3234, "PCM Complete"),
            (0x3235, "San-X"),
            (0x3238, "Kemco"),
            (0x3239, "SETA Corporation"),
            (0x3330, "Viacom"),
            (0x3331, "Nintendo"),
            (0x3332, "Bandai"),
            (0x3333, "Ocean Software/Acclaim Entertainment"),
            (0x3334, "Konami"),
            (0x3335, "HectorSoft"),
            (0x3337, "Taito"),
            (0x3338, "Hudson Soft"),
            (0x3339, "Banpresto"),
            (0x3431, "Ubi Soft1"),
            (0x3432, "Atlus"),
            (0x3434, "Malibu Interactive"),
            (0x3436, "Angel"),
            (0x3437, "Bullet-Proof Software2"),
            (0x3439, "Irem"),
            (0x3530, "Absolute"),
            (0x3531, "Acclaim Entertainment"),
            (0x3532, "Activision"),
            (0x3533, "Sammy USA Corporation"),
            (0x3534, "Konami"),
            (0x3535, "Hi Tech Expressions"),
            (0x3536, "LJN"),
            (0x3537, "Matchbox"),
            (0x3538, "Mattel"),
            (0x3539, "Milton Bradley Company"),
            (0x3630, "Titus Interactive"),
            (0x3631, "Virgin Games Ltd.3"),
            (0x3634, "Lucasfilm Games4"),
            (0x3637, "Ocean Software"),
            (0x3639, "EA (Electronic Arts)"),
            (0x3730, "Infogrames5"),
            (0x3731, "Interplay Entertainment"),
            (0x3732, "Broderbund"),
            (0x3733, "Sculptured Software6"),
            (0x3735, "The Sales Curve Limited7"),
            (0x3738, "THQ"),
            (0x3739, "Accolade"),
            (0x3830, "Misawa Entertainment"),
            (0x3833, "lozc"),
            (0x3836, "Tokuma Shoten"),
            (0x3837, "Tsukuda Original"),
            (0x3931, "Chunsoft Co.8"),
            (0x3932, "Video System"),
            (0x3933, "Ocean Software/Acclaim Entertainment"),
            (0x3935, "Varie"),
            (0x3936, "Yonezawa/s’pal"),
            (0x3937, "Kaneko"),
            (0x3939, "Pack-In-Video"),
            (0x3948, "Bottom Up"),
            (0x4134, "Konami (Yu-Gi-Oh!)"),
            (0x424C, "MTO"),
            (0x444B, "Kodansha"),
        ];
        let old_licensee_tuples = [
            (0x00, "None"),
            (0x01, "Nintendo"),
            (0x08, "Capcom"),
            (0x09, "HOT-B"),
            (0x0A, "Jaleco"),
            (0x0B, "Coconuts Japan"),
            (0x0C, "Elite Systems"),
            (0x13, "EA (Electronic Arts)"),
            (0x18, "Hudson Soft"),
            (0x19, "ITC Entertainment"),
            (0x1A, "Yanoman"),
            (0x1D, "Japan Clary"),
            (0x1F, "Virgin Games Ltd.3"),
            (0x24, "PCM Complete"),
            (0x25, "San-X"),
            (0x28, "Kemco"),
            (0x29, "SETA Corporation"),
            (0x30, "Infogrames5"),
            (0x31, "Nintendo"),
            (0x32, "Bandai"),
            (0x33, "NEW_LICENSEE"),
            (0x34, "Konami"),
            (0x35, "HectorSoft"),
            (0x38, "Capcom"),
            (0x39, "Banpresto"),
            (0x3C, ".Entertainment i"),
            (0x3E, "Gremlin"),
            (0x41, "Ubi Soft1"),
            (0x42, "Atlus"),
            (0x44, "Malibu Interactive"),
            (0x46, "Angel"),
            (0x47, "Spectrum Holoby"),
            (0x49, "Irem"),
            (0x4A, "Virgin Games Ltd.3"),
            (0x4D, "Malibu Interactive"),
            (0x4F, "U.S. Gold"),
            (0x50, "Absolute"),
            (0x51, "Acclaim Entertainment"),
            (0x52, "Activision"),
            (0x53, "Sammy USA Corporation"),
            (0x54, "GameTek"),
            (0x55, "Park Place"),
            (0x56, "LJN"),
            (0x57, "Matchbox"),
            (0x59, "Milton Bradley Company"),
            (0x5A, "Mindscape"),
            (0x5B, "Romstar"),
            (0x5C, "Naxat Soft13"),
            (0x5D, "Tradewest"),
            (0x60, "Titus Interactive"),
            (0x61, "Virgin Games Ltd.3"),
            (0x67, "Ocean Software"),
            (0x69, "EA (Electronic Arts)"),
            (0x6E, "Elite Systems"),
            (0x6F, "Electro Brain"),
            (0x70, "Infogrames5"),
            (0x71, "Interplay Entertainment"),
            (0x72, "Broderbund"),
            (0x73, "Sculptured Software6"),
            (0x75, "The Sales Curve Limited7"),
            (0x78, "THQ"),
            (0x79, "Accolade"),
            (0x7A, "Triffix Entertainment"),
            (0x7C, "Microprose"),
            (0x7F, "Kemco"),
            (0x80, "Misawa Entertainment"),
            (0x83, "Lozc"),
            (0x86, "Tokuma Shoten"),
            (0x8B, "Bullet-Proof Software2"),
            (0x8C, "Vic Tokai"),
            (0x8E, "Ape"),
            (0x8F, "I’Max"),
            (0x91, "Chunsoft Co.8"),
            (0x92, "Video System"),
            (0x93, "Tsubaraya Productions"),
            (0x95, "Varie"),
            (0x96, "Yonezawa/S’Pal"),
            (0x97, "Kemco"),
            (0x99, "Arc"),
            (0x9A, "Nihon Bussan"),
            (0x9B, "Tecmo"),
            (0x9C, "Imagineer"),
            (0x9D, "Banpresto"),
            (0x9F, "Nova"),
            (0xA1, "Hori Electric"),
            (0xA2, "Bandai"),
            (0xA4, "Konami"),
            (0xA6, "Kawada"),
            (0xA7, "Takara"),
            (0xA9, "Technos Japan"),
            (0xAA, "Broderbund"),
            (0xAC, "Toei Animation"),
            (0xAD, "Toho"),
            (0xAF, "Namco"),
            (0xB0, "Acclaim Entertainment"),
            (0xB1, "ASCII Corporation or Nexsoft"),
            (0xB2, "Bandai"),
            (0xB4, "Square Enix"),
            (0xB6, "HAL Laboratory"),
            (0xB7, "SNK"),
            (0xB9, "Pony Canyon"),
            (0xBA, "Culture Brain"),
            (0xBB, "Sunsoft"),
            (0xBD, "Sony Imagesoft"),
            (0xBF, "Sammy Corporation"),
            (0xC0, "Taito"),
            (0xC2, "Kemco"),
            (0xC3, "Square"),
            (0xC4, "Tokuma Shoten"),
            (0xC5, "Data East"),
            (0xC6, "Tonkinhouse"),
            (0xC8, "Koei"),
            (0xC9, "UFL"),
            (0xCA, "Ultra"),
            (0xCB, "Vap"),
            (0xCC, "Use Corporation"),
            (0xCD, "Meldac"),
            (0xCE, "Pony Canyon"),
            (0xCF, "Angel"),
            (0xD0, "Taito"),
            (0xD1, "Sofel"),
            (0xD2, "Quest"),
            (0xD3, "Sigma Enterprises"),
            (0xD4, "ASK Kodansha Co."),
            (0xD6, "Naxat Soft13"),
            (0xD7, "Copya System"),
            (0xD9, "Banpresto"),
            (0xDA, "Tomy"),
            (0xDB, "LJN"),
            (0xDD, "NCS"),
            (0xDE, "Human"),
            (0xDF, "Altron"),
            (0xE0, "Jaleco"),
            (0xE1, "Towa Chiki"),
            (0xE2, "Yutaka"),
            (0xE3, "Varie"),
            (0xE5, "Epcoh"),
            (0xE7, "Athena"),
            (0xE8, "Asmik Ace Entertainment"),
            (0xE9, "Natsume"),
            (0xEA, "King Records"),
            (0xEB, "Atlus"),
            (0xEC, "Epic/Sony Records"),
            (0xEE, "IGS"),
            (0xF0, "A Wave"),
            (0xF3, "Extreme Entertainment"),
            (0xFF, "LJN"),
        ];
        let new_map = HashMap::from(new_licensee_tuples);
        let old_map = HashMap::from(old_licensee_tuples);

        let title: String;
        title = read_to_str(&card[0x0134..0x0143]);

        let licensee: &str;
        if card[0x014B] == 0x33 {
            licensee = new_map[&(((card[0x0144] as u16) << 8) | (card[0x0145] as u16))];
        } else {
            licensee = old_map[&card[0x014B]];
        }
        println!("licensee code {}", card[0x014B]);
        // $00	ROM ONLY
        let mbc_indecies = HashMap::from([
            (0x01, 1),
            (0x02, 1),
            (0x03, 1),
            (0x05, 2),
            (0x06, 2),
            (0x0F, 3),
            (0x10, 3),
            (0x11, 3),
            (0x12, 3),
            (0x13, 3),
            (0x19, 5),
            (0x1A, 5),
            (0x1B, 5),
            (0x1C, 5),
            (0x1D, 5),
            (0x1E, 5),
            (0x20, 6),
            (0x22, 7),
        ]);

        let rom_codes: [u8; 3] = [0x00, 0x08, 0x09];
        let ram_codes: [u8; 15] = [
            0x02, 0x03, 0x08, 0x09, 0x0C, 0x0D, 0x10, 0x12, 0x13, 0x1A, 0x1B, 0x1D, 0x1E, 0x22,
            0xFF,
        ];
        let battery_codes: [u8; 11] = [
            0x03, 0x06, 0x09, 0x0D, 0x0F, 0x10, 0x13, 0x1B, 0x1E, 0x22, 0xFF,
        ];
        let mmm01_codes: [u8; 3] = [0x0B, 0x0C, 0x0D];
        let timer_codes: [u8; 2] = [0x0F, 0x10];
        let cart_type = card[0x0147];
        let huc_indecies = HashMap::from([(0xff, 1), (0xfe, 3)]);
        let mbc_index: u8;
        let huc_index: u8;
        if mbc_indecies.contains_key(&cart_type) {
            mbc_index = mbc_indecies[&cart_type];
        } else {
            mbc_index = 0;
        }
        if huc_indecies.contains_key(&cart_type) {
            huc_index = huc_indecies[&cart_type];
        } else {
            huc_index = 0;
        }
        let info = cart_info::CartridgeInfo {
            cart_type: cart_type,
            rom_size: card.len(),
            title: title.to_string(),
            licensee: licensee.to_string(),
            rom: rom_codes.contains(&cart_type),
            ram: ram_codes.contains(&cart_type),
            battery: battery_codes.contains(&cart_type),
            mmm_01: mmm01_codes.contains(&cart_type),
            timer: timer_codes.contains(&cart_type),
            mbc_index: mbc_index,
            huc_index: huc_index,
            bandai_tama: cart_type == 0xfd,
            pocket_camera: cart_type == 0xfc,
            sensor: cart_type == 0x22,
            rumble: [0x1c, 0x1d, 0x1e, 0x22].contains(&cart_type),
        };
        Ok(info)
    }
}

pub fn read_to_str(data: &[u8]) -> String {
    match String::from_utf8(data.to_vec()) {
        Err(_err) => String::from("Unknown"),
        Ok(word) => word,
    }
}
