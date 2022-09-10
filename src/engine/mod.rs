
use std::usize;

use super::interface::defaults;
pub mod species;
use species::Species;
use rand::Rng;


use sdl2::{
    pixels::Color,
    sys::{rand, random},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    species: Species,
    ra: u8,
    rb: u8,
    clock: u8,
}

impl Cell {
    pub fn new(species: Species) -> Cell {
        Cell {
            species: species,
            ra: 0,
            rb: 0,
            clock: 0,
        }
    }

    pub fn get_species(&self) -> Species {
        self.species
    }

    pub fn update(&self, api: Api) {
        self.species.update(*self, api);
    }
}

static EMPTY_CELL : Cell = Cell{
    species: Species::Empty,
    ra: 0,
    rb: 0,
    clock: 0,

};
pub struct Engine {
    pub world: World,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Wind {
    dx: u8,
    dy: u8,
    pressure: u8,
    density: u8,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            world: World::new(defaults::WIDTH, defaults::HEIGHT),
        }
    }
}



pub struct World {
    width: usize,
    height: usize,
    pub cells: Vec<Cell>,
    winds: Vec<Wind>,
    generation: u8,

}

pub struct Api<'a> {
    x: usize,
    y: usize,
    world: &'a mut World,
}

impl<'a> Api<'a> {
    pub fn swap(&mut self, x: usize, y: usize) {
        let index = self.world.get_index(x, y);
        let index2 = self.world.get_index(self.x, self.y);
        self.world.cells.swap(index, index2);
    }
    pub fn get(&mut self, dx: i32, dy: i32) -> Cell {

        let nx = self.x + dx as usize;
        let ny = self.y + dy as usize;
        if nx <= 0 || nx > self.world.width - 1 || ny <= 0 || ny > self.world.height - 1 {
            return Cell {
                species: Species::Wall,
                ra: 0,
                rb: 0,
                clock: self.world.generation,
            };
        }
        self.world.get_cell(nx, ny)
        
    }

    pub fn set(&mut self, dx: i32, dy: i32, v: Cell) {
        if dx > 2 || dx < -2 || dy > 2 || dy < -2 {
            panic!("oob set");
        }
        let nx = self.x + dx as usize;
        let ny = self.y + dy as usize;

        if nx < 0 || nx > self.world.width - 1 || ny < 0 || ny > self.world.height - 1 {
            return;
        }
        let i = self.world.get_index(nx, ny);
        // v.clock += 1;
        self.world.cells[i] = v;
        self.world.cells[i].clock = self.world.generation.wrapping_add(1);
    }

    pub fn rand_int(n: i32) -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..n)
    }

    pub fn rand_dir_2(&mut self) -> i32 {
        let i = rand::thread_rng().gen_range(0..100);
        if (i % 2) == 0 {
            -1
        } else {
            1
        }
    }
}

impl World {

    fn get_wind(&self, x: usize, y: usize) -> Wind {
        let i = self.get_index(x, y);
        return self.winds[i];
    }

        fn update_cell(cell: Cell, api: Api) {
            if cell.clock == api.world.generation {
                return;
            }
            cell.update(api);
        }
    }
        
    

impl World {
    pub fn new(width: usize, height: usize) -> World {
        World {
            width,
            height,
            cells: vec![Cell::new(Species::Empty); defaults::WIDTH * defaults::HEIGHT],
            winds: vec![Wind{dx: 0, dy: 0, pressure: 0, density: 0}; defaults::WIDTH * defaults::HEIGHT],
            generation: 0,

        }
    }
    fn get_index(&self, x: usize, y: usize) -> usize {
        x + y * self.width as usize
    }

    fn get_cell(&self, x: usize, y: usize) -> Cell {
        self.cells[self.get_index(x, y)]
    }

    pub fn get(&self, x: usize, y: usize) -> Cell {
        if x >= self.width as usize || y >= self.height as usize {
            return EMPTY_CELL;
        }
        self.cells[x + y * self.width as usize]
    }

    pub fn coord(&self, x: usize, y: usize) -> usize {
        x + y * self.width as usize
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        self.cells[x + y * self.width as usize] = cell;
    }

    pub fn is_cell_free(&self, x: usize, y: usize) -> bool {
        if x < 0 || x >= self.width as usize || y < 0 || y >= self.height as usize {
            return false;
        }
        self.get(x, y) == EMPTY_CELL
    }

    pub fn clear(&mut self) {
        self.cells = vec![EMPTY_CELL; self.width as usize * self.height as usize];
    }

    pub fn is_empty(&self, index: usize) -> bool {
        self.cells[index] == EMPTY_CELL
    }

    pub fn tick(&mut self) {
        // let mut next = self.cells.clone();
        // let dx = self.winds[(self.width * self.height / 2) as usize].dx;
        // let js: JsValue = (dx).into();
        // console::log_2(&"dx: ".into(), &js);

        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.get_cell(x, y);

            }
        }
        self.generation = self.generation.wrapping_add(1);
        for x in 0..self.width {
            let scanx = if self.generation % 2 == 0 {
                self.width - (1 + x)
            } else {
                x
            };

            for y in 0..self.height {
                let idx = self.get_index(scanx, y);
                let cell = self.get_cell(scanx, y);

                World::update_cell(
                    cell,
                    Api {
                        world: self,
                        x: scanx,
                        y,
                    },
                );
            }
            //std::thread::sleep(std::time::Duration::from_millis(1/600));
        }

        self.generation = self.generation.wrapping_add(1);
    }
}

