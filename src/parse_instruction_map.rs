use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

const INSTRUCTION_MAP_PATH: &str = "assets/instruction_map.json";

#[derive(Serialize, Deserialize)]
pub struct InstructionMap {
    unprefixed: HashMap<u8, Instruction>,
    prefixed: HashMap<u8, Instruction>,
}

#[derive(Serialize, Deserialize)]
pub struct Instruction {
    name: String,
    bytes: usize,
    operands: Operands,
}

#[derive(Serialize, Deserialize)]
pub struct Operands {
    name: String,
    bytes: Option<usize>,
}

pub fn parse_instruction_map(rom: Vec<u8>) -> Result<InstructionMap> {
    let file = File::open(INSTRUCTION_MAP_PATH)?;
    let reader = BufReader::new(file);
    let json = serde_json::from_reader(reader);
}
