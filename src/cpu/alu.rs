use crate::cpu::registers::Registers;

const ADD_OPCODE: u8 = 0b100;
const SUB_OPCODE: u8 = 0b1001;
const OPCODE_MASK: u8 = 0b00111000;
const REGISTER_MASK: u8 = 0b111;

fn nand(a: u8, b: u8) -> u8 {
    !(a & b)
}

pub fn alu(opcode: u8, registers: Registers) -> Option<u8> {
    let masked_opcode = (opcode & OPCODE_MASK) >> 3;
    let src_register = (opcode & REGISTER_MASK);
    let mut src_value = registers.get_register_value(src_register)?;

    if (masked_opcode >> 2 == 0) { // ADD / ADC / SUB / SBC
        if (masked_opcode >> 1 & 0b1) {
            src_value = !src_value;
        }
        return Some(add(registers.a, src_value, masked_opcode & 0b1 == 1));
    } else { // AND / XOR / OR / CP
        Some(1)
    }
}

pub fn add(src: u8, dest: u8, carry: bool) -> u8 {
    (src + dest)
}
