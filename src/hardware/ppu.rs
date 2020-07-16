use log::debug;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct PPU {
    pub registers: Registers,
    pub clock: Clock,
}

impl PPU {
    pub fn update(&mut self) {
        match (self.clock.scanline, self.clock.cycle) {
            (241, 1) => {
                debug!("Setting vblank");
                let status_flags = self.registers.status_flags();
                self.registers.set_status_flags(StatusFlags {
                    vblank: true,
                    ..status_flags
                });
            }
            (261, 1) => {
                debug!("Clearing vblank");
                let status_flags = self.registers.status_flags();
                self.registers.set_status_flags(StatusFlags {
                    vblank: false,
                    ..status_flags
                })
            }
            _ => (),
        }
    }
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

    pub fn status_flags(&self) -> StatusFlags {
        StatusFlags::from(self.ppustatus)
    }

    pub fn set_status_flags(&mut self, status_flags: StatusFlags) {
        self.ppustatus = status_flags.into();
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct StatusFlags {
    pub least_significant_bits: u8,
    pub sprite_overflow: bool,
    pub sprite_0_hit: bool,
    pub vblank: bool,
}

impl From<u8> for StatusFlags {
    fn from(value: u8) -> Self {
        Self {
            least_significant_bits: (value & 0b0001_1111),
            sprite_overflow: (value & 0b0010_0000) != 0,
            sprite_0_hit: (value & 0b0100_0000) != 0,
            vblank: (value & 0b1000_000) != 0,
        }
    }
}

impl Into<u8> for StatusFlags {
    fn into(self) -> u8 {
        let mut value = self.least_significant_bits;
        if self.sprite_overflow {
            value |= 0b0010_0000;
        }
        if self.sprite_0_hit {
            value |= 0b0100_0000;
        }
        if self.vblank {
            value |= 0b1000_0000;
        }
        value
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct Clock {
    pub cycle: u32,
    pub scanline: u32,
}

impl Clock {
    pub fn step(&mut self) {
        self.cycle = match self.cycle + 1 {
            result if result <= 340 => result,
            _ => {
                self.scanline = match self.scanline + 1 {
                    result if result <= 261 => result,
                    _ => 0,
                };
                0
            }
        };
    }
}