use std::{io, mem};

use crate::cpu::instructions::alu;
use crate::cpu::instructions::prefix;
use crate::cpu::registers::{Flags, Registers};
use crate::cpu::{DWord, CPU};
use crate::error;
use crate::mmu::MMU;

const LDH_ADDRESS_START: usize = 0xff00;

const INSTRUCTION_TYPE_MASK: u8 = 0b00100111;
const STK_INSTRUCTION_TYPE_MASK: u8 = 0b00001111;
const RST_INSTRUCTION_MASK: u8 = 0b00000111;

const RET_CC_OPCODE: u8 = 0b000;
const JP_CC_IMM16_OPCODE: u8 = 0b010;
const CALL_CC_IMM16_OPCODE: u8 = 0b100;
const ALU_OPCODE: u8 = 0b110;
const RST_OPCODE: u8 = 0b111;

const POP_OPCODE: u8 = 0b0001;
const PUSH_OPCODE: u8 = 0b0101;

const RET_OPCODE: u8 = 0b11001001;
const RETI_OPCODE: u8 = 0b11011001;
const JP_IMM16_OPCODE: u8 = 0b11000011;
const JP_HL_OPCODE: u8 = 0b11101001;
const CALL_IMM16_OPCODE: u8 = 0b11001101;

const PREFIX_OPCODE: u8 = 0b11001011;

const LDH_CMEM_A_OPCODE: u8 = 0b11100010;
const LDH_IMM8_A_OPCODE: u8 = 0b11100000;
const LD_IMM16_A_OPCODE: u8 = 0b11101010;
const LDH_A_CMEM_OPCODE: u8 = 0b11110010;
const LDH_A_IMM8_OPCODE: u8 = 0b11110000;
const LD_A_IMM16_OPCODE: u8 = 0b11111010;

const ADD_SP_IMM8_OPCODE: u8 = 0b11101000;
const LD_HL_SP_IMM8_OPCODE: u8 = 0b11111000;
const LD_SP_HL_OPCODE: u8 = 0b11111001;

const DI_OPCODE: u8 = 0b11110011;
const EI_OPCODE: u8 = 0b11111011;

const CC_MASK: u8 = 0b00011000;
const CC_SHIFT: u8 = 3;

const NZ_CC: u8 = 0;
const Z_CC: u8 = 1;
const NC_CC: u8 = 2;
const C_CC: u8 = 3;

const TGT_MASK: u8 = 0b00111000;

pub fn execute(opcode: u8, cpu: &mut CPU, mmu: &mut MMU) -> Result<(), io::Error> {
    let registers = &mut cpu.registers;
    if opcode & RST_INSTRUCTION_MASK == RST_OPCODE {
        return rst_tgt3(opcode, registers, mmu);
    }
    match opcode & INSTRUCTION_TYPE_MASK {
        RET_CC_OPCODE => return ret_cc(opcode, registers, mmu),
        JP_CC_IMM16_OPCODE => return jp_cc_imm16(opcode, cpu, mmu),
        CALL_CC_IMM16_OPCODE => return call_cc_imm16(opcode, cpu, mmu),
        ALU_OPCODE => return alu_imm8(opcode, cpu, mmu),
        _ => {}
    };
    match opcode & STK_INSTRUCTION_TYPE_MASK {
        POP_OPCODE => return pop_r16stk(opcode, registers, mmu),
        PUSH_OPCODE => return push_r16stk(opcode, registers, mmu),
        _ => {}
    };
    match opcode {
        RET_OPCODE => return ret(registers, mmu),
        RETI_OPCODE => reti(cpu, mmu)?,
        JP_IMM16_OPCODE => return jp_imm16(cpu, mmu),
        JP_HL_OPCODE => jp_hl(cpu),
        CALL_IMM16_OPCODE => return call_imm16(cpu, mmu),

        PREFIX_OPCODE => return prefix(cpu, mmu),
        LDH_CMEM_A_OPCODE => return ldh_cmem_a(registers, mmu),
        LDH_IMM8_A_OPCODE => return ldh_imm8_a(cpu, mmu),
        LD_IMM16_A_OPCODE => return ld_imm16_a(cpu, mmu),
        LDH_A_CMEM_OPCODE => return ldh_a_cmem(registers, mmu),
        LDH_A_IMM8_OPCODE => return ldh_a_imm8(cpu, mmu),
        LD_A_IMM16_OPCODE => return ld_a_imm16(cpu, mmu),

        ADD_SP_IMM8_OPCODE => return add_sp_imm8(cpu, mmu),
        LD_HL_SP_IMM8_OPCODE => return ld_hl_sp_imm8(cpu, mmu),
        LD_SP_HL_OPCODE => ld_sp_hl(registers),

        DI_OPCODE => di(cpu),
        EI_OPCODE => ei(cpu),
        _ => return Err(error::invalid_opcode()),
    };
    Ok(())
}

fn alu_imm8(opcode: u8, cpu: &mut CPU, mmu: &mut MMU) -> Result<(), io::Error> {
    alu::alu(opcode, cpu.fetch_next_word(mmu)?, &mut cpu.registers)
}

fn prefix(cpu: &mut CPU, mmu: &mut MMU) -> Result<(), io::Error> {
    let opcode = cpu.fetch_next_word(mmu)?;
    prefix::execute(opcode, cpu, mmu)
}

fn push(value: u16, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    registers.sp -= mem::size_of::<DWord>() as u16;
    mmu.set_dword(registers.sp as usize, value)
}

