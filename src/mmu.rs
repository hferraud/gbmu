use std::io;

use crate::cartridge::mbc0::MBC0;
use crate::error;
use crate::hram::HRAM;
use crate::wram::WRAM;

#[allow(unused)]
const MEMORY_SIZE: usize = 0xFFFF;
const ROM_START: usize = 0x0000;
const ROM_END: usize = 0x7FFF;
#[allow(unused)]
const ROM_DWORD_END: usize = ROM_END - 1;
#[allow(unused)]
const VRAM_START: usize = 0x8000;
#[allow(unused)]
const VRAM_END: usize = 0x9FFF;
#[allow(unused)]
const EXRAM_START: usize = 0xA000;
#[allow(unused)]
const EXRAM_END: usize = 0xBFFF;
const WRAM_START: usize = 0xC000;
#[allow(unused)]
const WRAM_END: usize = 0xDFFF;
#[allow(unused)]
const WRAM_DWORD_END: usize = WRAM_END - 1;
#[allow(unused)]
const OAM_START: usize = 0xFE00;
#[allow(unused)]
const OAM_END: usize = 0xFE9F;
#[allow(unused)]
const IO_START: usize = 0xFF00;
#[allow(unused)]
const IO_END: usize = 0xFF7F;
const HRAM_START: usize = 0xFF80;
const HRAM_END: usize = 0xFFFE;
const IE_REGISTER: usize = 0xFFFF;

pub struct MMU {
    mbc:  MBC0,
    wram: WRAM,
    hram: HRAM,
    ie: u8,
}

impl MMU {
    pub fn new(mbc: MBC0, cbg_mode: bool) -> Self {
        let wram = WRAM::new(cbg_mode);
        let hram = HRAM::new();
        MMU {
            mbc,
            wram,
            hram,
            ie: u8::default(),
        }
    }

    pub fn get_word(&mut self, address: usize) -> Result<u8, io::Error> {
        Ok(*(self.fetch_word_address(address)?))
    }

    pub fn get_dword(&mut self, address: usize) -> Result<u16, io::Error> {
        let mut dword = self.get_word(address)? as u16;
        dword |= (self.get_word(address + 1)? as u16) << 8;
        Ok(dword)
    }

    pub fn set_word(&mut self, address: usize, value: u8) -> Result<(), io::Error> {
        *self.fetch_word_address(address)? = value;
        Ok(())
    }

    pub fn set_dword(&mut self, address: usize, value: u16) -> Result<(), io::Error> {
        self.set_word(address, (value & 0x00FF) as u8)?;
        self.set_word(address + 1, (value >> 8) as u8)?;
        Ok(())
    }

    fn fetch_word_address(&mut self, address: usize) -> Result<&mut u8, io::Error> {
        match address {
            ROM_START..=ROM_END => Ok(&mut self.mbc[address]),
            WRAM_START..=WRAM_END => Ok(&mut self.wram[address - WRAM_START]),
            HRAM_START..=HRAM_END => Ok(&mut self.hram[address - HRAM_START]),
            IE_REGISTER => Ok(&mut self.ie),
            _ => Err(error::invalid_address(address)),
        }
    }
}
