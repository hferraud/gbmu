use crate::cartridge::mbc0::MBC0;
use crate::wram::WRAM;

const MEMORY_SIZE: u16 = 0xFFFF;
const ROM_START: u16 = 0x0000;
const ROM_END: u16 = 0x7FFF;
const VRAM_START: u16 = 0x8000;
const VRAM_END: u16 = 0x9FFF;
const EXRAM_START: u16 = 0xA000;
const EXRAM_END: u16 = 0xBFFF;
const WRAM_START: u16 = 0xC000;
const WRAM_END: u16 = 0xDFFF;
const OAM_START: u16 = 0xFE00;
const OAM_END: u16 = 0xFE9F;
const IO_START: u16 = 0xFF00;
const IO_END: u16 = 0xFF7F;
const HRAM_START: u16 = 0xFF80;
const HRAM_END: u16 = 0xFFFE;
const IE_REGISTER: u16 = 0xFFFF;

struct MMU<'a> {
    mbc: &'a mut MBC0,
    wram: &'a mut WRAM,
}

impl<'a> MMU<'a> {
    pub fn new(mbc: &'a mut MBC0, wram: &'a mut WRAM) -> Self {
        MMU { mbc, wram }
    }

    pub fn get(&mut self, address: u16) -> Option<u8> {
        return Some(*self.get_physical_address(address)?);
    }

    pub fn set(&mut self, address: u16, value: u8) {
        if let Some(physical_address) = self.get_physical_address(address) {
            *physical_address = value;
        }
    }

    fn get_physical_address(&mut self, address: u16) -> Option<&mut u8> {
        if address >= ROM_START && address <= ROM_END {
            return Some(self.mbc.get_address(address));
        } else if address >= WRAM_START && address <= WRAM_END {
            return Some(self.wram.get_address(address - WRAM_START));
        }
        return None;
    }
}
