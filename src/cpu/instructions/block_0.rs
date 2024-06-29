use crate::cpu::registers::{Flags, Registers};
use crate::cpu::CPU;
use crate::error;
use crate::mmu::MMU;
use std::io;

const INSTRUCTION_TYPE_MASK: u8 = 0b00000111;
const EXTENDED_INSTRUCTION_TYPE_MASK: u8 = INSTRUCTION_TYPE_MASK | 0b00001000;

const INC_R16_OPCODE: u8 = 0b00000011;
const DEC_R16_OPCODE: u8 = 0b00001011;
const ADD_HL_R16_OPCODE: u8 = 0b00001001;
const INC_R8_OPCODE: u8 = 0b00000100;
const DEC_R8_OPCODE: u8 = 0b00000101;
const LD_R16_IMM16_OPCODE: u8 = 0b00000001;
const LD_R16MEM_A_OPCODE: u8 = 0b00000010;
const LD_A_R16MEM_OPCODE: u8 = 0b00001010;
const LD_IMM16MEM_SP_OPCODE: u8 = 0b00001000;

const RLCA_OPCODE: u8 = 0b00000111;
const RRCA_OPCODE: u8 = 0b00001111;
const RLA_OPCODE: u8 = 0b00010111;
const RRA_OPCODE: u8 = 0b00011111;
const DAA_OPCODE: u8 = 0b00100111;
const CPL_OPCODE: u8 = 0b00101111;
const SCF_OPCODE: u8 = 0b00110111;
const CCF_OPCODE: u8 = 0b00111111;

const JR_IMM8_OPCODE: u8 = 0b00011000;
const JR_NZ_IMM8_OPCODE: u8 = 0b00100000;
const JR_Z_IMM8_OPCODE: u8 = 0b00101000;
const JR_NC_IMM8_OPCODE: u8 = 0b00110000;
const JR_C_IMM8_OPCODE: u8 = 0b00111000;

const NOP_OPCODE: u8 = 0b00000000;

pub fn execute(opcode: u8, cpu: &mut CPU, mmu: &mut MMU) -> Result<(), io::Error> {
    match opcode & INSTRUCTION_TYPE_MASK {
        LD_R16_IMM16_OPCODE => return ld_r16_imm16(opcode, cpu, mmu),
        LD_R16MEM_A_OPCODE => return ld_r16mem_a(opcode, &mut cpu.registers, mmu),
        LD_A_R16MEM_OPCODE => return ld_a_r16mem(opcode, &mut cpu.registers, mmu),
        LD_IMM16MEM_SP_OPCODE => return ld_imm16mem_sp(cpu, mmu),
        INC_R16_OPCODE => return inc_r16(opcode, &mut cpu.registers),
        DEC_R16_OPCODE => return dec_r16(opcode, &mut cpu.registers),
        ADD_HL_R16_OPCODE => return add_hl_r16(opcode, &mut cpu.registers),
        _ => {}
    };
    match opcode & EXTENDED_INSTRUCTION_TYPE_MASK {
        INC_R8_OPCODE => return inc_r8(opcode, &mut cpu.registers, mmu),
        DEC_R8_OPCODE => return dec_r8(opcode, &mut cpu.registers, mmu),
        _ => {}
    };
    match opcode {
        NOP_OPCODE => return Ok(()),
        RLCA_OPCODE => rlca(&mut cpu.registers),
        RRCA_OPCODE => rrca(&mut cpu.registers),
        RLA_OPCODE => rla(&mut cpu.registers),
        RRA_OPCODE => rra(&mut cpu.registers),
        DAA_OPCODE => daa(&mut cpu.registers),
        CPL_OPCODE => cpl(&mut cpu.registers),
        SCF_OPCODE => scf(&mut cpu.registers),
        CCF_OPCODE => ccf(&mut cpu.registers),
        JR_IMM8_OPCODE => return jr_imm8(cpu, mmu),
        JR_NZ_IMM8_OPCODE => return jr_cc_imm8(cpu, mmu, !cpu.registers.get_flag(Flags::Z)),
        JR_Z_IMM8_OPCODE => return jr_cc_imm8(cpu, mmu, cpu.registers.get_flag(Flags::Z)),
        JR_NC_IMM8_OPCODE => return jr_cc_imm8(cpu, mmu, !cpu.registers.get_flag(Flags::C)),
        JR_C_IMM8_OPCODE => return jr_cc_imm8(cpu, mmu, cpu.registers.get_flag(Flags::C)),
        _ => return Err(error::unsupported_instruction()),
    }
    Ok(())
}

fn inc_r16(opcode: u8, registers: &mut Registers) -> Result<(), io::Error> {
    let register = super::get_r16_code(opcode);
    let register_value = registers.get_dword(register)?;
    registers.set_dword(register, register_value + 1)
}

