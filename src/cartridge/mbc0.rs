#[derive(Debug)]

pub struct MBC0 {
    rom: Vec<u8>,
}

impl MBC0 {
    pub fn new(data: Vec<u8>) -> Self {
        MBC0 { rom: data }
    }
}
