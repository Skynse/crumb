use rand::Rng;
use sdl2::pixels::Color;


use super::{World, Api, Wind};
use crate::engine::{Cell, EMPTY_CELL};
#[derive( Clone, Copy, PartialEq, Debug, Eq)]


#[derive(Default)]
pub enum Species {
    EMPT = 0,
    WALL = 1,
    #[default]
    DUST = 2,
    SAND = 3,
    WATR = 4,
    GAS = 5,
    OIL = 6,
    FIRE = 7,
    SMKE = 8,
}

impl Species {
    pub fn update(&self, cell: Cell, api: Api) {
        match self {
            Species::EMPT => {},
            Species::WALL => {},
            Species::DUST => update_dust(cell, api),
            Species::SAND => update_sand(cell, api),
            Species::WATR => update_water(cell, api),
            Species::GAS => todo!(),
            Species::OIL => todo!(),
            Species::FIRE => update_fire(cell, api),
            Species::SMKE => update_smoke(cell, api),
        }
    }
}

pub fn update_dust(cell: Cell, mut api: Api) {
    let dx = api.rand_dir();
    let nb = api.get(dx, 1);
    let nbr = api.get(dx + 1, 1);
    let nbl = api.get(dx - 1, 1);

    if nb.species == Species::EMPT {
        api.set(dx, 1, cell);
        api.set(0, 0, EMPTY_CELL);
    } else if nbr.species == Species::EMPT {
        api.set(dx + 1, 1, cell);
        api.set(0, 0, EMPTY_CELL);
    } else if nbl.species == Species::EMPT {
        api.set(dx - 1, 1, cell);
        api.set(0, 0, EMPTY_CELL);
    }
}
pub fn update_sand(cell: Cell, mut api: Api) {
    let dx = api.rand_dir();
    let nb = api.get(dx, 1);
    let nbr = api.get(dx + 1, 1);
    let nbl = api.get(dx - 1, 1);

    if nb.species == Species::EMPT {
        api.set(dx, 1, cell);
        api.set(0, 0, EMPTY_CELL);
    } else if nbr.species == Species::EMPT {
        api.set(dx + 1, 1, cell);
        api.set(0, 0, EMPTY_CELL);
    } else if nbl.species == Species::EMPT {
        api.set(dx - 1, 1, cell);
        api.set(0, 0, EMPTY_CELL);
    }
}

pub fn update_water(cell: Cell, mut api: Api) {
    let dispersal_rate: i32 = 10;
    let dx = api.rand_dir();
    let nb = api.get(dx, 1);
    let nbr = api.get(dx + 1, 1);
    let nbl = api.get(dx - 1, 1);
    let nr = api.get(1, 0);
    let nl = api.get(-1, 0);
    let nt = api.get(0, -1);

    if nt.species == Species::SAND {
        // make water flow up and bring sand down
        api.set(0, -1, cell);
        api.set(0, 0, Cell{species: Species::SAND, ra: 0, rb: 0, clock: 0, ..Default::default()});
    }

    // if the temperature of the water is high enough, it will evaporate into smoke
    if cell.temperature > 100.0 {
        api.set(0, 0, Cell{species: Species::SMKE, ra: 0, rb: 0, clock: 0, ..Default::default()});
    }

    if nb.species == Species::EMPT {
        api.set(dx, 1, cell);
        api.set(0, 0, EMPTY_CELL);
    } else if nbr.species == Species::EMPT {
        api.set(dx + 1, 1, cell);
        api.set(0, 0, EMPTY_CELL);
    } else if nbl.species == Species::EMPT {
        api.set(dx - 1, 1, cell);
        api.set(0, 0, EMPTY_CELL);
    }

    else {
        if nr.species == Species::EMPT {
            api.set(1, 0, cell);
            api.set(0, 0, EMPTY_CELL);
        } else if nl.species == Species::EMPT {
            api.set(-1, 0, cell);
            api.set(0, 0, EMPTY_CELL);
        }
    }

}

pub fn update_fire(cell: Cell, mut api: Api) {
    let ra = cell.ra;
    let mut degraded = cell.clone();
    degraded.ra = ra - (2 + api.rand_dir()) as u8;

    let (dx, dy) = api.rand_vec_8();
    
    // set the temperature of surrounding cells to temperature + 1
    let nb = api.get(dx, dy);
    if nb.species == Species::EMPT {
        api.set(dx, dy, Cell{species: Species::FIRE, ra: 0, rb: 0, clock: 0, temperature: cell.temperature + 1.0, ..Default::default()});
    }

    // if the temperature of the fire is high enough, it will burn the surrounding cells

    api.set_fluid(Wind {
        dx: 0,
        dy: 150,
        pressure: 1,
        density: 120,
    });
    if api.get(dx, dy).species == Species::GAS || api.get(dx, dy).species == Species::DUST {
        api.set(
            dx,
            dy,
            Cell {
                species: Species::FIRE,
                ra: (150 + (dx + dy) * 10) as u8,
                rb: 0,
                clock: 0,
                ..Default::default()
            },
        );
        api.set_fluid(Wind {
            dx: 0,
            dy: 0,
            pressure: 80,
            density: 40,
        });
    }
    if ra < 5 || api.get(dx, dy).species == Species::WATR {
        api.set(0, 0, Cell{species: Species::SMKE, ra: 0, rb: 0, clock: 0, ..Default::default()});
    } else if api.get(dx, dy).species == Species::EMPT {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, dy, degraded);
    } else {
        api.set(0, 0, degraded);
    }
}

pub fn update_smoke(cell: Cell, mut api: Api) {
    let dx = api.rand_dir();
    let nu = api.get(dx, -1);
    
    if nu.species == Species::EMPT {
        api.set(dx, -1, cell);
        api.set(0, 0, EMPTY_CELL);
    }

    // die after 100 ticks
    if cell.clock > 100 {
        api.set(0, 0, EMPTY_CELL);
    }

}