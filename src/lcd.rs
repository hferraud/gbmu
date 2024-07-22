use std::ops::{Index, IndexMut};

const GAMEBOY_SCREEN_WIDTH: usize = 160;
const GAMEBOY_SCREEN_HEIGHT: usize = 144;

type Pixel = (u8, u8, u8);

pub struct LCD {
    pub image: [[Pixel; GAMEBOY_SCREEN_HEIGHT]; GAMEBOY_SCREEN_WIDTH],
}

impl Index<(usize, usize)> for LCD {
    type Output = Pixel;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.image[x][y]
    }
}

impl IndexMut<(usize, usize)> for LCD {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Pixel {
        &mut self.image[x][y]
    }
}

impl LCD {
    pub fn new() -> Self {
        Self {
            image: [[(0, 0, 0); GAMEBOY_SCREEN_HEIGHT]; GAMEBOY_SCREEN_WIDTH]
        }
    }

    pub fn iter(&self) -> std::slice::Iter<[Pixel; GAMEBOY_SCREEN_HEIGHT]> {
        self.image.iter()
    }
}
