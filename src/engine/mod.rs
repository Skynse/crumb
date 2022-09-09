pub mod element;

use element::ElementType;

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
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;
    const SIZE: usize = Self::WIDTH * Self::HEIGHT;

    fn new() -> Self {
       Self {
           elements: vec![ElementType::Empty; Self::SIZE],

       }
    }

    pub fn get(&self, x: usize, y: usize) -> ElementType {
        self.elements[x + y * Self::WIDTH]
    }

    pub fn coord(&self, x: usize, y: usize) -> usize {
        x + y * Self::WIDTH
    }

    pub fn set(&mut self, x: usize, y: usize, element: ElementType) {
        self.elements[x + y * Self::HEIGHT] = element;
    }

    pub fn is_cell_free(&self, x: usize, y: usize) -> bool {
        self.get(x, y) == ElementType::Empty
    }

    pub fn draw(&mut self) {

        //draw wall on bottom
        for x in 0..Self::WIDTH {
            self.set(x, Self::HEIGHT - 1, ElementType::Wall);
        }

        for x in 0..Self::WIDTH {
            for y in 0..Self::HEIGHT {
                
                let element = self.get(x, y);
                if element == ElementType::Empty || element == ElementType::Wall {
                    continue;
                }

                // check if bottom is empty, then move down
                if  self.is_cell_free(x, y + 1) {
                    self.set(x, y, ElementType::Empty);
                    self.set(x, y + 1, element);
                }
                
                else if self.is_cell_free(x-1, y+1) {
                    self.set(x, y, ElementType::Empty);
                    self.set(x-1, y+1, element);
                }

                else if self.is_cell_free(x+1, y+1) {
                    self.set(x, y, ElementType::Empty);
                    self.set(x+1, y+1, element);
                }
            }
        }
    }
    
}