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

    pub fn execute(
        cpu: &mut CPU,
        opcode: u8,
        registers: &mut Registers,
        mmu: &mut MMU,
    ) -> Result<(), io::Error> {
        match opcode & INSTRUCTION_TYPE_MASK {
            LD_R16_IMM16_OPCODE => return ld_r16_imm16(cpu, opcode, mmu),
            LD_R16MEM_A_OPCODE => return ld_r16mem_a(opcode, registers, mmu),
            LD_A_R16MEM_OPCODE => return ld_a_r16mem(opcode, registers, mmu),
            LD_IMM16MEM_SP_OPCODE => return ld_imm16mem_sp(cpu, mmu),
            INC_R16_OPCODE => return inc_r16(opcode, registers),
            DEC_R16_OPCODE => return dec_r16(opcode, registers),
            ADD_HL_R16_OPCODE => return add_hl_r16(opcode, registers),
            _ => {}
        };
        match opcode & EXTENDED_INSTRUCTION_TYPE_MASK {
            INC_R8_OPCODE => return inc_r8(opcode, registers),
            DEC_R8_OPCODE => return dec_r8(opcode, registers),
            _ => {}
        };
        match opcode {
            RLCA_OPCODE => rlca(registers),
            RRCA_OPCODE => rrca(registers),
            RLA_OPCODE => rla(registers),
            RRA_OPCODE => rra(registers),
            DAA_OPCODE => daa(registers),
            CPL_OPCODE => return cpl(registers),
            SCF_OPCODE => scf(registers),
            CCF_OPCODE => ccf(registers),
            JR_IMM8_OPCODE => return jr_imm8(cpu, mmu),
            JR_NZ_IMM8_OPCODE => return jr_cc_imm8(cpu, mmu, !registers.get_flag(Flags::Z)),
            JR_Z_IMM8_OPCODE => return jr_cc_imm8(cpu, mmu, registers.get_flag(Flags::Z)),
            JR_NC_IMM8_OPCODE => return jr_cc_imm8(cpu, mmu, !registers.get_flag(Flags::C)),
            JR_C_IMM8_OPCODE => return jr_cc_imm8(cpu, mmu, registers.get_flag(Flags::C)),
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
        let r16_code = super::get_r16_code(opcode);
        mmu.set_word(registers.get_dword(r16_code)? as usize, registers.a)
    }

    fn ld_a_r16mem(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
        let r16_code = super::get_r16_code(opcode);
        let word_address = registers.get_dword(r16_code)? as usize;
        registers.a = *mmu.fetch_word_address(word_address)?;
        Ok(())
    }

    fn ld_r16_imm16(cpu: &mut CPU, opcode: u8, mmu: &mut MMU) -> Result<(), io::Error> {
        let register = super::get_r16_code(opcode);
        let imm16 = cpu.fetch_next_dword(mmu)?;
        cpu.registers.set_dword(register, imm16)
    }

    fn ld_imm16mem_sp(cpu: &mut CPU, mmu: &mut MMU) -> Result<(), io::Error> {
        let address = cpu.fetch_next_dword(mmu)?;
        *mmu.fetch_dword_address(address as usize)? = cpu.registers.sp;
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
    use crate::cpu::alu::alu;
    use crate::cpu::registers::{Flags, Registers};
    use crate::cpu::{DWord, CPU};
    use crate::error;
    use crate::mmu::MMU;
    use std::{io, mem};

    const LDH_ADDRESS_START: usize = 0xff00;

    const INSTRUCTION_TYPE_MASK: u8 = 0b111;
    const STK_INSTRUCTION_TYPE_MASK: u8 = 0b1111;

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

    const LDH_CMEM_A_OPCODE: u8 = 0b11100010;
    const LDH_IMM8_A_OPCODE: u8 = 0b11100000;
    const LD_IMM16_A_OPCODE: u8 = 0b11101010;
    const LDH_A_CMEM_OPCODE: u8 = 0b11110010;
    const LDH_A_IMM8_OPCODE: u8 = 0b11110000;
    const LD_A_IMM16_OPCODE: u8 = 0b11111010;

    const DI_OPCODE: u8 = 0b11110011;
    const EI_OPCODE: u8 = 0b11111011;

    const CC_MASK: u8 = 0b00011000;
    const CC_SHIFT: u8 = 3;

    const NZ_CC: u8 = 0;
    const Z_CC: u8 = 1;
    const NC_CC: u8 = 2;
    const C_CC: u8 = 3;

    const TGT_MASK: u8 = 0b00111000;

    pub fn execute(cpu: &mut CPU, opcode: u8, mmu: &mut MMU) -> Result<(), io::Error> {
        let registers = &mut cpu.registers;
        match opcode & INSTRUCTION_TYPE_MASK {
            RET_CC_OPCODE => return ret_cc(opcode, registers, mmu),
            JP_CC_IMM16_OPCODE => return jp_cc_imm16(opcode, cpu, mmu),
            CALL_CC_IMM16_OPCODE => return call_cc_imm16(opcode, cpu, mmu),
            ALU_OPCODE => return alu_imm8(opcode, cpu, mmu),
            RST_OPCODE => return rst_tgt3(opcode, registers, mmu),
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

            LDH_CMEM_A_OPCODE => return ldh_cmem_a(registers, mmu),
            LDH_IMM8_A_OPCODE => return ldh_imm8_a(cpu, mmu),
            LD_IMM16_A_OPCODE => return ld_imm16_a(cpu, mmu),
            LDH_A_CMEM_OPCODE => return ldh_a_cmem(registers, mmu),
            LDH_A_IMM8_OPCODE => return ldh_a_imm8(cpu, mmu),
            LD_A_IMM16_OPCODE => return ld_a_imm16(cpu, mmu),

            DI_OPCODE => di(cpu),
            EI_OPCODE => ei(cpu),
            _ => return Err(error::invalid_opcode()),
        };
        Ok(())
    }

    fn alu_imm8(opcode: u8, cpu: &mut CPU, mmu: &mut MMU) -> Result<(), io::Error> {
        alu(opcode, cpu.fetch_next_word(mmu)?, &mut cpu.registers)
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
        mmu.set_word(address as usize, cpu.registers.a);
        Ok(())
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
        cpu.registers.set_h_flag(cpu.registers.sp as u8, value);
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
}
