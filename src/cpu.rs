mod registers;
mod alu;
mod loader;


pub const BLOCK_MASK: u8 = 0b11000000;
pub const BLOCK_SHIFT: u8 = 6;
pub const BLOCK_0: u8 = 0b00;
pub const BLOCK_1: u8 = 0b01;
pub const BLOCK_2: u8 = 0b10;
pub const BLOCK_3: u8 = 0b11;


pub struct CPU {
    registers: registers::Registers,
}

impl CPU {

}