use crate::app::instruction_map::{Instruction, InstructionMap};
use crate::error;
use crate::gameboy::Gameboy;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{collections::HashSet, env, thread};

const PREFIXED_OPCODE: u8 = 0xCB;

pub struct GameData {
    pub gameboy: Gameboy,
    pub instructions: Vec<(u16, Instruction)>,
    pub breakpoints: HashSet<u16>,
    pub run_status: RunStatus,
    pub join_handle: Option<JoinHandle<Result<()>>>,
}

#[derive(Eq, PartialEq)]
pub enum RunStatus {
    Running,
    Waiting,
    Stop,
    Error,
}

impl GameData {
    pub fn new(_rom_path: &str, instruction_map: &InstructionMap) -> Result<Arc<Mutex<Self>>> {
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

        let game_data = Arc::new(Mutex::new(GameData {
            gameboy,
            instructions,
            breakpoints,
            run_status: RunStatus::Waiting,
            join_handle: None,
        }));

        let thread_game_data = Arc::clone(&game_data);
        game_data
            .lock()
            .expect("game_data mutex is poisoned")
            .join_handle = Some(thread::spawn(move || Self::routine(thread_game_data)));

        Ok(game_data)
    }

    fn routine(game_data___: Arc<Mutex<GameData>>) -> Result<()> {
        loop {
            let mut game_data = game_data___.lock().expect("game_data mutex is poisoned");

            match game_data.run_status {
                RunStatus::Running => {
                    // TODO handle error in parent
                    game_data.gameboy.run_instruction().inspect_err(|_| {
                        game_data.run_status = RunStatus::Error;
                    })?;

                    if game_data
                        .breakpoints
                        .contains(&game_data.gameboy.cpu.registers.pc)
                    {
                        game_data.run_status = RunStatus::Waiting;
                    }
                    // TODO sleep()
                }
                RunStatus::Waiting => continue,
                RunStatus::Stop => return Ok(()),
                RunStatus::Error => return Err(anyhow!("Undefined error")),
            }
        }
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
                opcode = rom[i + 1];
                Self::get_instruction(opcode, i, &instruction_map.cbprefixed)?
            } else {
                Self::get_instruction(opcode, i, &instruction_map.unprefixed)?
            };

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
            .cloned()
    }
}
