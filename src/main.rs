use gbmu::cartridge;
use std::env;
use gbmu::mmu::MMU;
use gbmu::wram::WRAM;
use gbmu::cpu::CPU;
use gbmu::error;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ./{} <ROM>", args[0]);
        return Err(Box::new(error::invalid_argument()));
    }
    let rom_path = &args[1];
    let mut cartridge = cartridge::Cartridge::load_rom(rom_path).unwrap();
    // println!("{:#x?}", cartridge);
    let mut wram = WRAM::new(false);
    let mut mmu = MMU::new(&mut cartridge.mbc, &mut wram);
    let mut cpu = CPU::new();
    cpu.run(&mut mmu)?;
    Ok(())
}
