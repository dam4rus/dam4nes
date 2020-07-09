const PRG_PAGE_SIZE: usize = 16 * 1024;
const CRH_PAGE_SIZE: usize = 8 * 1024;

#[derive(Debug, Default)]
pub struct ROM<'a> {
    prg_rom_page_count: u8,
    chr_rom_page_count: u8,
    flags_6: u8,
    flags_7: u8,
    prg_ram_page_count: u8,
    flags_9: u8,
    flags_10: u8,
    prg_rom: &'a [u8],
    chr_rom: &'a [u8],
}

impl<'a> ROM<'a> {
    pub fn from_slice(value: &'a [u8]) -> Result<Self, &'static str> {
        match value {
            [0x4E, 0x45, 0x53, 0x1A, prg_rom_page_count, chr_rom_page_count, flags_6, flags_7, prg_ram_page_count, flags_9, flags_10, 0x00, 0x00, 0x00, 0x00, 0x00, rest @ ..] =>
            {
                let prg_rom_size = *prg_rom_page_count as usize * PRG_PAGE_SIZE;
                let chr_rom_size = *chr_rom_page_count as usize * CRH_PAGE_SIZE;
                Ok(Self {
                    prg_rom_page_count: *prg_rom_page_count,
                    chr_rom_page_count: *chr_rom_page_count,
                    flags_6: *flags_6,
                    flags_7: *flags_7,
                    prg_ram_page_count: *prg_ram_page_count,
                    flags_9: *flags_9,
                    flags_10: *flags_10,
                    prg_rom: &rest[0..prg_rom_size],
                    chr_rom: &rest[prg_rom_size..prg_rom_size + chr_rom_size],
                })
            }
            _ => Err("Input is not a valid iNES file format"),
        }
    }

    pub fn mirroring(&self) -> Mirroring {
        match (self.flags_6 & 0x01) != 0 {
            false => Mirroring::Horizontal,
            true => Mirroring::Vertical,
        }
    }

    pub fn prg_rom(&self) -> &[u8] {
        self.prg_rom
    }

    pub fn chr_rom(&self) -> &[u8] {
        self.chr_rom
    }
}

pub enum Mirroring {
    Horizontal,
    Vertical,
}
