use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::write;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::File;
use std::io::BufReader;

const INSTRUCTION_MAP_PATH: &str = "assets/instruction_map.json";

#[derive(Deserialize)]
pub struct InstructionMapRaw {
    unprefixed: HashMap<String, Instruction>,
    cbprefixed: HashMap<String, Instruction>,
}

pub struct InstructionMap {
    pub cbprefixed: HashMap<u8, Instruction>,
    pub unprefixed: HashMap<u8, Instruction>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Instruction {
    pub mnemonic: String,
    pub bytes: usize,
    pub operands: Vec<Operands>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Operands {
    name: String,
    bytes: Option<usize>,
}

impl InstructionMap {
    pub fn new() -> Result<InstructionMap> {
        let file = File::open(INSTRUCTION_MAP_PATH)?;
        let reader = BufReader::new(file);
        let instruction_map_raw: InstructionMapRaw = serde_json::from_reader(reader)?;

        InstructionMap::from_instruction_map_raw(instruction_map_raw)
    }

    fn from_instruction_map_raw(value: InstructionMapRaw) -> Result<Self> {
        Ok(Self {
            unprefixed: convert_keys_to_u8(value.unprefixed)?,
            cbprefixed: convert_keys_to_u8(value.cbprefixed)?,
        })
    }
}

fn convert_keys_to_u8(map: HashMap<String, Instruction>) -> Result<HashMap<u8, Instruction>> {
    map.into_iter()
        .map(|(key, elem)| Ok((u8::from_str_radix(&key[2..], 16)?, elem)))
        .collect()
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.mnemonic)?;
        for operand in &self.operands {
            write!(f, "\t{}", operand)?;
        }
        Ok(())
    }
}

impl Display for Operands {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.name)
    }
}
