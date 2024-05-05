use crate::cpu::registers::Flags;
use crate::cpu::registers::{Registers, A_REGISTER_CODE};
use crate::error;
use std::io;

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

pub fn alu(opcode: u8, registers: &mut Registers) -> Result<(), io::Error> {
    let masked_opcode = (opcode & OPCODE_MASK) >> 3;
    let src_register = opcode & REGISTER_MASK;
    let src_value = registers.get_word(src_register)?;

    registers.reset_flags();
    let result = match masked_opcode {
        ADD_OPCODE => add(src_value, false, registers)?,
        ADC_OPCODE => add(src_value, true, registers)?,
        SUB_OPCODE => sub(src_value, false, registers)?,
        SBC_OPCODE => sub(src_value, true, registers)?,
        AND_OPCODE => {
            registers.set_flags(Flags::H, true);
            registers.get_word(A_REGISTER_CODE)? & src_value
        }
        XOR_OPCODE => registers.get_word(A_REGISTER_CODE)? ^ src_value,
        OR_OPCODE => registers.get_word(A_REGISTER_CODE)? | src_value,
        CP_OPCODE => cp(src_value, true, registers)?,
        _ => return Err(error::invalid_opcode()),
    };
    if result == 0 {
        registers.set_flags(Flags::Z, true);
    }
    if masked_opcode != CP_OPCODE {
        registers.set_word(A_REGISTER_CODE, result)?;
    }
    return Ok(());
}

fn add(operand: u8, carry: bool, registers: &mut Registers) -> Result<u8, io::Error> {
    if carry {
        registers.set_flags(
            Flags::C,
            operand > u8::MAX - registers.get_word(A_REGISTER_CODE)?,
        );
    }
    registers.set_flags(
        Flags::H,
        ((operand & 0x0F) + (registers.get_word(A_REGISTER_CODE)? & 0x0F)) & 0x10 != 0,
    );
    Ok(operand + registers.get_word(A_REGISTER_CODE)?)
}

fn sub(operand: u8, carry: bool, registers: &mut Registers) -> Result<u8, io::Error> {
    registers.set_flags(Flags::N, true);
    Ok(add(!operand, carry, registers)?)
}

fn cp(operand: u8, carry: bool, registers: &mut Registers) -> Result<u8, io::Error> {
    Ok(sub(operand, carry, registers)?)
}
