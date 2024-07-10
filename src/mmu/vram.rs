use std::ops::{Index, IndexMut};

const DMG_VRAM_SIZE: usize = 1 << 13; //  8 192
const CBG_VRAM_SIZE: usize = 1 << 14; // 16 384
const BANK_WIDTH: usize = 1 << 13;

pub struct VRAM {
    data: Vec<u8>,
    bank: u8,
}

impl VRAM {
    pub fn new(cbg_mode: bool) -> Self {
        let data = if cbg_mode {
            vec![0; CBG_VRAM_SIZE]
        } else {
            vec![0; DMG_VRAM_SIZE]
        };
        VRAM { data, bank: 0 }
    }
}

impl Index<usize> for VRAM {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index + self.bank as usize * BANK_WIDTH]
    }
}

impl IndexMut<usize> for VRAM {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index + self.bank as usize * BANK_WIDTH]
    }
}
