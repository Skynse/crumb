use std::usize;

use crate::interface::defaults::{UI_X, UI_Y};

use super::interface::defaults;
pub mod species;
use rand::{Rng, SeedableRng};
use species::Species;

use rand_xoshiro::SplitMix64;

#[derive(Clone, Copy, Debug, Default)]
pub struct Cell {
    pub species: Species,
    pub ra: u8,
    pub rb: u8,
    pub(crate) clock: u8,
}

// RA: []

impl Cell {
    pub fn new(species: Species) -> Cell {
        let rb = if species == Species::GOL { 1 } else { 0 };
        Cell {
            species: species,
            ra: 100 + rand::thread_rng().gen_range(0..2) * 50 as u8,
            rb: rb,
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

static EMPTY_CELL: Cell = Cell {
    species: Species::EMPT,
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
            world: World::new(
                defaults::WIDTH as i32 - UI_X as i32,
                defaults::HEIGHT as i32 - UI_Y as i32,
            ),
        }
    }
}

#[allow(dead_code)]
pub struct World {
    width: i32,
    height: i32,
    pub cells: Vec<Cell>,
    winds: Vec<Wind>,
    pub generation: u8,
    burns: Vec<Wind>,
    pub rng: SplitMix64,
}

pub struct Api<'a> {
    x: i32,
    y: i32,
    world: &'a mut World,
}

impl<'a> Api<'a> {
    pub fn rand_int(&mut self, n: i32) -> i32 {
        self.world.rng.gen_range(0..n)
    }

    pub fn rand_dir(&mut self) -> i32 {
        let i = self.rand_int(1000);
        (i % 3) - 1
    }

    pub fn once_in(&mut self, n: i32) -> bool {
        self.rand_int(n) == 0
    }

    pub fn get_fluid(&mut self) -> Wind {
        let idx = self.world.get_index(self.x, self.y);
        self.world.winds[idx]
    }
    pub fn set_fluid(&mut self, v: Wind) {
        let idx = self.world.get_index(self.x, self.y);
        self.world.burns[idx] = v;
    }

    pub fn rand_vec(&mut self) -> (i32, i32) {
        let i = self.rand_int(2000);
        match i % 9 {
            0 => (1, 1),
            1 => (1, 0),
            2 => (1, -1),
            3 => (0, -1),
            4 => (-1, -1),
            5 => (-1, 0),
            6 => (-1, 1),
            7 => (0, 1),
            _ => (0, 0),
        }
    }

    pub fn rand_vec_8(&mut self) -> (i32, i32) {
        let i = self.rand_int(8);
        match i {
            0 => (1, 1),
            1 => (1, 0),
            2 => (1, -1),
            3 => (0, -1),
            4 => (-1, -1),
            5 => (-1, 0),
            6 => (-1, 1),
            _ => (0, 1),
        }
    }

    pub fn get(&mut self, dx: i32, dy: i32) -> Cell {
        if dx > 2 || dx < -2 || dy > 2 || dy < -2 {
            panic!("oob set");
        }
        let nx = self.x + dx;
        let ny = self.y + dy;
        if nx < 0 || nx > self.world.width - 1 || ny < 0 || ny > self.world.height - 1 {
            return Cell {
                species: Species::WALL,
                ra: 0,
                rb: 0,
                clock: self.world.generation,
            };
        }
        self.world.get_cell(nx, ny)
    }

