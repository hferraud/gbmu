use std::io;

use crate::cpu::registers::Flags;
use crate::cpu::registers::Registers;
use crate::error;
use crate::mmu::MMU;

const OPERATION_MASK: u8 = 0b00111000;
const OPERATION_SHIFT: u8 = 3;
const REGISTER_MASK: u8 = 0b00000111;
const OPERATION_TYPE_MASK: u8 = 0b100;
const NEGATION_MASK: u8 = 0b010;
const CARRY_FLAG_MASK: u8 = 0b001;
const SUM_OPERATION: u8 = 0b000;
const BITWISE_OPERATION: u8 = 0b100;
const ADD_OPERATION: u8 = 0b000;
const ADC_OPERATION: u8 = 0b001;
const SUB_OPERATION: u8 = 0b010;
const SBC_OPERATION: u8 = 0b011;
const AND_OPERATION: u8 = 0b100;
const XOR_OPERATION: u8 = 0b101;
const OR_OPERATION: u8 = 0b110;
const CP_OPERATION: u8 = 0b111;

pub fn alu(opcode: u8, operand: u8, registers: &mut Registers) -> Result<(), io::Error> {
    let operation = (opcode & OPERATION_MASK) >> OPERATION_SHIFT;

    registers.reset_flags();
    let result = match operation & OPERATION_TYPE_MASK {
        SUM_OPERATION => sum_operation(operation, operand, registers)?,
        BITWISE_OPERATION => bitwise_operation(operation, operand, registers)?,
        _ => return Err(error::invalid_opcode()),
    };
    alu_post_process(operation, result, registers)
}

pub fn alu_register(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    let src_register = opcode & REGISTER_MASK;
    let src_value = registers.get_word(src_register, mmu)?;

    alu(opcode, src_value, registers)
}

fn alu_post_process(operation: u8, result: u8, registers: &mut Registers) -> Result<(), io::Error> {
    if result == 0 {
        registers.set_flags(Flags::Z, true);
    }
    if operation != CP_OPERATION {
        registers.a = result;
    }
    Ok(())
}

fn sum_operation(operation: u8, operand: u8, registers: &mut Registers) -> Result<u8, io::Error> {
    let carry_flag = operation & CARRY_FLAG_MASK != 0;
    let negation = operation & NEGATION_MASK != 0;

    match negation {
        false => add(operand, carry_flag, registers),
        true => sub(operand, carry_flag, registers),
    }
}

fn bitwise_operation(
    operation: u8,
    operand: u8,
    registers: &mut Registers,
) -> Result<u8, io::Error> {
    match operation {
        AND_OPERATION => and(operand, registers),
        XOR_OPERATION => Ok(registers.a ^ operand),
        OR_OPERATION => Ok(registers.a | operand),
        CP_OPERATION => cp(operand, true, registers),
        _ => Err(error::invalid_opcode()),
    }
}

fn add(operand: u8, carry: bool, registers: &mut Registers) -> Result<u8, io::Error> {
    let a = registers.a;
    let (result, overflow) = a.overflowing_add(operand);
    if carry {
        registers.set_flags(Flags::C, overflow);
    }
    registers.set_h_flag_add(operand, a);
    Ok(result)
}

fn sub(operand: u8, carry: bool, registers: &mut Registers) -> Result<u8, io::Error> {
    let a = registers.a;
    let (result, overflow) = a.overflowing_sub(operand);
    
    if carry {
        registers.set_flags(Flags::C, overflow)
    }
    registers.set_flags(Flags::N, true);
    registers.set_h_flag_sub(a, operand);
    Ok(result)
}

fn and(operand: u8, registers: &mut Registers) -> Result<u8, io::Error> {
    registers.set_flags(Flags::H, true);
    Ok(registers.a & operand)
}

fn cp(operand: u8, carry: bool, registers: &mut Registers) -> Result<u8, io::Error> {
    sub(operand, carry, registers)
}