fn dec_r16(opcode: u8, registers: &mut Registers) -> Result<(), io::Error> {
    let register = super::get_r16_code(opcode);
    let register_value = registers.get_dword(register)?;
    registers.set_dword(register, register_value - 1)
}

fn add_hl_r16(opcode: u8, registers: &mut Registers) -> Result<(), io::Error> {
    let register = super::get_r16_code(opcode);
    let register_value = registers.get_dword(register)?;
    registers.set_hl(registers.get_hl() + register_value);
    Ok(())
}

fn inc_r8(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    let register = super::get_r8_code(opcode);
    let register_value = registers.get_word(register, mmu)?;

    registers.set_h_flag(register_value, 1);
    registers.set_word(register, register_value + 1, mmu)
}

fn dec_r8(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    let register = super::get_r8_code(opcode);
    let register_value = registers.get_word(register, mmu)?;

    registers.set_h_flag(register_value, !1);
    registers.set_word(register_value, register_value - 1, mmu)
}

fn ld_r16mem_a(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    let r16_code = super::get_r16_code(opcode);
    mmu.set_word(registers.get_dword(r16_code)? as usize, registers.a)
}

fn ld_a_r16mem(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    let r16_code = super::get_r16_code(opcode);
    let word_address = registers.get_dword(r16_code)? as usize;
    registers.a = mmu.get_word(word_address)?;
    Ok(())
}

fn ld_r16_imm16(opcode: u8, cpu: &mut CPU, mmu: &mut MMU) -> Result<(), io::Error> {
    let register = super::get_r16_code(opcode);
    let imm16 = cpu.fetch_next_dword(mmu)?;
    cpu.registers.set_dword(register, imm16)
}

fn ld_imm16mem_sp(cpu: &mut CPU, mmu: &mut MMU) -> Result<(), io::Error> {
    let address = cpu.fetch_next_dword(mmu)?;
    mmu.set_dword(address as usize, cpu.registers.sp);
    Ok(())
}

fn rlca(registers: &mut Registers) {
    registers.set_flags(Flags::C, registers.a & 0b10000000 != 0);
    registers.a <<= 1;
}

fn rrca(registers: &mut Registers) {
    registers.set_flags(Flags::C, registers.a & 0b00000001 != 0);
    registers.a >>= 1;
}

fn rla(registers: &mut Registers) {
    let carry = registers.get_flag(Flags::C) as u8;
    registers.set_flags(Flags::C, registers.a & 0b10000000 != 0);
    registers.a = (registers.a << 1) | carry;
}

fn rra(registers: &mut Registers) {
    let carry = registers.get_flag(Flags::C) as u8;
    registers.set_flags(Flags::C, registers.a & 0b00000001 != 0);
    registers.a = (registers.a >> 1) | (carry << 7);
}

fn daa(registers: &mut Registers) {
    let op = match registers.get_flag(Flags::N) {
        true => |a: u8, correction: u8| a - correction,
        false => |a: u8, correction: u8| a + correction,
    };

    if registers.get_flag(Flags::H) || registers.a & 0xF > 9 {
        registers.a = op(registers.a, 0x6);
    }
    if registers.get_flag(Flags::C) || (registers.a & 0xF0) >> 4 > 9 {
        registers.a = op(registers.a, 0x60);
    }

    registers.set_flags(Flags::Z, registers.a == 0);
    // N is left untouched
    registers.set_flags(Flags::H, false);
    registers.set_flags(Flags::C, (registers.a & 0xF0) >> 4 > 9);
}

fn cpl(registers: &mut Registers) {
    registers.set_flags(Flags::N, true);
    registers.set_flags(Flags::H, true);
    registers.a = !registers.a;
}

fn scf(registers: &mut Registers) {
    registers.set_flags(Flags::N, false);
    registers.set_flags(Flags::H, false);
    registers.set_flags(Flags::C, true);
}

fn ccf(registers: &mut Registers) {
    registers.set_flags(Flags::C, !registers.get_flag(Flags::C));
}

fn jr_imm8(cpu: &mut CPU, mmu: &mut MMU) -> Result<(), io::Error> {
    jr_cc_imm8(cpu, mmu, true)
}

fn jr_cc_imm8(cpu: &mut CPU, mmu: &mut MMU, cc: bool) -> Result<(), io::Error> {
    let relative = imm8_to_jr(cpu, mmu)?;

    if !cc {
        return Ok(());
    } else if relative < 0 {
        cpu.registers.pc -= (-relative) as u16;
    } else {
        cpu.registers.pc += relative as u16;
    }
    Ok(())
}

fn imm8_to_jr(cpu: &mut CPU, mmu: &mut MMU) -> Result<i16, io::Error> {
    let mut imm8 = cpu.fetch_next_word(mmu)? as i16;

    imm8 -= 127;
    if imm8 >= 0 {
        imm8 += 1;
    }
    Ok(imm8)
}
