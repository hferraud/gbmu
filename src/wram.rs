use std::ops::{Index, IndexMut};

const DMG_WRAM_SIZE: usize = 1 << 13; //  8 192
const CBG_WRAM_SIZE: usize = 1 << 15; // 32 768
const BANK_WIDTH: usize = 0x1000;

pub struct WRAM {
    data: Vec<u8>,
    bank: u8,
}

impl WRAM {
    pub fn new(cbg_mode: bool) -> Self {
        let data = if cbg_mode {
            vec![0; CBG_WRAM_SIZE]
        } else {
            vec![0; DMG_WRAM_SIZE]
        };

        WRAM { data, bank: 1 }
    }
}

impl Index<usize> for WRAM {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        if (index) < BANK_WIDTH {
            &self.data[index]
        } else {
            &self.data[index + (self.bank - 1) as usize * BANK_WIDTH]
        }
    }
}

impl IndexMut<usize> for WRAM {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if (index) < BANK_WIDTH {
            &mut self.data[index]
        } else {
            &mut self.data[index + (self.bank - 1) as usize * BANK_WIDTH]
        }
    }
}
