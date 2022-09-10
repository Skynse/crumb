pub mod element;
use super::interface::defaults;

use rand::Rng;

use element::ElementType;
use sdl2::{
    pixels::Color,
    sys::{rand, random},
};

use self::element::Element;

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
    pub air_pressure: f32,
    pub air_density: f32,
    pub air_temperature: f32,
    pub air_direction: f32,
}
impl Buffer {
    const WIDTH: usize = defaults::WIDTH;
    const HEIGHT: usize = defaults::HEIGHT;

    const SIZE: usize = Self::WIDTH * Self::HEIGHT;

    fn new() -> Self {
        Self {
            elements: vec![ElementType::Empty; Self::SIZE],
            air_pressure: 0.0,
            air_density: 0.0,
            air_temperature: 0.0,
            air_direction: -1.0,
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
        self.elements[x + y * Self::WIDTH] = element;
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

    pub fn is_empty(&self, index: usize) -> bool {
        self.elements[index] == ElementType::Empty
    }
    pub fn draw(&mut self) {
        // draw the walls
        for y in 0..Self::HEIGHT {
            self.set(0, y / 3, ElementType::Wall);
            self.set((Self::WIDTH - 1) / 3, y / 3, ElementType::Wall);
        }

        // draw the ceiling
        for x in 0..Self::WIDTH {
            self.set(x / 3, 0, ElementType::Wall);
        }

        // draw the floor
        for x in 0..Self::WIDTH {
            self.set(x / 3, (Self::HEIGHT - 1) / 3, ElementType::Wall);
        }

        let dir: i32 = if rand::thread_rng().gen() { 1 } else { -1 };
        for y in (0..Self::HEIGHT - 1).rev() {
            for x in 0..Self::WIDTH {
                let element = self.get(x, y);
                if element == ElementType::Empty || element == ElementType::Wall {
                    continue;
                }

                if self.get(x, y) != ElementType::Wall || self.get(x, y) != ElementType::Empty {
                    if self.get(x, y + 1) == ElementType::Empty {
                        self.swap(x, y, x, y + 1);
                    } else if self.get((x as i32 + dir as i32) as usize, y) == ElementType::Empty {
                        self.swap(x, y, (x as i32 + dir as i32) as usize, y + 1);
                    }
                }

            }
        }
    }
}
