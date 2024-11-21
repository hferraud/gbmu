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
use gbmu::ppu::print_bg;

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

    while cpu.registers.pc != 0x2b6 {
        println!("A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}", cpu.registers.a, cpu.registers.f, cpu.registers.b, cpu.registers.c, cpu.registers.d, cpu.registers.e, cpu.registers.h, cpu.registers.l, cpu.registers.sp, cpu.registers.pc, mmu.get_word(cpu.registers.pc as usize)?, mmu.get_word(cpu.registers.pc as usize + 1)?, mmu.get_word(cpu.registers.pc as usize + 2)?, mmu.get_word(cpu.registers.pc as usize + 3)?);
        cpu.run(&mut mmu);
        unsafe {
            ppu::run(&mut mmu);
        }
    }
    // print_bg(&mut mmu);
    Ok(())
}
