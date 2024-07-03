use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct MBC0 {
    rom: Vec<u8>,
}

impl MBC0 {
    pub fn new(data: Vec<u8>) -> Self {
        MBC0 { rom: data }
    }
}

impl Index<usize> for MBC0 {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        // TODO return Result<&Self::Output> to avoid panicking if there's
        //      an error in the ROM
        &self.rom[index]
    }
}

impl IndexMut<usize> for MBC0 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // TODO return Result<&mut Self::Output> to avoid panicking if there's
        //      an error in the ROM
        &mut self.rom[index]
    }
}
