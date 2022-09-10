use rand::Rng;
use sdl2::pixels::Color;


use super::{World, Api};
use crate::engine::Cell;
#[derive( Clone, Copy, PartialEq, Debug, Eq)]
pub enum Species {
    Empty = 0,
    Wall = 1,
    Dust = 2,
}

impl Species {
    pub fn update(&self, cell: Cell, api: Api) {
        match self {
            Species::Empty => {},
            Species::Wall => {},
            Species::Dust => update_dust(cell, api),
        }
    }
}

pub fn update_dust(cell: Cell, mut api: Api) {
    let dir: i32 = if rand::thread_rng().gen() { 1 } else { -1 };
    for y in (0..api.world.height - 1).rev() {
        for x in 0..api.world.width{
            let x = x as i32;
            let y = y as i32;
            let cell = api.get(x, y);
            if cell.get_species() == Species::Empty {
                continue;
            }

            if api.get(x, y).get_species() != Species::Empty {
                if api.get(x, y + 1).get_species() == Species::Empty {
                    // check if we're at the bottom of the screen
                    if y + 1 < api.world.height as i32/5 {
                        api.world.swap(x, y, x, y + 1);
                    }
                } else if api.get((x as i32 + dir as i32), y).get_species() == Species::Empty {
                    api.world.swap(x, y, (x as i32 + dir as i32) , y + 1);
                }
            }

        }
    }
    
}
