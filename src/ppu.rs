use crate::mmu::MMU;

use std::io;
use std::ptr::write;

const LY_REGISTER: u16 = 0xFF44;

pub unsafe fn run(mmu: &mut MMU) -> Result<(), io::Error> {
    static mut cycle: u32 = 0;

    if cycle % 456 == 0 {
        let ly = mmu.get_word(LY_REGISTER as usize)? + 1;
        if ly == 154 {
            mmu.set_word(LY_REGISTER as usize, 0)?;
        } else {
            mmu.set_word(LY_REGISTER as usize, ly)?;
        }
        cycle = 0;
    }
    cycle += 1;
    Ok(())
}

pub fn print_bg(mmu: &mut MMU) -> Result<(), io::Error> {
    for row in 0..32 {
        for i in 0..(32 * 8 * 2 + 33) {
            print!("-");
        }
        println!();
        for line in 0..8 {
            print!("|");
            for col in 0..32 {
                let tile_index = mmu.get_word(0x9800 + 32 * row + col)?;
                print_tile_line(mmu, tile_index, line);
                print!("|");
            }
            println!();
        }
    }
    for i in 0..(32 * 8 * 2 + 33) {
        print!("-");
    }
    println!();
    println!();
    println!();
    Ok(())
}

fn print_tile_line(mmu: &mut MMU, tile_id: u8, line: u8) -> Result<(), io::Error> {
    let tile_line = mmu.get_dword(0x8000 as usize + (tile_id as usize * 16) + (line as usize * 2))?;


    for j in 0..8 {
        let left = tile_line >> 8;
        let right = tile_line;
        let char_sets = " ###";
        let mut bit = (right >> (7 - j) & 1) + ((left >> (7 - j) & 1) << 1);

        print!("{} ", char_sets.chars().nth(bit as usize).unwrap());
    }
    Ok(())
}
