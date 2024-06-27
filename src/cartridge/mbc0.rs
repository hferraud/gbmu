use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct MBC0 {
    rom: Vec<u8>,
}

impl MBC0 {
    pub fn new(data: Vec<u8>) -> Self {
        MBC0 { rom: data }
    }

    pub fn get_address(&mut self, address: usize) -> &mut u8 {
        &mut self.rom[address]
    }
}

impl Index<usize> for MBC0 {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rom[index]
    }
}

impl IndexMut<usize> for MBC0 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rom[index]
    }
}
