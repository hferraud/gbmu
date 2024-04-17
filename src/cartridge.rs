pub mod header;

use std::mem::size_of;
use std::os::unix::fs::FileExt;

use crate::cartridge::header::CartridgeHeader;

#[derive(Debug)]
pub struct Cartridge {
    pub header: CartridgeHeader,
}

impl Cartridge {
    pub fn load_rom(rom_path: &str) -> Result<Cartridge, std::io::Error> {
        let file = std::fs::File::open(rom_path)?;
        let mut buffer = vec![0; size_of::<CartridgeHeader>()];

        file.read_exact_at(&mut buffer, header::HEADER_OFFSET)?;
        Ok (Cartridge {
            header: CartridgeHeader::from_bytes(&buffer)?,
        })
    }
}
