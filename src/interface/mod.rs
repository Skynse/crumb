use crate::engine::{Engine, species, Cell};
use crate::interface::components::*;

use species::Species;
pub mod defaults;
mod components;
use rand::Rng;
use sdl2::{mouse, pixels::Color, rect::Rect, render::Canvas};

use self::{defaults::{UI_X, UI_Y, WIDTH, HEIGHT}, button::Button};

const BACKGROUND_COLOR: Color = Color::RGB(0, 0, 0);
const MAX_CURSOR_SIZE: usize = 50;
const  PI: f32 = 3.1415926535;

pub fn varyColor(color: Color) -> Color {
    // vary color by 10%
    let vary_amount = rand::thread_rng().gen_range(1..50);
    let mut r = color.r;
    let mut g = color.g;
    let mut b = color.b;

    if r > 0 {
        r = r - vary_amount;
    }
    if g > 0 {
        g = g - vary_amount;
    }
    if b > 0 {
        b = b - vary_amount;
    }

    Color::RGB(r, g, b)
}

fn is_in_window(x: i32, y: i32, wx: i32, wy: i32) -> bool {
    x >= 0 && x < wx as i32 && y >= 0 && y < wy as i32
}
pub struct Interface {
    engine: Engine,
}
// create an array of possible cell species for our selector
const CELL_SPECIES: [Species; 9] = [
    Species::EMPT,
    Species::WALL,
    Species::DUST,
    Species::SAND,
    Species::WATR,
    Species::FIRE,
    Species::GAS,
    Species::OIL,
    Species::SMKE,
];

impl Interface {

    
    pub fn run(mut engine_: Engine) {

        let mut selected_index = 2;
        let mut cursor_size = 3;

        let mut paused: bool = false;
        // create a function pointer for pausing
        let mut pause = || {
            paused = !paused;
        };

        let ttf_context = sdl2::ttf::init().expect("Failed to initialize TTF");
            let font = ttf_context
                .load_font("./assets/FiraSans-Bold.ttf", 12)
                .expect("Failed to load font");

        let sdl = sdl2::init().expect("Failed to initialize SDL2");
        
        let mut canvas = {
            let video = sdl.video().expect("Failed to initialize video subsystem");
            let window = video
                .window("Crumb", defaults::WIDTH as u32, defaults::HEIGHT as u32)
                .position_centered()
                .build()
                .expect("Failed to create window");
            window
                .into_canvas()
                .accelerated()
                .present_vsync()
                .build()
                .expect("Failed to create canvas")
        };

        // change canvas scale to 2.0
        canvas.set_scale(2.0, 2.0).expect("Failed to set scale");
        let tc = canvas.texture_creator();
        let mut texture = tc
            .create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGB24, 1, 1)
            .expect("Failed to create texture");
    
        

        let mut event_pump = sdl.event_pump().expect("Failed to create event pump");

        // start game loop

        loop {
            
            for event in event_pump.poll_iter() {
                match event {
                    // when number keys are pressed, change the selected cell
                    sdl2::event::Event::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } => match keycode {
                        sdl2::keyboard::Keycode::Num1 => {
                            selected_index = 0;
                        }
                        sdl2::keyboard::Keycode::Num2 => {
                            selected_index = 1;
                        }
                        sdl2::keyboard::Keycode::Num3 => {
                            selected_index = 2;
                        }
                        sdl2::keyboard::Keycode::Num4 => {
                            selected_index = 3;
                        }
                        sdl2::keyboard::Keycode::Num5 => {
                            selected_index = 4;
                        }
                        sdl2::keyboard::Keycode::Num6 => {
                            selected_index = 5;
                        }
                        sdl2::keyboard::Keycode::Num7 => {
                            selected_index = 6;
                        }

                        // when spacebar is pressed, pause the simulation
                        sdl2::keyboard::Keycode::Space => {
                            paused = !paused;
                        }
                        // when the up arrow is pressed, increase the cursor size
                        sdl2::keyboard::Keycode::Up => {
                            if cursor_size < MAX_CURSOR_SIZE {
                                cursor_size += 1;
                            }
                        }
                        // when the down arrow is pressed, decrease the cursor size
                        sdl2::keyboard::Keycode::Down => {
                            if cursor_size > 1 {
                                cursor_size -= 1;
                            }
                        }

                        sdl2::keyboard::Keycode::C => {
                            engine_.world.clear();
                        }



                        // when the escape key is pressed, quit the simulation
                        sdl2::keyboard::Keycode::Escape => {
                            return;
                        }
                        _ => {}
                    },
                    sdl2::event::Event::Quit { .. } => return,
                    
                    sdl2::event::Event::MouseWheel { y, .. } => {
                        if y > 0 {
                            cursor_size = (cursor_size + 1).min(MAX_CURSOR_SIZE);
                        } else {
                            cursor_size = (cursor_size - 1).max(1);
                        }
                    }
                    _ => {}
                }
            }
            canvas.set_draw_color(BACKGROUND_COLOR);
            canvas.clear();


