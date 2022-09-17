use sdl2::pixels::Color;

use crate::engine::Cell;
use crate::engine::species::Species;
use crate::interface::vary_color;

use super::BACKGROUND_COLOR;

pub fn cell_to_color(cell: Cell) -> Color {
    match cell.get_species() {
        Species::EMPT => BACKGROUND_COLOR,
        Species::WALL => Color::RGB(255, 255, 255),
        Species::DUST => vary_color(Color::RGB(255, 200, 230)),
        Species::SAND => vary_color(Color::RGB(255, 200, 100)),
        Species::WATR => vary_color(Color::RGB(100, 100, 255)),
        Species::GAS => vary_color(Color::RGB(255, 255, 255)),
        Species::OIL => vary_color(Color::RGB(255, 100, 0)),
        Species::FIRE => vary_color(Color::RGB(255, 120, 0)),
        Species::SMKE => vary_color(Color::RGB(100, 100, 100)),
        Species::GOL => match cell.rb {
            // check if cell is alive or dead when ra  is 1
            1 => vary_color(Color::RGB(255, 255, 255)),
            _ => vary_color(Color::RGB(0, 0, 0)),
        },
        Species::WOOD => vary_color(Color::RGB(100, 50, 0)),
        Species::OXGN => vary_color(Color::RGB(146,182,213)),
        Species::HYGN => vary_color(Color::RGB(51, 71, 109)),

        
    }
}