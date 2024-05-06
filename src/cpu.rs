use std::{io, mem};
use crate::mmu::MMU;

mod alu;
mod instructions;
mod loader;
mod registers;

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
    pub fn fetch_next_word(&mut self, mmu: &mut MMU) -> Result<u8, io::Error> {
        let word = mmu.get_word(self.registers.pc as usize)?;
        self.registers.pc += mem::size_of::<u8>() as u16;
        Ok(word)
    }

   pub unsafe fn fetch_next_dword(&mut self, mmu: &mut MMU) -> Result<u16, io::Error> {
       let dword = mmu.get_dword(self.registers.pc as usize)?;
       self.registers.pc = mem::size_of::<u16>() as u16;
       Ok(dword)
   }
}
