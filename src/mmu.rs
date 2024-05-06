use std::io;

use crate::cartridge::mbc0::MBC0;
use crate::error;
use crate::wram::WRAM;

const MEMORY_SIZE: usize = 0xFFFF;
const ROM_START: usize = 0x0000;
const ROM_END: usize = 0x7FFF;
const VRAM_START: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const EXRAM_START: usize = 0xA000;
const EXRAM_END: usize = 0xBFFF;
const WRAM_START: usize = 0xC000;
const WRAM_END: usize = 0xDFFF;
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

    pub unsafe fn get_dword(&mut self, address: usize) -> Result<u16, io::Error> {
        Ok(*(self.fetch_dword_address(address)?))
    }

    pub fn set_word(&mut self, address: usize, value: u8) -> Result<(), io::Error> {
        *self.fetch_word_address(address)? = value;
        Ok(())
    }

    pub unsafe fn set_dword(&mut self, address: usize, value: u16) -> Result<(), io::Error> {
        *self.fetch_dword_address(address)? = value;
        Ok(())
    }

    pub fn fetch_word_address(&mut self, address: usize) -> Result<&mut u8, io::Error> {
        if address >= ROM_START && address <= ROM_END {
            return Ok(self.mbc.get_address(address));
        } else if address >= WRAM_START && address <= WRAM_END {
            return Ok(self.wram.get_address(address - WRAM_START));
        }
        Err(error::invalid_address())
    }

    // TODO: make sure that unsafe is really needed here
    pub unsafe fn fetch_dword_address(&mut self, address: usize) -> Result<&mut u16, io::Error> {
        let word_address = self.fetch_word_address(address)?;
        let dword_address: &mut u16 = &mut *(word_address as *mut u8 as *mut u16);
        Ok(dword_address)
    }
}
