trait MBC {
    pub fn read(&self, address: usize) -> u8;
    pub fn write(&self, address: usize, value: u8);
}
