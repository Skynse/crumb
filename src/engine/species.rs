use rand::Rng;
use sdl2::pixels::Color;


use super::{World, Api};
use crate::engine::{Cell, EMPTY_CELL};
#[derive( Clone, Copy, PartialEq, Debug, Eq)]

pub enum Species {
    Empty = 0,
    Wall = 1,
    Dust = 2,
    Sand = 3,
    Water = 4,
    Gas = 5,
    Oil = 6,
}

impl Species {
    pub fn update(&self, cell: Cell, api: Api) {
        match self {
            Species::Empty => {},
            Species::Wall => {},
            Species::Dust => update_dust(cell, api),
            Species::Sand => todo!(),
            Species::Water => todo!(),
            Species::Gas => todo!(),
            Species::Oil => todo!(),
        }
    }
}

fn rand_dir() -> i32 {
    // random value in {-1, 0, 1}
    let mut rng = rand::thread_rng();
    rng.gen_range(-1..=1)
}

pub fn update_dust(cell: Cell, mut api: Api) {
    let dx = rand_dir();

    /** */
    let nbr = api.get(1, 0);
    if nbr.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else if nbr.species == Species::Water
        || nbr.species == Species::Gas
        || nbr.species == Species::Oil
    {
        api.set(0, 0, nbr);
        api.set(0, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}
