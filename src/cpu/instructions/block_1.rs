use crate::cpu::loader::load;
use crate::cpu::registers::Registers;
use std::io;

pub fn execute(opcode: u8, registers: &mut Registers) -> Result<(), io::Error> {
    load(opcode, registers)
}
