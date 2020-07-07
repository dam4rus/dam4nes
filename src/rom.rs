const PRG_PAGE_SIZE: usize = 16 * 1024;
const CRH_PAGE_SIZE: usize = 8 * 1024;

#[derive(Debug, Default)]
pub struct ROM {
    prg_rom_page_count: u8,
    chr_rom_page_count: u8,
    mapper_id: u8,
    prg_ram_page_count: u8,
    flags: u8,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
}

impl ROM {
    pub fn from_slice(value: &[u8]) -> Result<Self, &'static str> {
        match value {
            [0x4E, 0x45, 0x53, 0x1A, prg_rom_page_count, chr_rom_page_count, mapper_low, mapper_high, prg_ram_page_count, flags, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, rest @ ..] =>
            {
                let prg_rom_size = *prg_rom_page_count as usize * PRG_PAGE_SIZE;
                let chr_rom_size = *chr_rom_page_count as usize * CRH_PAGE_SIZE;
                Ok(Self {
                    prg_rom_page_count: *prg_rom_page_count,
                    chr_rom_page_count: *chr_rom_page_count,
                    mapper_id: mapper_low | (mapper_high << 4),
                    prg_ram_page_count: *prg_ram_page_count,
                    flags: *flags,
                    prg_rom: rest[0..prg_rom_size].to_vec(),
                    chr_rom: rest[prg_rom_size..prg_rom_size + chr_rom_size].to_vec(),
                })
            }
            _ => Err("Input is not a valid iNES file format"),
        }
    }
}
