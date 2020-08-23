pub(crate) const PRG_PAGE_SIZE: usize = 16 * 1024;
pub(crate) const CRH_PAGE_SIZE: usize = 8 * 1024;

#[derive(Debug, Default)]
pub struct ROM {
    content: Vec<u8>,
    prg_rom_page_count: u8,
    chr_rom_page_count: u8,
    flags_6: u8,
    flags_7: u8,
    prg_ram_page_count: u8,
    flags_9: u8,
    flags_10: u8,
}

impl ROM {
    pub fn with_content(content: Vec<u8>) -> Result<Self, &'static str> {
        match content.as_slice() {
            [0x4E, 0x45, 0x53, 0x1A, prg_rom_page_count, chr_rom_page_count, flags_6, flags_7, prg_ram_page_count, flags_9, flags_10, 0x00, 0x00, 0x00, 0x00, 0x00, ..] =>
            {
                let prg_rom_page_count = *prg_rom_page_count;
                let chr_rom_page_count = *chr_rom_page_count;
                let flags_6 = *flags_6;
                let flags_7 = *flags_7;
                let prg_ram_page_count = *prg_ram_page_count;
                let flags_9 = *flags_9;
                let flags_10 = *flags_10;
                Ok(Self {
                    content,
                    prg_rom_page_count,
                    chr_rom_page_count,
                    flags_6,
                    flags_7,
                    prg_ram_page_count,
                    flags_9,
                    flags_10,
                })
            }
            _ => Err("Input is not a valid iNES file format"),
        }
    }

    // pub fn mirroring(&self) -> Mirroring {
    //     match (self.flags_6 & 0x01) != 0 {
    //         false => Mirroring::Horizontal,
    //         true => Mirroring::Vertical,
    //     }
    // }

    pub fn prg_rom(&self) -> &[u8] {
        &self.content[0x10..0x10 + self.prg_rom_size()]
    }

    pub fn chr_rom(&self) -> &[u8] {
        &self.content[0x10 + self.prg_rom_size()..0x10 + self.prg_rom_size() + self.chr_rom_size()]
    }

    fn prg_rom_size(&self) -> usize {
        self.prg_rom_page_count as usize * PRG_PAGE_SIZE
    }
    
    fn chr_rom_size(&self) -> usize {
        self.chr_rom_page_count as usize * CRH_PAGE_SIZE
    }
}

// pub enum Mirroring {
//     Horizontal,
//     Vertical,
// }
