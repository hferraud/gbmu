use gbmu::cartridge;
use gbmu::cpu::CPU;
use gbmu::error;
use gbmu::mmu::MMU;
use gbmu::ppu;
use std::env;

use std::error::Error;
use std::io;
use std::io::Read;
use gbmu::cartridge::Cartridge;

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

    let mut breakpoint: bool = false;
    loop {
        if breakpoint {
            println!("New instruction:");
            println!("PC: {:#06x}", cpu.registers.pc);
            println!("Opcode");
        }
        if breakpoint {
            println!("Instruction: {:#06x}", mmu.get_word(cpu.registers.pc as usize)?);
        }
        cpu.run(&mut mmu);
        unsafe { ppu::run(&mut mmu); }
        if breakpoint {
            println!("{:#x?}", cpu.registers);
            println!();
        }
        if cpu.registers.pc == 0x08e {
            breakpoint = true;
        }
        if breakpoint {
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .expect("Erreur de lecture");
            if input.trim() == "q" {
                break;
            }
        }
    }
    Ok(())
}
