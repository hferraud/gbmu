use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::lcd::LCD;
use crate::mmu::MMU;
use crate::ppu;
use anyhow::Result;

use std::io;

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
            cpu: CPU::default(),
            lcd: LCD::default(),
        })
    }

    pub fn run_instruction(&mut self) -> Result<(), io::Error> {
        self.cpu.run(&mut self.mmu)?;
        ppu::run(&mut self.mmu)
    }
}