            // paint the world 
            for y in (0..canvas.viewport().height() - UI_Y as u32).rev() {
                for x in 0..canvas.viewport().width()-UI_X as u32 {
                    let cell = engine_.world.get(x as usize, y as usize);
                    let color = match cell.get_species() {
                        Species::EMPT => BACKGROUND_COLOR,
                        Species::WALL => Color::RGB(255, 255, 255),
                        Species::DUST => varyColor(Color::RGB(255, 200, 230)),
                        Species::SAND => varyColor(Color::RGB(255, 200, 100)),
                        Species::WATR => varyColor(Color::RGB(100, 100, 255)),
                        Species::GAS => varyColor(Color::RGB(255, 255, 255)),
                        Species::OIL => varyColor(Color::RGB(255, 100, 0)),
                        Species::FIRE => varyColor(Color::RGB(255, 120, 0)),
                        Species::SMKE => varyColor(Color::RGB(100, 100, 100)),
                    };
                    

                    canvas.set_draw_color(color);
                    // draw a pixel using rect
                    canvas
                        .fill_rect(Rect::new(x as i32, y as i32, 1, 1))
                        .expect("Failed to draw pixel");
                }
            }

            let mouse_state = event_pump.mouse_state();
            let mouse_x = mouse_state.x() /2;
            let mouse_y = mouse_state.y()/2;
            let full_mouse: (i32, i32) = (mouse_x, mouse_y);

            if mouse_state.left() {

                if is_in_window(mouse_x, mouse_y, (WIDTH) as i32, (HEIGHT) as i32) {
                    for y in 0..cursor_size {
                        for x in 0..cursor_size {
                            // draw to the center of the cursor
                            engine_.world.set(
                                ((mouse_x  as i32) as usize + x).saturating_sub(cursor_size / 2),
                                ((mouse_y as i32) as usize + y).saturating_sub(cursor_size / 2),
                                Cell::new(CELL_SPECIES[selected_index]),
                            );
                        }
                    }

                }
            }
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            // draw the cursor
            canvas
                .draw_rect(sdl2::rect::Rect::new(
                    mouse_x - cursor_size as i32/ 2,
                    mouse_y - cursor_size as i32 / 2,
                    cursor_size as u32,
                    cursor_size as u32,
                ))
                .expect("Failed to draw cursor");

            // print temperature of cell at mouse position
            let cell = engine_.world.get(mouse_x as usize, mouse_y as usize);
            let viewport = canvas.viewport();
            let vwidth = viewport.width();
            let vheight = viewport.height();
            // use let binding to avoid borrowing issues
            let text = format!("{:?}, Temp: {} C",cell.get_species(), cell.get_temperature());
            
            draw_text(&mut canvas, &font, text.as_str(), (0) as i32,  (0) as i32);
            

            // draw selected cell at the end of the screen
            draw_text(&mut canvas, &font, format!("{:?}", CELL_SPECIES[selected_index]).as_str(), vwidth as i32 - UI_X as i32, 0);
            

            if !paused {
                engine_.world.tick();
            }
            canvas.present();
        }
    }
}


fn draw_text(canvas: &mut Canvas<sdl2::video::Window>, font: &sdl2::ttf::Font, text: &str, x: i32, y: i32) {
    let surface = font.render(text).blended(Color::RGB(100, 100, 100)).unwrap();
    // use let binding to avoid borrowing issues
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
    let texture_query = texture.query();
    canvas
        .copy(
            &texture,
            None,
            Rect::new(x, y, texture_query.width, texture_query.height),
        )
        .expect("Failed to copy texture");
}
