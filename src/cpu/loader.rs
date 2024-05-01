use crate::cpu::{BLOCK_MASK, BLOCK_SHIFT, BLOCK_1};
use crate::cpu::registers::Registers;


const LOAD_REGISTER_DST_MASK: u8 = 0b00111000;
const LOAD_REGISTER_SRC_MASK: u8 = 0b00000111;


pub fn load(opcode: u8, registers: &mut Registers) -> Option<u8> {
    let block = (opcode & BLOCK_MASK) >> BLOCK_SHIFT;
    match block {
        BLOCK_1 => load_register_to_register(opcode, registers),
        _ => None
    }
}

fn load_register_to_register(opcode: u8, registers: &mut Registers) -> Option<u8> {
    let dest_register = (opcode & LOAD_REGISTER_DST_MASK) >> 3;
    let src_register = opcode & LOAD_REGISTER_SRC_MASK;
    let src_value = registers.get_register_value(src_register)?;
    registers.set_register(dest_register, src_value);
    return Some(src_value);
}
