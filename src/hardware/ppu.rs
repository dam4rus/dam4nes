use super::memory::Memory;

use log::debug;

const INTERNAL_MEMORY_SIZE: usize = 2 * 1024;
const OAM_SIZE: usize = 256;
const PATTERN_TABLE_SECTION_SIZE: usize = 4 * 1024;
const NAME_TABLE_SIZE: usize = 1024;
const TILE_SIZE: usize = 16;

#[derive(Copy, Clone)]
pub struct PPU {
    pub registers: Registers,
    pub clock: Clock,
    pub internal_memory: [u8; INTERNAL_MEMORY_SIZE],
    pub oam: OAM,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            registers: Default::default(),
            clock: Default::default(),
            internal_memory: [0u8; INTERNAL_MEMORY_SIZE],
            oam: OAM::new(),
        }
    }

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

#[derive(Copy, Clone)]
pub struct OAM(pub [u8; OAM_SIZE]);

impl OAM {
    pub fn new() -> Self {
        Self([0u8; OAM_SIZE])
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PatternTables<'a> {
    pub left: &'a [u8],
    pub right: &'a [u8],
}

impl<'a> PatternTables<'a> {
    pub fn new(left: &'a [u8], right: &'a [u8]) -> Result<Self, String> {
        match (left.len(), right.len()) {
            (PATTERN_TABLE_SECTION_SIZE, PATTERN_TABLE_SECTION_SIZE) => Ok(Self { left, right }),
            (left_len, right_len) => Err(format!(
                "Invalid pattern table slice size. Meeds to be {} bytes, got: (left: {}, right: {})",
                PATTERN_TABLE_SECTION_SIZE, left_len, right_len,
            )),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct NameTables<'a> {
    pub top_left: &'a [u8],
    pub top_right: &'a [u8],
    pub bottom_left: &'a [u8],
    pub bottom_right: &'a [u8],
}

impl<'a> NameTables<'a> {
    pub fn new(
        top_left: &'a [u8],
        top_right: &'a [u8],
        bottom_left: &'a [u8],
        bottom_right: &'a [u8],
    ) -> Result<Self, String> {
        match (top_left.len(), top_right.len(), bottom_left.len(), bottom_right.len()) {
            (NAME_TABLE_SIZE, NAME_TABLE_SIZE, NAME_TABLE_SIZE, NAME_TABLE_SIZE) => {
                Ok(Self { top_left, top_right, bottom_left, bottom_right })
            }
            (top_left_len, top_right_len, bottom_left_len, bottom_right_len) => {
                Err(format!(
                    "Invalid nametable slice size. Needs to be {} bytes, got: (top_left: {}, top_right: {}, bottom_left: {}, bottom_right: {})",
                    NAME_TABLE_SIZE,
                    top_left_len,
                    top_right_len,
                    bottom_left_len,
                    bottom_right_len,
                ))
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct Tile(pub [[u8; 8]; 8]);

impl Tile {
    pub fn from_slice(slice: &[u8]) -> Result<Self, String> {
        match slice.len() {
            16 => {
                let mut tile = [[0u8; 8]; 8];

                let (first_planes, second_planes) = slice.split_at(8);
                let zipped_planes = first_planes.iter().zip(second_planes);

                for (row, planes) in &mut tile.iter_mut().zip(zipped_planes) {
                    for (bit_index, color) in &mut row.iter_mut().rev().enumerate() {
                        let color_0 = (planes.0 >> bit_index) & 0x01;
                        let color_1 = ((planes.1 >> bit_index) & 0x01) << 1;
                        *color = color_0 | color_1;
                    }
                }

                Ok(Self(tile))
            }
            n => Err(format!(
                "Invalid tile slice size. A tile needs to be {} bytes but got {}",
                TILE_SIZE, n
            )),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Self {
        Self {
            r: value.0,
            g: value.1,
            b: value.2,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct Palette;

impl Palette {
    const COLORS: [Color; 0x40] = [
        Color{ r: 84, g: 84, b: 84 },
        Color{ r: 0, g: 30, b: 3 },
        Color{ r: 8, g: 16, b: 3 },
        Color{ r: 48, g: 0, b: 3 },
        Color{ r: 68, g: 0, b: 3 },
        Color{ r: 92, g: 0, b: 3 },
        Color{ r: 84, g: 4, b: 3 },
        Color{ r: 60, g: 24, b: 3 },
        Color{ r: 32, g: 42, b: 3 },
        Color{ r: 8, g: 58, b: 3 },
        Color{ r: 0, g: 64, b: 3 },
        Color{ r: 0, g: 60, b: 3 },
        Color{ r: 0, g: 50, b: 3 },
        Color{ r: 0, g: 0, b: 3 },
        Color{ r: 0, g: 0, b: 3 },
        Color{ r: 0, g: 0, b: 3 },
        Color{ r: 152, g: 150, b: 3 },
        Color{ r: 8, g: 76, b: 3 },
        Color{ r: 48, g: 50, b: 3 },
        Color{ r: 92, g: 30, b: 3 },
        Color{ r: 136, g: 20, b: 3 },
        Color{ r: 160, g: 20, b: 3 },
        Color{ r: 152, g: 34, b: 3 },
        Color{ r: 120, g: 60, b: 3 },
        Color{ r: 84, g: 90, b: 3 },
        Color{ r: 40, g: 114, b: 3 },
        Color{ r: 8, g: 124, b: 3 },
        Color{ r: 0, g: 118, b: 3 },
        Color{ r: 0, g: 102, b: 3 },
        Color{ r: 0, g: 0, b: 3 },
        Color{ r: 0, g: 0, b: 3 },
        Color{ r: 0, g: 0, b: 3 },
        Color{ r: 236, g: 238, b: 3 },
        Color{ r: 76, g: 154, b: 3 },
        Color{ r: 120, g: 124, b: 3 },
        Color{ r: 176, g: 98, b: 3 },
        Color{ r: 228, g: 84, b: 3 },
        Color{ r: 236, g: 88, b: 3 },
        Color{ r: 236, g: 106, b: 3 },
        Color{ r: 212, g: 136, b: 3 },
        Color{ r: 160, g: 170, b: 3 },
        Color{ r: 116, g: 196, b: 3 },
        Color{ r: 76, g: 208, b: 3 },
        Color{ r: 56, g: 204, b: 3 },
        Color{ r: 56, g: 180, b: 3 },
        Color{ r: 60, g: 60, b: 3 },
        Color{ r: 0, g: 0, b: 3 },
        Color{ r: 0, g: 0, b: 3 },
        Color{ r: 236, g: 238, b: 3 },
        Color{ r: 168, g: 204, b: 3 },
        Color{ r: 188, g: 188, b: 3 },
        Color{ r: 212, g: 178, b: 3 },
        Color{ r: 236, g: 174, b: 3 },
        Color{ r: 236, g: 174, b: 3 },
        Color{ r: 236, g: 180, b: 3 },
        Color{ r: 228, g: 196, b: 3 },
        Color{ r: 204, g: 210, b: 3 },
        Color{ r: 180, g: 222, b: 3 },
        Color{ r: 168, g: 226, b: 3 },
        Color{ r: 152, g: 226, b: 3 },
        Color{ r: 160, g: 214, b: 3 },
        Color{ r: 160, g: 162, b: 3 },
        Color{ r: 0, g: 0, b: 3 },
        Color{ r: 0, g: 0, b: 3 },
    ];
}

pub struct MMU<'a, 'b, 'c> {
    pub pattern_tables: &'a PatternTables<'b>,
    pub name_tables: &'a NameTables<'c>,
}

impl<'a, 'b, 'c> Memory for MMU<'a, 'b, 'c> {
    fn read(&self, address: u16) -> Option<u8> {
        match address {
            0x0000..=0x0FFF => Some(self.pattern_tables.left[address as usize]),
            start @ 0x1000..=0x1FFF => Some(self.pattern_tables.right[(address - start) as usize]),
            start @ 0x2000..=0x23FF => Some(self.name_tables.top_left[(address - start) as usize]),
            start @ 0x2400..=0x27FF => Some(self.name_tables.top_right[(address - start) as usize]),
            start @ 0x2800..=0x2BFF => Some(self.name_tables.bottom_left[(address - start) as usize]),
            start @ 0x2C00..=0x2FFF => Some(self.name_tables.bottom_right[(address - start) as usize]),
            0x3000..=0x3EFF => self.read(address - 0x1000), // mirrors 0x2000..=0x2EFF
            0x3F00..=0x3FFF => unimplemented!(),            // palette ram indexes
            _ => None,
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::Tile;

    #[test]
    fn test_tile() {
        let tile_data: [u8; 16] = [
            0x41, 0xC2, 0x44, 0x48, 0x10, 0x20, 0x40, 0x80, 0x01, 0x02, 0x04, 0x08, 0x16, 0x21, 0x42, 0x87,
        ];
        let tile = Tile::from_slice(&tile_data).unwrap();
        let Tile(tile_matrix) = tile;
        assert_eq!(
            tile_matrix,
            [
                [0, 1, 0, 0, 0, 0, 0, 3],
                [1, 1, 0, 0, 0, 0, 3, 0],
                [0, 1, 0, 0, 0, 3, 0, 0],
                [0, 1, 0, 0, 3, 0, 0, 0],
                [0, 0, 0, 3, 0, 2, 2, 0],
                [0, 0, 3, 0, 0, 0, 0, 2],
                [0, 3, 0, 0, 0, 0, 2, 0],
                [3, 0, 0, 0, 0, 2, 2, 2],
            ]
        );
    }
}
