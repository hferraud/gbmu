mod block_0 {
    use crate::cpu::registers::{Flags, Registers, A_REGISTER_CODE};
    use crate::error;
    use crate::mmu::MMU;
    use std::arch::aarch64::vreinterpret_u8_f32;
    use std::io;

    const INSTRUCTION_TYPE_MASK: u8 = 0b00000111;
    const EXTENDED_INSTRUCTION_TYPE_MASK: u8 = INSTRUCTION_TYPE_MASK | 0b00001000;
    const R16_MASK: u8 = 0b00110000;
    const R8_MASK: u8 = 0b00111000;
    const R16_SHIFT: u8 = 4;
    const R8_SHIFT: u8 = 3;
    const LD_R16_IMM16_OPCODE: u8 = 0b00000001;
    const LD_R16MEM_A_OPCODE: u8 = 0b00000010;
    const LD_A_R16MEM_OPCODE: u8 = 0b00001010;
    const LD_IMM16_SP_OPCODE: u8 = 0b00001000;
    const INC_R16_OPCODE: u8 = 0b00000011;
    const DEC_R16_OPCODE: u8 = 0b00001011;
    const ADD_HL_R16_OPCODE: u8 = 0b00001001;
    const INC_R8_OPCODE: u8 = 0b00000100;
    const DEC_R8_OPCODE: u8 = 0b00000101;

    const RLCA_OPCODE: u8 = 0b00000111;
    const RRCA_OPCODE: u8 = 0b00001111;
    const RLA_OPCODE: u8 = 0b00010111;
    const RRA_OPCODE: u8 = 0b00011111;
    const DAA_OPCODE: u8 = 0b00100111;
    const CPL_OPCODE: u8 = 0b00101111;
    const SCF_OPCODE: u8 = 0b00110111;
    const CCF_OPCODE: u8 = 0b00111111;

    const JR_IMM8_OPCODE: u8 = 0b00011000;
    const JR_Z_IMM8_OPCODE: u8 = 0b0010100;
    const JR_C_IMM8_OPCODE: u8 = 0b0011100;
    const JR_NZ_IMM8_OPCODE: u8 = 0b0010000;
    const JR_NC_IMM8_OPCODE: u8 = 0b0011000;

    macro_rules! get_r16_code {
        ($opcode: expr) => {
            ($opcode & R16_MASK) >> R16_SHIFT
        };
    }

    macro_rules! get_r8_code {
        ($opcode: expr) => {
            ($opcode & R8_MASK) >> R8_SHIFT
        };
    }

    pub fn execute(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
        match opcode & INSTRUCTION_TYPE_MASK {
            // LD_R16_IMM16_OPCODE => , TODO: Implement this
            LD_R16MEM_A_OPCODE => return ld_r16mem_a(opcode, registers, mmu),
            LD_A_R16MEM_OPCODE => return ld_a_r16mem(opcode, registers, mmu),
            // LD_IMM16_SP_OPCODE => , TODO: Implement this
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
            RLCA_OPCODE => return rlca(registers),
            RRCA_OPCODE => return rrca(registers),
            RLA_OPCODE => return rla(registers),
            RRA_OPCODE => return rra(registers),
            // DAA_OPCODE =>
            // CPL_OPCODE =>
            // SCF_OPCODE =>
            // CCF_OPCODE =>
            _ => Err(error::unsupported_instruction()),
        }
    }

    fn inc_r16(opcode: u8, registers: &mut Registers) -> Result<(), io::Error> {
        let register = get_r16_code!(opcode);
        let register_value = registers.get_dword(register)?;
        registers.set_dword(register, register_value + 1)
    }

    fn dec_r16(opcode: u8, registers: &mut Registers) -> Result<(), io::Error> {
        let register = get_r16_code!(opcode);
        let register_value = registers.get_dword(register)?;
        registers.set_dword(register, register_value + 1)
    }

    fn add_hl_r16(opcode: u8, registers: &mut Registers) -> Result<(), io::Error> {
        let register = get_r16_code!(opcode);
        let register_value = registers.get_dword(register)?;
        registers.set_hl(registers.get_hl() + register_value);
        Ok(())
    }
    fn inc_r8(opcode: u8, registers: &mut Registers) -> Result<(), io::Error> {
        let register = get_r8_code!(opcode);
        let register_value = registers.get_dword(register)?;
        registers.set_dword(register, register_value + 1)
    }

    fn dec_r8(opcode: u8, registers: &mut Registers) -> Result<(), io::Error> {
        let register = get_r8_code!(opcode);
        let register_value = registers.get_word(register)?;
        registers.set_word(register, register_value + 1)
    }

    fn ld_r16mem_a(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
        mmu.set_word(
            registers.get_dword(get_r16_code!(opcode))? as usize,
            registers.get_word(A_REGISTER_CODE)?,
        )
    }

    fn ld_a_r16mem(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
        registers.set_word(
            A_REGISTER_CODE,
            *mmu.fetch_word_address(registers.get_dword(get_r16_code!(opcode))? as usize)?,
        )
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
        // TODO: Not finished
        let unit = registers.get_word(A_REGISTER_CODE)? % 10;
        let decimal = registers.get_word(A_REGISTER_CODE)? / 10;
        registers.set_word(A_REGISTER_CODE, (decimal << 4) + unit)
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
    use crate::cpu::alu::alu;
    use crate::cpu::registers::Registers;
    use std::io;

    pub fn execute(opcode: u8, registers: &mut Registers) -> Result<(), io::Error> {
        alu(opcode, registers)
    }
}

mod block_3 {
    // pub fn execute(opcode: u8) -> Option<u8> {
    //
    // }
}
