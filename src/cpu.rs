mod instructions;
mod registers;

use crate::mmu::MMU;
use std::{io, mem};

pub const BLOCK_MASK: u8 = 0b11000000;
pub const BLOCK_SHIFT: u8 = 6;
pub const BLOCK_0: u8 = 0b00;
pub const BLOCK_1: u8 = 0b01;
pub const BLOCK_2: u8 = 0b10;
pub const BLOCK_3: u8 = 0b11;

pub type Word = u8;
pub type DWord = u16;

pub struct CPU {
    registers: registers::Registers,
    ime: bool,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: registers::Registers::new(),
            ime: false,
        }
    }

    pub fn run(&mut self, mmu: &mut MMU) -> Result<(), io::Error> {
        let mut breakpoint: bool = false;
        loop {
            if breakpoint {
                println!("New instruction:");
                println!("PC: {:#06x}", self.registers.pc);
                println!("Opcode");
            }
            let word = self.fetch_next_word(mmu)?;
            if breakpoint {
                println!("Instruction:");
            }
            instructions::execute(word, self, mmu)?;
            if breakpoint {
                println!("{:#x?}", self.registers);
                println!();
            }
            if self.registers.pc == 0x29a {
                breakpoint = true;
            }
            if breakpoint {
                let mut input = String::new();
                io::stdin().read_line(&mut input)
                    .expect("Erreur de lecture");
                if input.trim() == "q" {
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn fetch_next_word(&mut self, mmu: &mut MMU) -> Result<u8, io::Error> {
        let word = mmu.get_word(self.registers.pc as usize)?;
        println!("word: {:#04x} = {:#010b}", word, word);
        self.registers.pc += mem::size_of::<Word>() as DWord;
        Ok(word)
    }

    pub fn fetch_next_dword(&mut self, mmu: &mut MMU) -> Result<u16, io::Error> {
        let dword = mmu.get_dword(self.registers.pc as usize)?;
        println!("dword: {:#06x} = {:#018b}", dword, dword);
        self.registers.pc += mem::size_of::<DWord>() as DWord;
        Ok(dword)
    }
}
