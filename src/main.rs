use gbmu::cartridge;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ./{} <ROM>", args[0]);
        return;
    }
    let rom_path = &args[1];
    let cartridge = cartridge::Cartridge::load_rom(rom_path).unwrap();
    println!("{:#x?}", cartridge.header);
}
