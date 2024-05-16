use crate::cpu::alu::alu_register;
use crate::cpu::registers::Registers;
use std::io;

pub fn execute(opcode: u8, registers: &mut Registers) -> Result<(), io::Error> {
    alu_register(opcode, registers)
}
