use std::str::Matches;
use crate::cpu::registers::Registers;
use crate::cpu::registers::Flags;

const OPCODE_MASK: u8 = 0b00111000;
const REGISTER_MASK: u8 = 0b00000111;
const ADD_OPCODE: u8 = 0b000;
const ADC_OPCODE: u8 = 0b001;
const SUB_OPCODE: u8 = 0b010;
const SBC_OPCODE: u8 = 0b011;
const AND_OPCODE: u8 = 0b100;
const XOR_OPCODE: u8 = 0b101;
const OR_OPCODE: u8 = 0b110;
const CP_OPCODE: u8 = 0b111;

fn nand(a: u8, b: u8) -> u8 {
    !(a & b)
}

pub fn alu(opcode: u8, registers: &mut Registers) -> Option<u8> {
    let masked_opcode = (opcode & OPCODE_MASK) >> 3;
    let src_register = (opcode & REGISTER_MASK);
    let src_value = registers.get_register_value(src_register)?;

    registers.reset_flags();
    let result = match masked_opcode {
        ADD_OPCODE => Some(add(src_value, false, registers)),
        ADC_OPCODE => Some(add(src_value, true, registers)),
        SUB_OPCODE => Some(sub(src_value, false, registers)),
        SBC_OPCODE => Some(sub(src_value, true, registers)),
        AND_OPCODE => {
            registers.set_flags(Flags::H, true);
            Some(registers.a & src_value)
        },
        XOR_OPCODE => Some(registers.a ^ src_value),
        OR_OPCODE => Some(registers.a | src_value),
        CP_OPCODE => Some(cp(src_value, true, registers)),
        _ => None
    };
    if let Some(result) = result {
        if result == 0 {
            registers.set_flags(Flags::Z, true);
        }
        if (masked_opcode != CP_OPCODE) {
            registers.a = result;
        }
    }
    return result;
}

pub fn add(operand: u8, carry: bool, registers: &mut Registers) -> u8 {
    if (carry) {
        registers.set_flags(Flags::C, operand > u8::MAX - registers.a);
    }
    registers.set_flags(Flags::H, ((operand & 0x0F) + (registers.a & 0x0F)) & 0x10 != 0);
    (operand + registers.a)
}

pub fn sub(operand: u8, carry: bool, registers: &mut Registers) -> u8 {
    registers.set_flags(Flags::N, true);
    add(!operand, carry, registers)
}

pub fn cp(operand: u8, carry: bool, registers: &mut Registers) -> u8 {
    sub(operand, carry, registers)
}
