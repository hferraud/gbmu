use std::io;

use crate::cartridge::mbc0::MBC0;
use crate::error;
use gpio::GPIO;
use hram::HRAM;
use oam::OAM;
use vram::VRAM;
use wram::WRAM;

pub mod gpio;
pub mod hram;
pub mod oam;
pub mod vram;
pub mod wram;

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
const GPIO_START: usize = 0xFF00;
const GPIO_END: usize = 0xFF7F;
const HRAM_START: usize = 0xFF80;
const HRAM_END: usize = 0xFFFE;
const IE_REGISTER: usize = 0xFFFF;

pub struct MMU<'a> {
    mbc: &'a mut MBC0,
    wram: WRAM,
    hram: HRAM,
    gpio: GPIO,
    vram: VRAM,
    oam: OAM,
    ie: u8,
}

impl<'a> MMU<'a> {
    pub fn new(mbc: &'a mut MBC0, cbg_mode: bool) -> Self {
        let wram = WRAM::new(cbg_mode);
        let hram = HRAM::new();
        let gpio = GPIO::new();
        let vram = VRAM::new(cbg_mode);
        let oam = OAM::new();
        MMU {
            mbc,
            wram,
            hram,
            gpio,
            vram,
            oam,
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
            GPIO_START..=GPIO_END => Ok(&mut self.gpio[address - GPIO_START]),
            VRAM_START..=VRAM_END => Ok(&mut self.vram[address - VRAM_START]),
            OAM_START..=OAM_END => Ok(&mut self.oam[address - OAM_START]),
            IE_REGISTER => Ok(&mut self.ie),
            _ => Err(error::invalid_address(address)),
        }
    }
}
