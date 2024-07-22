use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::mmu::MMU;
use crate::lcd::LCD;
use anyhow::Result;

// TODO create a gameboy folder with all the gameboy modules
pub struct Gameboy {
    pub cpu: CPU,
    pub mmu: MMU,
    pub lcd: LCD,
    
}

impl Gameboy {
    pub fn new(rom_path: &str) -> Result<Self> {
        let cartridge = Cartridge::load_rom(rom_path)?;
        Ok(Self {
            mmu: MMU::new(cartridge.mbc, false),
            cpu: CPU::new(),
            lcd: LCD::new()
        })
    }
}
