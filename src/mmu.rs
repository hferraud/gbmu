use std::io;

use crate::cartridge::mbc0::MBC0;
use crate::error;
use crate::wram::WRAM;

const MEMORY_SIZE: usize = 0xFFFF;
const ROM_START: usize = 0x0000;
const ROM_END: usize = 0x7FFF;
const ROM_DWORD_END: usize = ROM_END - 1;
const VRAM_START: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const EXRAM_START: usize = 0xA000;
const EXRAM_END: usize = 0xBFFF;
const WRAM_START: usize = 0xC000;
const WRAM_END: usize = 0xDFFF;
const WRAM_DWORD_END: usize = WRAM_END - 1;
const OAM_START: usize = 0xFE00;
const OAM_END: usize = 0xFE9F;
const IO_START: usize = 0xFF00;
const IO_END: usize = 0xFF7F;
const HRAM_START: usize = 0xFF80;
const HRAM_END: usize = 0xFFFE;
const IE_REGISTER: usize = 0xFFFF;

pub struct MMU<'a> {
    mbc: &'a mut MBC0,
    wram: &'a mut WRAM,
}

impl<'a> MMU<'a> {
    pub fn new(mbc: &'a mut MBC0, wram: &'a mut WRAM) -> Self {
        MMU { mbc, wram }
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
        self.set_word(address, (value >> 8) as u8)?;
        self.set_word(address + 1, value as u8)?;
        Ok(())
    }

    fn fetch_word_address(&mut self, address: usize) -> Result<&mut u8, io::Error> {
        match address {
            ROM_START..=ROM_END => Ok(self.mbc.get_address(address)),
            WRAM_START..=WRAM_END => Ok(self.wram.get_address(address - WRAM_START)),
            _ => Err(error::invalid_address()),
        }
    }
}
