mod block_0;
mod block_1;
mod block_2;
mod block_3;

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
