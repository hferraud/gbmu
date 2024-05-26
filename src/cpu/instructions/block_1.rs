use crate::cpu::instructions::loader::load;
use crate::cpu::registers::Registers;
use crate::mmu::MMU;

use std::io;

pub fn execute(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    load(opcode, registers, mmu)
}
