#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct PPU {
    pub registers: Registers,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct Registers {
    pub ppuctrl: u8,
    pub ppumask: u8,
    pub ppustatus: u8,
    pub oamaddr: u8,
    pub oamdata: u8,
    pub ppuscroll: u8,
    pub ppuaddr: u8,
    pub ppudata: u8,
    pub oamdma: u8,
}

impl Registers {
    // pub fn base_nametable_address(&self) -> u16 {
    //     let value = self.ppuctrl & 0b0000_0011;
    //     0x2000 + (value as u16 * 0x0400)
    // }
}
