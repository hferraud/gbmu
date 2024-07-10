use crate::mmu::MMU;

use std::io;

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
