use std::ops::{Index, IndexMut};

const OAM_SIZE: usize = 0xa0; //  160
const BANK_WIDTH: usize = 1 << 13;

pub struct OAM {
    data: Box<[u8]>,
    bank: u8,
}

impl OAM {
    pub fn new() -> Self {
        OAM {
            data: vec![0; OAM_SIZE].into_boxed_slice(),
            bank: 0,
        }
    }
}

impl Index<usize> for OAM {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index + self.bank as usize * BANK_WIDTH]
    }
}

impl IndexMut<usize> for OAM {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index + self.bank as usize * BANK_WIDTH]
    }
}
