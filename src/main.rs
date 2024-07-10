use gbmu::cartridge;
use gbmu::cpu::CPU;
use gbmu::error;
use gbmu::mmu::MMU;
use gbmu::ppu;
use std::env;

use gbmu::cartridge::Cartridge;
use std::error::Error;
use std::io;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ./{} <ROM>", args[0]);
        return Err(Box::new(error::invalid_argument()));
    }
    let rom_path = &args[1];
    let mut cartridge = cartridge::Cartridge::load_rom(rom_path).unwrap();
    let mut mmu = MMU::new(&mut cartridge.mbc, false);
    let mut cpu = CPU::new();

    loop {
        cpu.run(&mut mmu);
        unsafe {
            ppu::run(&mut mmu);
        }
    }
}
