use crate::mmu::MMU;

use std::io;
use std::sync::Mutex;

const LY_REGISTER: u16 = 0xFF44;

pub fn run(mmu: &mut MMU) -> Result<(), io::Error> {
    static CYCLE: Mutex<u32> = Mutex::new(0);

    let mut cycle = CYCLE.lock().expect("ppm::run::CYCLE Mutex poisoned");

    if *cycle % 456 == 0 {
        let ly = mmu.get_word(LY_REGISTER as usize)? + 1;
        if ly == 154 {
            mmu.set_word(LY_REGISTER as usize, 0)?;
        } else {
            mmu.set_word(LY_REGISTER as usize, ly)?;
        }
        *cycle = 0;
    }
    *cycle += 1;
    Ok(())
}
