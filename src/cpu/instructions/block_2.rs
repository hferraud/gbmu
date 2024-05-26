use std::io;

use crate::mmu::MMU;
use crate::cpu::instructions::alu;
use crate::cpu::registers::Registers;

pub fn execute(opcode: u8, registers: &mut Registers, mmu: &mut MMU) -> Result<(), io::Error> {
    alu::alu_register(opcode, registers, mmu)
}
