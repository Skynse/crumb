
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
            world: World::new(),
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
    pub fn get(&mut self, dx: i32, dy: i32) -> Cell {

        let nx = self.x + dx as usize;
        let ny = self.y + dy as usize;
        if nx < 0 || nx > self.world.width - 1 || ny < 0 || ny > self.world.height - 1 {
            return Cell {
                species: Species::Wall,
                ra: 0,
                rb: 0,
                clock: self.world.generation,
            };
            println!("{} DSMMMMMMMMMMMMMM {}", nx, ny);
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
    
    fn blow_wind(cell: Cell, wind: Wind, mut api: Api) {
        if cell.clock - api.world.generation == 1 {
            return;
        }
        let mut dx = 0;
        let mut dy = 0;

        let thresh = 40;

        let wx = wind.dy as i32 - 126;
        let wy = wind.dx as i32 - 126;
        
        if wy > thresh {
            dx = 1;
        }

        if wy < -thresh {
            dx = -1;
        }

        if wx > thresh {
            dy = 1;
        }

        if wx < -thresh {
            dy = -1;
        }

        if dx != 0 || dy != 0 {
            if api.world.is_cell_free(api.x + dx as usize, api.y + dy as usize) {
                api.world.swap(api.x as i32, api.y as i32, api.x as i32+ dx as i32, api.y as i32 + dy as i32);
            }
        }
    }

        fn update_cell(cell: Cell, api: Api) {
            cell.update(api);
        }
        
    }

impl World {

    fn new() -> Self {
        Self {
            width: defaults::WIDTH,
            height: defaults::HEIGHT,
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

    pub fn swap(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let i1 = self.get_index(x1 as usize, y1 as usize);
        let i2 = self.get_index(x2 as usize, y2 as usize);
        let tmp = self.cells[i1];
        self.cells[i1] = self.cells[i2];
        self.cells[i2] = tmp;
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
                let wind = self.get_wind(x, y);
                World::blow_wind(
                    cell,
                    wind,
                    Api {
                        world: self,
                        x,
                        y,
                    },
                )
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
        }

        self.generation = self.generation.wrapping_add(1);
    }
}

