use std::ops::{Index, IndexMut};

const HRAM_SIZE: usize = 0x7F;

#[derive(Debug)]
pub struct HRAM {
    data: Vec<u8>,
}

impl HRAM {
    pub fn new() -> Self {
        HRAM {
            data: vec![0; HRAM_SIZE]
        }
    }
}

impl Index<usize> for HRAM {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for HRAM {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
