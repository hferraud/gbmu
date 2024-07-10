use std::io;

use crate::cpu::registers::{Flags, Registers};
use crate::cpu::CPU;
use crate::error;
use crate::mmu::MMU;

const INSTRUCTION_TYPE_MASK: u8 = 0b11000000;
const INSTRUCTION_TYPE_SHIFT: u8 = 6;
const EXTENDED_INSTRUCTION_TYPE_MASK: u8 = 0b00111000;
const EXTENDED_INSTRUCTION_TYPE_SHIFT: u8 = 3;

const BIT_OPCODE: u8 = 0b01;
const RES_OPCODE: u8 = 0b10;
const SET_OPCODE: u8 = 0b11;

const RLC_OPCODE: u8 = 0b000;
const RRC_OPCODE: u8 = 0b001;
const RL_OPCODE: u8 = 0b010;
const RR_OPCODE: u8 = 0b011;
const SLA_OPCODE: u8 = 0b100;
const SRA_OPCODE: u8 = 0b101;
const SWAP_OPCODE: u8 = 0b110;
const SRL_OPCODE: u8 = 0b111;

const OPERAND_MASK: u8 = 0b111;
const BIT_INDEX_MASK: u8 = 0b00111000;
const BIT_INDEX_SHIFT: u8 = 3;

pub fn execute(opcode: u8, cpu: &mut CPU, mmu: &mut MMU) -> Result<(), io::Error> {
    match (opcode & INSTRUCTION_TYPE_MASK) >> INSTRUCTION_TYPE_SHIFT {
        BIT_OPCODE => return bit_b3_r8(opcode, &mut cpu.registers, mmu),
        RES_OPCODE => return res_b3_r8(opcode, &mut cpu.registers, mmu),
        SET_OPCODE => return set_b3_r8(opcode, &mut cpu.registers, mmu),
        _ => {}
    };
    match (opcode & EXTENDED_INSTRUCTION_TYPE_MASK) >> EXTENDED_INSTRUCTION_TYPE_SHIFT {
        RLC_OPCODE => rlc_r8(opcode, &mut cpu.registers, mmu),
        RRC_OPCODE => rrc_r8(opcode, &mut cpu.registers, mmu),
        RL_OPCODE => rl_r8(opcode, &mut cpu.registers, mmu),
        RR_OPCODE => rr_r8(opcode, &mut cpu.registers, mmu),
        SLA_OPCODE => sla_r8(opcode, &mut cpu.registers, mmu),
        SRA_OPCODE => sra_r8(opcode, &mut cpu.registers, mmu),
        SWAP_OPCODE => swap_r8(opcode, &mut cpu.registers, mmu),
        SRL_OPCODE => srl_r8(opcode, &mut cpu.registers, mmu),
        _ => Err(error::invalid_opcode()),
    }
}

fn get_r8_code(opcode: u8) -> u8 {
    opcode & OPERAND_MASK
}

fn get_bit_index(opcode: u8) -> u8 {
    (opcode & BIT_INDEX_MASK) >> BIT_INDEX_SHIFT
}

fn rlc_r8(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    let register = get_r8_code(opcode);
    let value = registers.get_word(register, mmu)?;
    let shifted_value = (value << 1) | (value >> 7);

    registers.reset_flags();
    registers.set_flags(Flags::Z, shifted_value == 0);
    registers.set_flags(Flags::C, (value & 0b10000000) != 0);
    registers.set_word(register, shifted_value, mmu)
}

fn rrc_r8(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    let register = get_r8_code(opcode);
    let value = registers.get_word(register, mmu)?;
    let shifted_value = (value >> 1) | (value << 7);

    registers.reset_flags();
    registers.set_flags(Flags::Z, shifted_value == 0);
    registers.set_flags(Flags::C, (value & 0b1) != 0);
    registers.set_word(register, shifted_value, mmu)
}

fn rl_r8(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    let register = get_r8_code(opcode);
    let value = registers.get_word(register, mmu)?;
    let shifted_value = (value << 1) | registers.get_flag(Flags::C) as u8;

    registers.reset_flags();
    registers.set_flags(Flags::Z, shifted_value == 0);
    registers.set_flags(Flags::C, (value & 0b1) != 0);
    registers.set_word(register, shifted_value, mmu)
}

fn rr_r8(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    let register = get_r8_code(opcode);
    let value = registers.get_word(register, mmu)?;
    let shifted_value = (value >> 1) | (registers.get_flag(Flags::C) as u8) << 7;

    registers.reset_flags();
    registers.set_flags(Flags::Z, shifted_value == 0);
    registers.set_flags(Flags::C, (value & 0b1) != 0);
    registers.set_word(register, shifted_value, mmu)
}

fn sla_r8(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    let register = get_r8_code(opcode);
    let value = registers.get_word(register, mmu)?;
    let shifted_value = value << 1;

    registers.reset_flags();
    registers.set_flags(Flags::Z, shifted_value == 0);
    registers.set_flags(Flags::C, (value & 0b10000000) != 0);
    registers.set_word(register, shifted_value, mmu)
}

fn sra_r8(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    let register = get_r8_code(opcode);
    let value = registers.get_word(register, mmu)?;
    let shifted_value = (value >> 1) | (value & 0b10000000);

    registers.reset_flags();
    registers.set_flags(Flags::Z, shifted_value == 0);
    registers.set_flags(Flags::C, (value & 0b1) != 0);
    registers.set_word(register, shifted_value, mmu)
}

fn swap_r8(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    let register = get_r8_code(opcode);
    let value = registers.get_word(register, mmu)?;
    let swap_value = (value << 4) | (value >> 4);

    registers.reset_flags();
    registers.set_flags(Flags::Z, swap_value == 0);
    registers.set_word(register, swap_value, mmu)
}

fn srl_r8(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    let register = get_r8_code(opcode);
    let value = registers.get_word(register, mmu)?;
    let shifted_value = value >> 1;

    registers.reset_flags();
    registers.set_flags(Flags::Z, shifted_value == 0);
    registers.set_flags(Flags::C, (value & 0b1) != 0);
    registers.set_word(register, shifted_value, mmu)
}

fn bit_b3_r8(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    let register = get_r8_code(opcode);
    let value = registers.get_word(register, mmu)?;
    let bit_index = get_bit_index(opcode);

    registers.set_flags(Flags::Z, (value >> bit_index) == 0);
    registers.set_flags(Flags::N, false);
    registers.set_flags(Flags::H, true);
    Ok(())
}

fn res_b3_r8(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    let register = get_r8_code(opcode);
    let value = registers.get_word(register, mmu)?;
    let bit_index = get_bit_index(opcode);

    registers.set_word(register, value & (0b0 << bit_index), mmu)
}

fn set_b3_r8(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    let register = get_r8_code(opcode);
    let value = registers.get_word(register, mmu)?;
    let bit_index = get_bit_index(opcode);

    registers.set_word(register, value | (0b1 << bit_index), mmu)
}
