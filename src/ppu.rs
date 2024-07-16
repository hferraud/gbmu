use crate::mmu::MMU;

use std::io;

const LY_REGISTER: u16 = 0xFF44;

pub unsafe fn run(mmu: &mut MMU) -> Result<(), io::Error> {
    static mut cycle: u32 = 0;

    if cycle % 456 == 0 {
        for i in 0..8 {
            let tile_line = mmu.get_dword(0x8000 + 0xa * 16 + i * 2)?;
            // println!("Address {:#0x}", 0x8000 + 19 * 16 + i * 2);
            for j in 0..8 {
                let left = tile_line >> 8;
                let right = tile_line;
                let char_sets = " .#@";
                let mut bit = (right >> (7 - j) & 1) + ((left >> (7 - j) & 1) << 1);

                print!("{}", char_sets.chars().nth(bit as usize).unwrap());
            }
            println!();
            // println!("{:#010b}, {:#010b}", (tile_line >> 8) as u8, tile_line as u8);
        }
        println!();
        println!();
        println!();
        let ly = mmu.get_word(LY_REGISTER as usize)? + 1;
        if ly == 154 {
            let start_tile = mmu.get_word(0x8190)?;
             mmu.set_word(LY_REGISTER as usize, 0)?;
        } else {
            mmu.set_word(LY_REGISTER as usize, ly)?;
        }
        cycle = 0;
    }
    cycle += 1;
    Ok(())
}
