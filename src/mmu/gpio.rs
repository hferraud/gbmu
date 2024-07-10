use std::ops::{Index, IndexMut};

const GPIO_SIZE: usize = 0x80;

#[derive(Debug)]
pub struct GPIO {
    data: Vec<u8>,
}

impl GPIO {
    pub fn new() -> Self {
        Self {
            data: vec![0; GPIO_SIZE],
        }
    }
}

impl Index<usize> for GPIO {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for GPIO {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
