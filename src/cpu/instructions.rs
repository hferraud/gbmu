pub const R16_MASK: u8 = 0b00110000;
pub const R16_SHIFT: u8 = 4;
pub const R8_MASK: u8 = 0b00111000;
pub const R8_SHIFT: u8 = 3;

fn get_r16_code(opcode: u8) -> u8 {
    (opcode & R16_MASK) >> R16_SHIFT
}

fn get_r8_code(opcode: u8) -> u8 {
    (opcode & R8_MASK) >> R8_SHIFT
}

mod block_0 {
    use crate::cpu::registers::{Flags, Registers, A_REGISTER_CODE};
    use crate::error;
    use crate::mmu::MMU;
    use std::io;
    use crate::cpu::CPU;

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

    pub fn execute(cpu: &mut CPU, opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
        match opcode & INSTRUCTION_TYPE_MASK {
            LD_R16_IMM16_OPCODE => return ld_r16_imm16(cpu, opcode, registers, mmu),
            LD_R16MEM_A_OPCODE => return ld_r16mem_a(opcode, registers, mmu),
            LD_A_R16MEM_OPCODE => return ld_a_r16mem(opcode, registers, mmu),
            LD_IMM16MEM_SP_OPCODE => return ld_imm16mem_sp(cpu, registers, mmu),
            INC_R16_OPCODE => return inc_r16(opcode, registers),
            DEC_R16_OPCODE => return dec_r16(opcode, registers),
            ADD_HL_R16_OPCODE => return add_hl_r16(opcode, registers),
            _ => {}
        }
        match opcode & EXTENDED_INSTRUCTION_TYPE_MASK {
            INC_R8_OPCODE => return inc_r8(opcode, registers),
            DEC_R8_OPCODE => return dec_r8(opcode, registers),
            _ => {}
        }
        match opcode {
            RLCA_OPCODE => rlca(registers),
            RRCA_OPCODE => rrca(registers),
            RLA_OPCODE => rla(registers),
            RRA_OPCODE => rra(registers),
            DAA_OPCODE => daa(registers),
            CPL_OPCODE => cpl(registers),
            SCF_OPCODE => {
                scf(registers);
                Ok(())
            }
            CCF_OPCODE => {
                ccf(registers);
                Ok(())
            }
            JR_IMM8_OPCODE => jr_imm8(cpu, registers, mmu),
            JR_NZ_IMM8_OPCODE => jr_cc_imm8(cpu, registers, mmu, !registers.get_flag(Flags::Z)),
            JR_Z_IMM8_OPCODE => jr_cc_imm8(cpu, registers, mmu, registers.get_flag(Flags::Z)),
            JR_NC_IMM8_OPCODE => jr_cc_imm8(cpu, registers, mmu, !registers.get_flag(Flags::C)),
            JR_C_IMM8_OPCODE => jr_cc_imm8(cpu, registers, mmu, registers.get_flag(Flags::C)),
            _ => Err(error::unsupported_instruction()),
        }
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

    fn inc_r8(opcode: u8, registers: &mut Registers) -> Result<(), io::Error> {
        let register = super::get_r8_code(opcode);
        let register_value = registers.get_word(register)?;

        registers.set_h_flag(register_value, 1);
        registers.set_word(register, register_value + 1)
    }

    fn dec_r8(opcode: u8, registers: &mut Registers) -> Result<(), io::Error> {
        let register = super::get_r8_code(opcode);
        let register_value = registers.get_word(register)?;

        registers.set_h_flag(register_value, !1);
        registers.set_word(register_value, register_value - 1)
    }

    fn ld_r16mem_a(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
        mmu.set_word(
            registers.get_dword(super::get_r16_code(opcode))? as usize,
            registers.get_word(A_REGISTER_CODE)?,
        )
    }

    fn ld_a_r16mem(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
        registers.set_word(
            A_REGISTER_CODE,
            *mmu.fetch_word_address(registers.get_dword(super::get_r16_code(opcode))? as usize)?,
        )
    }

    fn ld_r16_imm16(cpu: &mut CPU, opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
        registers.set_dword(super::get_r16_code(opcode), cpu.fetch_next_dword(mmu)?)
    }

    fn ld_imm16mem_sp(cpu: &mut CPU, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
        let address = cpu.fetch_next_dword(mmu)?;
        *mmu.fetch_dword_address(address as usize)? = registers.sp;
        Ok(())
    }

    fn rlca(registers: &mut Registers) -> Result<(), io::Error> {
        let a = registers.get_word(A_REGISTER_CODE)?;
        registers.set_flags(Flags::C, a & 0b10000000 != 0);
        registers.set_word(A_REGISTER_CODE, a << 1)
    }

    fn rrca(registers: &mut Registers) -> Result<(), io::Error> {
        let a = registers.get_word(A_REGISTER_CODE)?;
        registers.set_flags(Flags::C, a & 0b00000001 != 0);
        registers.set_word(A_REGISTER_CODE, a >> 1)
    }

    fn rla(registers: &mut Registers) -> Result<(), io::Error> {
        let a = registers.get_word(A_REGISTER_CODE)?;
        let carry = registers.get_flag(Flags::C) as u8;
        registers.set_flags(Flags::C, a & 0b10000000 != 0);
        registers.set_word(A_REGISTER_CODE, (a << 1) | carry)
    }

    fn rra(registers: &mut Registers) -> Result<(), io::Error> {
        let a = registers.get_word(A_REGISTER_CODE)?;
        let carry = registers.get_flag(Flags::C) as u8;
        registers.set_flags(Flags::C, a & 0b00000001 != 0);
        registers.set_word(A_REGISTER_CODE, (a >> 1) | (carry << 7))
    }

    fn daa(registers: &mut Registers) -> Result<(), io::Error> {
        let op = match registers.get_flag(Flags::N) {
            true => |a: u8, correction: u8| a - correction,
            false => |a: u8, correction: u8| a + correction,
        };

        let mut a = registers.get_word(A_REGISTER_CODE)?;
        if registers.get_flag(Flags::H) || a & 0xF > 9 {
            a = op(a, 0x6);
        }
        if registers.get_flag(Flags::C) || (a & 0xF0) >> 4 > 9 {
            a = op(a, 0x60);
        }

        registers.set_flags(Flags::Z, a == 0);
        // N is left untouched
        registers.set_flags(Flags::H, false);
        registers.set_flags(Flags::C, (a & 0xF0) >> 4 > 9);
        registers.set_word(A_REGISTER_CODE, a)
    }

    fn cpl(registers: &mut Registers) -> Result<(), io::Error> {
        registers.set_flags(Flags::N, true);
        registers.set_flags(Flags::H, true);
        registers.set_word(A_REGISTER_CODE, !registers.get_word(A_REGISTER_CODE)?)
    }

    fn scf(registers: &mut Registers) {
        registers.set_flags(Flags::N, false);
        registers.set_flags(Flags::H, false);
        registers.set_flags(Flags::C, true);
    }

    fn ccf(registers: &mut Registers) {
        registers.set_flags(Flags::C, !registers.get_flag(Flags::C));
    }

    fn jr_imm8(cpu: &mut CPU, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
        jr_cc_imm8(cpu, registers, mmu, true)
    }

    fn jr_cc_imm8(cpu: &mut CPU, registers: &mut Registers, mmu: &mut MMU, cc: bool) -> Result<(), io::Error> {
        let relative = imm8_to_jr(cpu, mmu)?;

        if !cc {
            return Ok(());
        } else if relative < 0 {
            registers.pc -= (-relative) as u16;
        } else {
            registers.pc += relative as u16;
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
}

mod block_1 {
    use crate::cpu::loader::load;
    use crate::cpu::registers::Registers;
    use std::io;

    pub fn execute(opcode: u8, registers: &mut Registers) -> Result<(), io::Error> {
        load(opcode, registers)
    }
}

mod block_2 {
    use crate::cpu::alu::alu_register;
    use crate::cpu::registers::Registers;
    use std::io;

    pub fn execute(opcode: u8, registers: &mut Registers) -> Result<(), io::Error> {
        alu_register(opcode, registers)
    }
}

mod block_3 {
    use std::{io, mem};
    use crate::error;
    use crate::cpu::{CPU, DWord};
    use crate::mmu::MMU;
    use crate::cpu::alu::alu;
    use crate::cpu::registers::{Flags, Registers};

    const INSTRUCTION_TYPE_MASK: u8 = 0b111;

    const RET_COND_OPCODE: u8 = 0b000;
    const JMP_COND_IMM16_OPCODE: u8 = 0b010;
    const CALL_COND_IMM16_OPCODE: u8 = 0b100;
    const ALU_OPCODE: u8 = 0b110;

    const RET_OPCODE: u8 = 0b11001001;
    const RETI_OPCODE: u8 = 0b11011001;
    const JP_IMM16_OPCODE: u8 = 0b11000011;
    const JP_HL_OPCODE: u8 = 0b11101001;
    const CALL_IMM16_OPCODE: u8 = 0b11001101;

    const COND_MASK: u8 = 0b00011000;
    const COND_SHIFT: u8 = 3;

    const NZ_COND_CODE: u8 = 0;
    const Z_COND_CODE: u8 = 1;
    const NC_COND_CODE: u8 = 2;
    const C_COND_CODE: u8 = 3;

    pub fn execute(cpu: &mut CPU, opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
        match opcode & INSTRUCTION_TYPE_MASK {
            RET_COND_OPCODE => ret_cond(opcode, registers, mmu)?,
            // JMP_COND_IMM16_OPCODE => ,
            // CALL_COND_IMM16_OPCODE => ,
            ALU_OPCODE => alu_imm8(cpu, opcode, registers, mmu)?,
            _ => {}
        }
        match opcode {
            RET_OPCODE => ret(registers, mmu),
            // RETI_OPCODE => ,
            // JP_IMM16_OPCODE => ,
            // JP_HL_OPCODE => ,
            // CALL_IMM16_OPCODE => ,
            _ => Err(error::invalid_opcode())
        }
    }

    pub fn alu_imm8(cpu: &mut CPU, opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
        alu(opcode, cpu.fetch_next_word(mmu)?, registers)
    }

    fn push(value: u16, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
        mmu.set_dword(registers.get_sp() as usize, value)?;
        registers.set_sp(registers.get_sp() + mem::size_of::<DWord>() as u16);
        Ok(())
    }

    fn pop(registers: &mut Registers, mmu: &mut MMU) -> Result<u16, io::Error> {
        let result = mmu.get_dword(registers.get_sp() as usize)?;
        registers.set_sp(registers.get_sp() - mem::size_of::<DWord>() as u16);
        Ok(result)
    }

    /// *SP++ = register
    pub fn push_r16stk(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
        let register = super::get_r16_code(opcode);
        let value = registers.get_dword_stk(register)?;
        push(value, registers, mmu)
    }

    /// register = *SP--
    pub fn pop_r16stk(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
        let register = super::get_r16_code(opcode);
        let value: u16 = pop(registers, mmu)?;
        registers.set_dword_stk(register, value)
    }


    pub fn ret(registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
        let value = pop(registers, mmu)?;
        registers.pc = value;
        Ok(())
    }

    pub fn ret_cond(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
        let cond = get_cond_code(opcode);
        let result = match cond {
            NZ_COND_CODE => !registers.get_flag(Flags::Z),
            Z_COND_CODE => registers.get_flag(Flags::Z),
            NC_COND_CODE => !registers.get_flag(Flags::C),
            C_COND_CODE => registers.get_flag(Flags::C),
            _ => return Err(error::invalid_condition_code()),
        };
        if result {
            ret(registers, mmu)?;
        }
        Ok(())
    }

    fn get_cond_code(opcode: u8) -> u8 {
        (opcode & COND_MASK) >> COND_SHIFT
    }

    pub fn jp_imm16(cpu: &mut CPU, mmu: &mut MMU) -> Result<(), io::Error> {
        let imm16 = cpu.fetch_next_dword(mmu)?;
        cpu.registers.pc = imm16;
        Ok(())
    }
}