    #[allow(unused_comparisons)]
    pub fn set(&mut self, dx: i32, dy: i32, v: Cell) {
        if dx > 2 || dx < -2 || dy > 2 || dy < -2 {
            panic!("oob set");
        }
        let nx = self.x + dx;
        let ny = self.y + dy;

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
    fn blow_wind(cell: Cell, wind: Wind, mut api: Api) {
        if cell.clock.wrapping_sub(api.world.generation) == 1 {
            return;
        }
        if cell.species == Species::EMPT {
            return;
        }
        let mut dx = 0;
        let mut dy = 0;

        let threshold = match cell.species {
            Species::EMPT => 500,
            Species::WALL => 500,
            Species::GOL => 500,

            Species::OIL => 50,
            Species::WATR => 50,

            Species::SAND => 30,
            Species::DUST => 10,
            Species::FIRE => 5,
            Species::GAS => 5,

            _ => 40,
        };

        let wx = (wind.dy as i32) - 126;
        let wy = (wind.dx as i32) - 126;

        if wx > threshold {
            dx = 1;
        }
        if wy > threshold {
            dy = 1;
        }
        if wx < -threshold {
            dx = -1;
        }
        if wy < -threshold {
            dy = -1;
        }
        if (dx != 0 || dy != 0) && api.get(dx, dy).species == Species::EMPT {
            api.set(0, 0, EMPTY_CELL);
            if dy == -1
                && api.get(dx, -2).species == Species::EMPT
                && (cell.species == Species::SAND
                    || cell.species == Species::WATR
                    || cell.species == Species::DUST
                    || cell.species == Species::OIL
                    || cell.species == Species::GOL)
            {
                dy = -2;
            }
            api.set(dx, dy, cell);
            return;
        }
    }
    fn update_cell(cell: Cell, api: Api) {
        if cell.clock.saturating_sub(api.world.generation) == 1 {
            return;
        }
        cell.update(api);
    }
}

// private methods
impl World {
    pub fn new(width: i32, height: i32) -> World {
        let rng: SplitMix64 = SeedableRng::seed_from_u64(0x734f6b89de5f83cc);
        World {
            width: (width - UI_X as i32) / 2 as i32,
            height: (height - UI_Y as i32) as i32,
            cells: vec![Cell::new(Species::EMPT); defaults::WIDTH * defaults::HEIGHT],
            winds: vec![
                Wind {
                    dx: 0,
                    dy: 0,
                    pressure: 100,
                    density: 0
                };
                defaults::WIDTH * defaults::HEIGHT
            ],
            generation: 0,
            burns: vec![
                Wind {
                    dx: 0,
                    dy: 0,
                    pressure: 0,
                    density: 0
                };
                defaults::WIDTH * defaults::HEIGHT
            ],
            rng,
        }
    }
    pub fn get_index(&self, x: i32, y: i32) -> usize {
        (x + y * self.width) as usize
    }

    fn get_cell(&self, x: i32, y: i32) -> Cell {
        let i = self.get_index(x, y);
        return self.cells[i];
    }

    pub fn get(&self, x: usize, y: usize) -> Cell {
        if x >= self.width as usize || y >= self.height as usize {
            return EMPTY_CELL;
        }
        self.cells[x + y * self.width as usize]
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        self.cells[x + y * self.width as usize] = cell;
    }

    pub fn clear(&mut self) {
        for i in 0..self.cells.len() {
            self.cells[i] = EMPTY_CELL;
        }
    }

    fn get_wind(&self, x: i32, y: i32) -> Wind {
        let i = self.get_index(x, y);
        return self.winds[i];
    }

    pub fn tick(&mut self) {
        // called every SDL frame
        /*
        for y in 0..self.height {
            for x in 0..self.width{
                let cell = self.get_cell(x, y);
                let wind = self.get_wind(x as i32, y as i32);
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
        */
        self.generation = (self.generation + 1) % 255;

        for x in 0..self.width - 1 {
            let scanx = if self.generation % 2 == 0 {
                self.width - (1 + x)
            } else {
                x
            };

            for y in 0..self.height - 1 {
                let _idx = self.get_index(scanx, y);
                let cell = self.get_cell(scanx, y);

                self.burns[_idx] = Wind {
                    dx: 0,
                    dy: 0,
                    pressure: 0,
                    density: 0,
                };

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

        self.generation = (self.generation + 1) % 255;
    }
}
