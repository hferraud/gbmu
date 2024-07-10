pub mod header;
pub mod mbc0;

use std::io;
use std::io::Read;
use std::ptr;

use crate::cartridge::header::CartridgeHeader;
use crate::cartridge::header::{HEADER_END, HEADER_OFFSET};
use crate::cartridge::mbc0::MBC0;
use crate::error;

#[derive(Debug)]
pub struct Cartridge {
    pub header: CartridgeHeader,
    pub mbc: MBC0,
}

impl Cartridge {
    pub fn load_mbc(cartridge_type: u8, data: Vec<u8>) -> MBC0 {
        match cartridge_type {
            0x00 => MBC0::new(data),
            _ => MBC0::new(data),
        }
    }

    pub fn load_rom(rom_path: &str) -> Result<Cartridge, std::io::Error> {
        let mut file = std::fs::File::open(rom_path)?;
        let mut buffer = vec![];
        file.read_to_end(&mut buffer)?;
        Self::load_power_up("power_up/dmg.bin", &mut buffer)?;
        let cartridge_header = CartridgeHeader::from_bytes(&buffer[HEADER_OFFSET..=HEADER_END])?;
        let cartridge_type = cartridge_header.cartridge_type[0];
        let cartridge = Cartridge {
            header: cartridge_header,
            mbc: Self::load_mbc(cartridge_type, buffer),
        };
        Ok(cartridge)
    }

    pub fn load_power_up(path: &str, rom: &mut Vec<u8>) -> Result<(), io::Error> {
        let mut file = std::fs::File::open(path)?;
        let mut buffer = vec![];
        file.read_to_end(&mut buffer)?;
        if rom.len() < buffer.len() {
            return Err(error::invalid_rom_size());
        }
        unsafe {
            ptr::copy_nonoverlapping(buffer.as_ptr(), rom.as_mut_ptr(), buffer.len());
        }
        Ok(())
    }
}
