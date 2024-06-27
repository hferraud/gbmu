const DMG_WRAM_SIZE: usize = 1 << 13; //  8 192
const CBG_WRAM_SIZE: usize = 1 << 15; // 32 768
const BANK_WIDTH: usize = 0x1000;

pub struct WRAM {
    data: Vec<u8>,
    bank: u8,
    cgb_mode: bool,
}

impl WRAM {
    pub fn new(cbg_mode: bool) -> Self {
        let data = if cbg_mode {
            vec![0; CBG_WRAM_SIZE]
        } else {
            vec![0; DMG_WRAM_SIZE]
        };

        WRAM {
            data,
            bank: 1,
            cgb_mode: cbg_mode,
        }
    }

    pub fn get_address(&mut self, address: usize) -> &mut u8 {
        if (address) < BANK_WIDTH {
            &mut self.data[address]
        } else {
            &mut self.data[address + self.bank as usize * BANK_WIDTH]
        }
    }
}
