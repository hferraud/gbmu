pub mod header;
use crate::cartridge::header::CartridgeHeader;

use std::mem::size_of;
use std::os::unix::fs::FileExt;

#[derive(Debug)]
pub struct Cartridge {
    header: CartridgeHeader,
}

impl Cartridge {
    pub fn new() -> Cartridge {
        Cartridge {
            header: CartridgeHeader::new(),
        }
    }

    pub fn load_rom(rom_path: &str) -> Result<Cartridge, std::io::Error> {
        let file = std::fs::File::open(rom_path)?;
        let mut buffer = vec![0; size_of::<CartridgeHeader>()];
        file.read_exact_at(&mut buffer, header::HEADER_OFFSET)?;
        let cartridge = Cartridge {
            header: CartridgeHeader::from_bytes(&buffer).unwrap_or(CartridgeHeader::new()),
        };
        Ok(cartridge)
    }
}
