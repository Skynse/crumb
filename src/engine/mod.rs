pub mod element;
use rand::Rng;

use element::ElementType;
use sdl2::{sys::{rand, random}, pixels::Color};

pub struct Engine {
   pub buffer: Buffer,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            buffer: Buffer::new(),
        }
    }

}

#[derive(Default)]
pub struct Buffer {
    pub elements: Vec<ElementType>,
}

impl Buffer {
    const cellSize: usize = 5;
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;
    
    const SIZE: usize = Self::WIDTH * Self::HEIGHT * 3;
    
    fn new() -> Self {
       Self {
           elements: vec![ElementType::Empty; Self::SIZE],
       }
    }

    pub fn get(&self, x: usize, y: usize) -> ElementType {
        if x >= Self::WIDTH || y >= Self::HEIGHT {
            return ElementType::Empty;
        }
        self.elements[x + y * Self::WIDTH]
    }

    pub fn coord(&self, x: usize, y: usize) -> usize {
        x + y * Self::WIDTH
    }

    pub fn set(&mut self, x: usize, y: usize, element: ElementType) {
        self.elements[x + y * Self::HEIGHT] = element;
    }

    pub fn is_cell_free(&self, x: usize, y: usize) -> bool {
        if x < 0 || x >= Self::WIDTH || y < 0 || y >= Self::HEIGHT {
            return false;
        }
        self.get(x, y) == ElementType::Empty
    }

    pub fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let temp = self.get(x1, y1);
        self.set(x1, y1, self.get(x2, y2));
        self.set(x2, y2, temp);
    }

    pub fn clear(&mut self) {
        self.elements = vec![ElementType::Empty; Self::SIZE];
    }
    pub fn draw(&mut self) {
        // draw the ground
        for y in Self::HEIGHT-10..Self::HEIGHT {
            for x in 0..Self::WIDTH {
                self.set(x, y, ElementType::Wall);
            }
        }

        let check_left_first: bool = rand::thread_rng().gen();

        for y in (0..Self::HEIGHT-1).rev() {
            for x in 0..Self::WIDTH {
                
                let element = self.get(x, y);
                if element == ElementType::Empty || element == ElementType::Wall {
                    continue;
                }
                
                // check if bottom is empty, then move down
                if  self.is_cell_free(x, y + 1) {
                    self.set(x, y, ElementType::Empty);
                    self.set(x, y + 1, element);
                    continue;
                }
              
                if check_left_first {
                      // if bottom is not free, check if bottom left corner is empty
                    if self.is_cell_free(x-1, y+1) {
                        self.set(x, y, ElementType::Empty);
                        self.set(x-1, y+1, element);
                        continue;
                    }
                    // if bottom left corner is not empty, check bottom right corner
                    else if self.is_cell_free(x+1, y+1) {
                        self.set(x, y, ElementType::Empty);
                        self.set(x+1, y+1, element);
                        continue;
                    }
                }
                else {
                      // if bottom left corner is not empty, check bottom right corner
                    if self.is_cell_free(x+1, y+1) {
                        self.set(x, y, ElementType::Empty);
                        self.set(x+1, y+1, element);
                        continue;
                    }
                    // if bottom is not free, check if bottom left corner is empty
                    else if self.is_cell_free(x-1, y+1) {
                        self.set(x, y, ElementType::Empty);
                        self.set(x-1, y+1, element);
                        continue;
                    }
                }
            

                    continue;
                }
            }
        }
    }