fn pop(registers: &mut Registers, mmu: &mut MMU) -> Result<u16, io::Error> {
    let result = mmu.get_dword(registers.sp as usize)?;
    registers.sp += mem::size_of::<DWord>() as u16;
    Ok(result)
}

fn push_r16stk(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    let register = super::get_r16_code(opcode);
    let value = registers.get_dword_stk(register)?;
    push(value, registers, mmu)
}

fn pop_r16stk(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    let register = super::get_r16_code(opcode);
    let value: u16 = pop(registers, mmu)?;
    registers.set_dword_stk(register, value)
}

fn ret(registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    let value = pop(registers, mmu)?;
    registers.pc = value;
    Ok(())
}

fn reti(cpu: &mut CPU, mmu: &mut MMU) -> Result<(), io::Error> {
    cpu.ime = true;
    ret(&mut cpu.registers, mmu)
}

fn ret_cc(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    if check_cc(opcode, registers)? {
        ret(registers, mmu)?;
    }
    Ok(())
}

fn check_cc(opcode: u8, registers: &mut Registers) -> Result<bool, io::Error> {
    match (opcode & CC_MASK) >> CC_SHIFT {
        NZ_CC => Ok(!registers.get_flag(Flags::Z)),
        Z_CC => Ok(registers.get_flag(Flags::Z)),
        NC_CC => Ok(!registers.get_flag(Flags::C)),
        C_CC => Ok(registers.get_flag(Flags::C)),
        _ => Err(error::invalid_condition_code()),
    }
}

fn jp_imm16(cpu: &mut CPU, mmu: &mut MMU) -> Result<(), io::Error> {
    let imm16 = cpu.fetch_next_dword(mmu)?;
    cpu.registers.pc = imm16;
    Ok(())
}

fn jp_cc_imm16(opcode: u8, cpu: &mut CPU, mmu: &mut MMU) -> Result<(), io::Error> {
    let imm16 = cpu.fetch_next_dword(mmu)?;

    if check_cc(opcode, &mut cpu.registers)? {
        cpu.registers.pc = imm16;
    }
    Ok(())
}

fn jp_hl(cpu: &mut CPU) {
    let hl = cpu.registers.get_hl();
    cpu.registers.pc = hl;
}

fn call(fn_address: u16, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    push(registers.pc, registers, mmu)?;

    registers.pc = fn_address;

    Ok(())
}

fn call_imm16(cpu: &mut CPU, mmu: &mut MMU) -> Result<(), io::Error> {
    let imm16 = cpu.fetch_next_dword(mmu)?;

    call(imm16, &mut cpu.registers, mmu)
}

fn call_cc_imm16(opcode: u8, cpu: &mut CPU, mmu: &mut MMU) -> Result<(), io::Error> {
    let imm16 = cpu.fetch_next_dword(mmu)?;

    if check_cc(opcode, &mut cpu.registers)? {
        call(imm16, &mut cpu.registers, mmu)?;
    }
    Ok(())
}

fn rst_tgt3(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    let target = opcode & TGT_MASK;

    call(target as u16, registers, mmu)
}

fn ldh_cmem_a(registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    let address = LDH_ADDRESS_START + registers.c as usize;
    mmu.set_word(address, registers.a)
}

fn ldh_a_cmem(registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    let address = LDH_ADDRESS_START + registers.c as usize;
    registers.a = mmu.get_word(address)?;
    Ok(())
}

fn ldh_imm8_a(cpu: &mut CPU, mmu: &mut MMU) -> Result<(), io::Error> {
    let address = LDH_ADDRESS_START + cpu.fetch_next_word(mmu)? as usize;
    mmu.set_word(address, cpu.registers.a)
}

fn ldh_a_imm8(cpu: &mut CPU, mmu: &mut MMU) -> Result<(), io::Error> {
    let address = LDH_ADDRESS_START + cpu.fetch_next_word(mmu)? as usize;
    cpu.registers.a = mmu.get_word(address)?;
    Ok(())
}

fn ld_imm16_a(cpu: &mut CPU, mmu: &mut MMU) -> Result<(), io::Error> {
    let address = cpu.fetch_next_dword(mmu)?;
    mmu.set_word(address as usize, cpu.registers.a)
}

fn ld_a_imm16(cpu: &mut CPU, mmu: &mut MMU) -> Result<(), io::Error> {
    let address = cpu.fetch_next_dword(mmu)?;
    cpu.registers.a = mmu.get_word(address as usize)?;
    Ok(())
}

fn add_sp_imm8(cpu: &mut CPU, mmu: &mut MMU) -> Result<(), io::Error> {
    let value = cpu.fetch_next_word(mmu)?;

    cpu.registers.reset_flags();
    let (result, overflow) = cpu.registers.sp.overflowing_add(value as u16);
    cpu.registers.set_flags(Flags::C, overflow);
    cpu.registers.set_h_flag_add(cpu.registers.sp as u8, value);
    cpu.registers.sp = result;
    Ok(())
}

fn ld_hl_sp_imm8(cpu: &mut CPU, mmu: &mut MMU) -> Result<(), io::Error> {
    add_sp_imm8(cpu, mmu)?;

    ld_sp_hl(&mut cpu.registers);
    Ok(())
}

fn ld_sp_hl(registers: &mut Registers) {
    registers.set_hl(registers.sp);
}

fn di(cpu: &mut CPU) {
    cpu.ime = false;
}

fn ei(cpu: &mut CPU) {
    cpu.ime = true;
}
