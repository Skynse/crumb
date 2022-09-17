use super::{Api, Wind};
use crate::engine::{Cell, EMPTY_CELL};

#[derive(Clone, Copy, PartialEq, Debug, Eq)]
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
    GOL = 9,
    WOOD = 10,
    OXGN = 11,
    HYGN,
}

pub const SPECIES_COUNT: usize = 13;
// create an impl for species which returns an array of species in the order they are defined
impl Species {
    pub fn all() -> [Species; SPECIES_COUNT] {
        [
            Species::EMPT,
            Species::WALL,
            Species::DUST,
            Species::SAND,
            Species::WATR,
            Species::GAS,
            Species::OIL,
            Species::FIRE,
            Species::SMKE,
            Species::GOL,
            Species::WOOD,
            Species::OXGN,
            Species::HYGN,
        ]
    }
}
impl Species {
    pub fn update(&self, cell: Cell, api: Api) {
        match self {
            Species::EMPT => {}
            Species::WALL => {}
            Species::DUST => update_dust(cell, api),
            Species::SAND => update_sand(cell, api),
            Species::WATR => update_water(cell, api),
            Species::GAS => {},
            Species::OIL => update_oil(cell, api),
            Species::FIRE => update_fire(cell, api),
            Species::SMKE => update_smoke(cell, api),
            Species::GOL => update_gol(cell, api),
            Species::WOOD => update_wood(cell, api),
            Species::OXGN => update_oxygen(cell, api),
            Species::HYGN => update_hydrogen(cell, api),
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
    let dx = api.rand_dir_2();

    let nbr = api.get(0, 1);
    if nbr.species == Species::EMPT {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::EMPT {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else if nbr.species == Species::WATR
        || nbr.species == Species::GAS
        || nbr.species == Species::OIL
    {
        api.set(0, 0, nbr);
        api.set(0, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

pub fn update_water(cell: Cell, mut api: Api) {
    let mut dx = api.rand_dir();
    let below = api.get(0, 1);
    let dx1 = api.get(dx, 1);
    // let mut dx0 = api.get(dx, 0);
    //fall down
    if below.species == Species::EMPT || below.species == Species::OIL {
        api.set(0, 0, below);
        let mut ra = cell.ra;
        if api.once_in(20) {
            //randomize direction when falling sometimes
            ra = 100 + api.rand_int(50) as u8;
        }
        api.set(0, 1, Cell { ra, ..cell });

        return;
    } else if dx1.species == Species::EMPT || dx1.species == Species::OIL {
        //fall diagonally
        api.set(0, 0, dx1);
        api.set(dx, 1, cell);
        return;
    } else if api.get(-dx, 1).species == Species::EMPT {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 1, cell);
        return;
    }
    let left = cell.ra % 2 == 0;
    dx = if left { 1 } else { -1 };
    let dx0 = api.get(dx, 0);
    let dxd = api.get(dx * 2, 0);

    if dx0.species == Species::EMPT && dxd.species == Species::EMPT {
        // scoot double
        api.set(0, 0, dxd);
        api.set(2 * dx, 0, Cell { rb: 6, ..cell });
        let (dx, dy) = api.rand_vec_8();
        let nbr = api.get(dx, dy);

        // spread opinion
        if nbr.species == Species::WATR {
            if nbr.ra % 2 != cell.ra % 2 {
                api.set(
                    dx,
                    dy,
                    Cell {
                        ra: cell.ra,
                        ..cell
                    },
                )
            }
        }
    } else if dx0.species == Species::EMPT || dx0.species == Species::OIL {
        api.set(0, 0, dx0);
        api.set(dx, 0, Cell { rb: 3, ..cell });
        let (dx, dy) = api.rand_vec_8();
        let nbr = api.get(dx, dy);
        if nbr.species == Species::WATR {
            if nbr.ra % 2 != cell.ra % 2 {
                api.set(
                    dx,
                    dy,
                    Cell {
                        ra: cell.ra,
                        ..cell
                    },
                )
            }
        }
    } else if cell.rb == 0 {
        if api.get(-dx, 0).species == Species::EMPT {
            // bump
            api.set(
                0,
                0,
                Cell {
                    ra: ((cell.ra as i32) + dx) as u8,
                    ..cell
                },
            );
        }
    } else {
        // become less certain (more bumpable)
        api.set(
            0,
            0,
            Cell {
                rb: cell.rb - 1,
                ..cell
            },
        );
    }
    // if api.once_in(8) {
    //     let (dx, dy) = api.rand_vec_8();
    //     let nbr = api.get(dx, dy);
    //     if nbr.species == Species::Water {
    //         if nbr.ra % 2 != cell.ra % 2 {
    //             api.set(0, 0, Cell { ra: nbr.ra, ..cell })
    //         }
    //     }
    // }

    // let (dx, dy) = api.rand_vec_8();
    // let nbr = api.get(dx, dy);
    // if nbr.species == Species::Water {
    //     if nbr.ra % 2 != cell.ra % 2 && api.once_in(2) {
    //         api.set(0, 0, Cell { ra: nbr.ra, ..cell })
    //     }
    // }

    // {

    // if api.get(-dx, 0).species == Species::EMPT {
    //     api.set(0, 0, EMPTY_CELL);
    //     api.set(-dx, 0, cell);
    // }
    }

pub fn update_fire(cell: Cell, mut api: Api) {
    let ra = cell.ra;
    let mut degraded = cell.clone();
    
    degraded.ra = ra.wrapping_sub((2_u8.wrapping_add(api.rand_dir() as u8))) as u8;

    let (dx, dy) = api.rand_vec();

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
        api.set(0, 0, EMPTY_CELL);
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

pub fn update_gol(cell: Cell, mut api: Api) {
    let gol_dead: Cell = Cell {
        species: Species::GOL,
        clock: 0,
        ra: 0,
        rb: 0,
    };

    let gol_alive: Cell = Cell {
        species: Species::GOL,
        clock: 0,
        ra: 0,
        rb: 1,
    };
    // get neighbors in all directions
    let nb = api.get(0, 1);
    let nt = api.get(0, -1);

    let nr = api.get(1, 0);
    let nl = api.get(-1, 0);
    
    let ntr = api.get(1, -1);
    let nbr = api.get(1, 1);
    
    
    let nbl = api.get(-1, 1);
    let ntl = api.get(-1, -1);
    
    let mut neighbors = 0;
    // check if neighbors are alive and neighbor is Species::GOL
    if nb.species == Species::GOL && nb.rb == 1 {
        neighbors += 1;
    }
    if nt.species == Species::GOL && nt.rb == 1 {
        neighbors += 1;
    }
    if nr.species == Species::GOL && nr.rb == 1 {
        neighbors += 1;
    }
    if nl.species == Species::GOL && nl.rb == 1 {
        neighbors += 1;
    }
    if ntr.species == Species::GOL && ntr.rb == 1 {
        neighbors += 1;
    }
    if nbr.species == Species::GOL && nbr.rb == 1 {
        neighbors += 1;
    }
    if nbl.species == Species::GOL && nbl.rb == 1 {
        neighbors += 1;
    }
    if ntl.species == Species::GOL && ntl.rb == 1 {
        neighbors += 1;
    }
    // rb denotes if the cell is alive or dead
    // 1 for alive, 0 for dead
    if cell.rb == 1 {
        if neighbors < 2 {
            api.set(0, 0, gol_dead);
        } else if neighbors > 3 {
            api.set(0, 0, gol_dead);
        } else {
            api.set(0, 0, gol_alive);
        }
    } else {
        if neighbors == 3 {
            api.set(0, 0, gol_alive);
        } else {
            api.set(0, 0, gol_dead);
        }
    }
    

}

pub fn update_wood(cell: Cell, mut api: Api) {
    // draw block of wood
    api.set(0, 0, cell);
}

fn update_oil(cell: Cell, mut api: Api) {
    let rb = cell.rb;
    let (dx, dy) = api.rand_vec();

    let mut new_cell = cell;
    let nbr = api.get(dx, dy);
    if rb == 0 && nbr.species == Species::FIRE
        || (nbr.species == Species::OIL && nbr.rb > 1 && nbr.rb < 20)
    {
        new_cell = Cell {
            species: Species::OIL,
            ra: cell.ra,
            rb: 50,
            clock: 0,
        };
    }

    if rb > 1 {
        new_cell = Cell {
            species: Species::OIL,
            ra: cell.ra,
            rb: rb - 1,
            clock: 0,
        };
        api.set_fluid(Wind {
            dx: 0,
            dy: 10,
            pressure: 10,
            density: 180,
        });
        if rb % 4 != 0 && nbr.species == Species::EMPT && nbr.species != Species::WATR {
            let ra = 20 + api.rand_int(30) as u8;
            api.set(
                dx,
                dy,
                Cell {
                    species: Species::FIRE,
                    ra,
                    rb: 0,
                    clock: 0,
                },
            );
        }
        if nbr.species == Species::WATR {
            new_cell = Cell {
                species: Species::OIL,
                ra: 50,
                rb: 0,
                clock: 0,
            };
        }
    } else if rb == 1 {
        api.set(
            0,
            0,
            Cell {
                species: Species::EMPT,
                ra: cell.ra,
                rb: 90,
                clock: 0,
            },
        );
        return;
    }

    if api.get(0, 1).species == Species::EMPT {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, new_cell);
    } else if api.get(dx, 1).species == Species::EMPT {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, new_cell);
    } else if api.get(-dx, 1).species == Species::EMPT {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 1, new_cell);
    } else if api.get(dx, 0).species == Species::EMPT {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, new_cell);
    } else if api.get(-dx, 0).species == Species::EMPT {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 0, new_cell);
    } else {
        api.set(0, 0, new_cell);
    }
}

pub fn update_oxygen(cell: Cell, mut api: Api) {
    // make oxygen, which moves around but not up
    let dx = api.rand_dir();
    let dy = api.rand_dir();
    let nu = api.get(dx, dy);

    if nu.species == Species::EMPT {
        api.set(dx, dy, cell);
        api.set(0, 0, EMPTY_CELL);
    }


}

pub fn update_hydrogen(cell: Cell, mut api: Api) {
    // make hydrogen, which moves around but not up
    let dx = api.rand_dir();
    let dy = api.rand_dir();
    let nu = api.get(dx, dy);

    if nu.species == Species::EMPT {
        api.set(dx, dy, cell);
        api.set(0, 0, EMPTY_CELL);
    }

    //if any surrounding cells are OXGN, they will combine to make water and remove the hydrogen
    let nb = api.get(dx, dy);
    let nt = api.get(dx, dy);
    let nr = api.get(dx, dy);
    let nl = api.get(dx, dy);
    let ntr = api.get(dx, dy);
    let nbr = api.get(dx, dy);
    let nbl = api.get(dx, dy);

    let ntl = api.get(dx, dy);

    if nb.species == Species::OXGN {
        api.set(dx, dy, Cell {
            species: Species::WATR,
            ra: 0,
            rb: 0,
            clock: 0,
            ..Default::default()
        });
        api.set(0, 0, EMPTY_CELL);
    }

    if nt.species == Species::OXGN {
        api.set(dx, dy, Cell {
            species: Species::WATR,
            ra: 0,
            rb: 0,
            clock: 0,
            ..Default::default()
        });
        api.set(0, 0, EMPTY_CELL);
    }

    if nr.species == Species::OXGN {
        api.set(dx, dy, Cell {
            species: Species::WATR,
            ra: 0,
            rb: 0,
            clock: 0,
            ..Default::default()
        });
        api.set(0, 0, EMPTY_CELL);
    }

    if nl.species == Species::OXGN {
        api.set(dx, dy, Cell {
            species: Species::WATR,
            ra: 0,
            rb: 0,
            clock: 0,
            ..Default::default()
        });
        api.set(0, 0, EMPTY_CELL);
    }

    if ntr.species == Species::OXGN {
        api.set(dx, dy, Cell {
            species: Species::WATR,
            ra: 0,
            rb: 0,
            clock: 0,
            ..Default::default()
        });
        api.set(0, 0, EMPTY_CELL);
    }

    if nbr.species == Species::OXGN {
        api.set(dx, dy, Cell {
            species: Species::WATR,
            ra: 0,
            rb: 0,
            clock: 0,
            ..Default::default()
        });
        api.set(0, 0, EMPTY_CELL);
    }

    if nbl.species == Species::OXGN {
        api.set(dx, dy, Cell {
            species: Species::WATR,
            ra: 0,
            rb: 0,
            clock: 0,
            ..Default::default()
        });
        api.set(0, 0, EMPTY_CELL);
    }

    if ntl.species == Species::OXGN {
        api.set(dx, dy, Cell {
            species: Species::WATR,
            ra: 0,
            rb: 0,
            clock: 0,
            ..Default::default()
        });
        api.set(0, 0, EMPTY_CELL);
    }

    
}