use crate::app::instruction_map::{Instruction, InstructionMap};
use crate::error;
use crate::gameboy::Gameboy;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::{collections::HashSet, env, io};

const PREFIXED_OPCODE: u8 = 0xCB;

pub struct GameData {
    pub gameboy: Gameboy,
    pub instructions: Vec<(u16, Instruction)>,
    pub breakpoints: HashSet<u16>,
    pub run_status: RunStatus,
}

#[derive(Eq, PartialEq)]
pub enum RunStatus {
    Waiting,
    Running,
}

impl GameData {
    pub fn new(_rom_path: &str, instruction_map: &InstructionMap) -> Result<Self> {
        // TODO remove this
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            println!("Usage: ./{} <ROM>", args[0]);
            panic!("{}", error::invalid_argument());
        }
        let rom_path = &args[1];
        // !

        let gameboy = Gameboy::new(rom_path)?;
        let instructions = Self::create_instructions_list(&gameboy, instruction_map)?;
        let breakpoints = HashSet::new();

        Ok(GameData {
            gameboy,
            instructions,
            breakpoints,
            run_status: RunStatus::Waiting,
        })
    }

    fn create_instructions_list(
        gameboy: &Gameboy,
        instruction_map: &InstructionMap,
    ) -> Result<Vec<(u16, Instruction)>> {
        let mut result = Vec::new();
        let rom = &gameboy.mmu.mbc.rom;

        let mut i = 0;
        while i < rom.len() {
            let mut opcode = rom[i];

            let instruction = if opcode == PREFIXED_OPCODE {
                // TODO crashes if rom ends with PREFIXED_OPCODE
                i += 1;
                opcode = rom[i];
                Self::get_instruction(opcode, i, &instruction_map.cbprefixed)
            } else {
                Self::get_instruction(opcode, i, &instruction_map.unprefixed)
            }?;

            let instruction_size = instruction.bytes;
            result.push((i as u16, instruction));

            i += instruction_size;
        }

        Ok(result)
    }

    fn get_instruction(
        opcode: u8,
        index: usize, // TODO remove
        instruction_map: &HashMap<u8, Instruction>,
    ) -> Result<Instruction> {
        instruction_map
            .get(&opcode)
            .ok_or(anyhow!(
                "Invalid instruction: index({}), opcode({:x} | {})",
                index,
                opcode,
                opcode
            ))
            .map(Instruction::clone)
    }
}
