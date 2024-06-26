use std::io;

use crate::cpu::registers::Registers;
use crate::mmu::MMU;

const LOAD_REGISTER_DST_MASK: u8 = 0b00111000;
const LOAD_REGISTER_SRC_MASK: u8 = 0b00000111;

pub fn load(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    let dest_register = (opcode & LOAD_REGISTER_DST_MASK) >> 3;
    let src_register = opcode & LOAD_REGISTER_SRC_MASK;
    let src_value = registers.get_word(src_register, mmu)?;
    registers.set_word(dest_register, src_value, mmu)
}
