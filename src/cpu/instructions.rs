use crate::cpu::CPU;
use crate::error;
use crate::mmu::MMU;

mod alu;
mod block_0;
mod block_1;
mod block_2;
mod block_3;
mod loader;
mod prefix;

pub const R16_MASK: u8 = 0b00110000;
pub const R16_SHIFT: u8 = 4;
pub const R8_MASK: u8 = 0b00111000;
pub const R8_SHIFT: u8 = 3;

const BLOCK_MASK: u8 = 0b11000000;

const BLOCK_0_CODE: u8 = 0b00000000;
const BLOCK_1_CODE: u8 = 0b01000000;
const BLOCK_2_CODE: u8 = 0b10000000;
const BLOCK_3_CODE: u8 = 0b11000000;

fn get_r16_code(opcode: u8) -> u8 {
    (opcode & R16_MASK) >> R16_SHIFT
}

fn get_r8_code(opcode: u8) -> u8 {
    (opcode & R8_MASK) >> R8_SHIFT
}

pub fn execute(opcode: u8, cpu: &mut CPU, mmu: &mut MMU) -> Result<(), std::io::Error> {
    match opcode & BLOCK_MASK {
        BLOCK_0_CODE => block_0::execute(opcode, cpu, mmu),
        BLOCK_1_CODE => block_1::execute(opcode, &mut cpu.registers, mmu),
        BLOCK_2_CODE => block_2::execute(opcode, &mut cpu.registers, mmu),
        BLOCK_3_CODE => block_3::execute(opcode, cpu, mmu),
        _ => Err(error::invalid_opcode()),
    }
}
