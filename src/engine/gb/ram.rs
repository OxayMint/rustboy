pub struct Ram {
    wram: [u8; 0x2000],
    hram: [u8; 0x80],
}

pub fn read(address: u16) -> u8 {
    address -= 0xC000;

    if (address >= 0x2000) {
        printf("INVALID WRAM ADDR %08X\n", address + 0xC000);
        exit(-1);
    }

    return ctx.wram[address];
}

pub fn write(address: u16, value: u8) {
    address -= 0xC000;

    ctx.wram[address] = value;
}

pub fn read(address: u16) -> u8 {
    address -= 0xFF80;

    return ctx.hram[address];
}

pub fn write(address: u16, value: u8) {
    address -= 0xFF80;

    ctx.hram[address] = value;
}
