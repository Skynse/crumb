use crate::engine::species::SPECIES_COUNT;
use crate::engine::{Engine, species, Cell};

use sdl2::{pixels::PixelFormatEnum};
use sdl2::{pixels::Color, rect::Rect, render::Canvas};

use species::Species;
pub mod defaults;
mod components;
use rand::Rng;

use self::{defaults::{UI_X, UI_Y, WIDTH, HEIGHT}};

const BACKGROUND_COLOR: Color = Color::RGB(0, 0, 0);
const MAX_CURSOR_SIZE: usize = 300;

const FONT: &[u8] = include_bytes!("res/Monocraft.ttf");

pub fn vary_color(color: Color) -> Color {
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
pub struct Interface;
// create an array of possible cell species for our selector

impl Interface {

    
    pub fn run(mut engine_: Engine) {
        let cell_species: &[Species] = &Species::all();


        let zoom = 100;

        let mut selected_index: usize = 2;
        let mut cursor_size = 3;

        let mut paused: bool = false;
        let mut ctrl_pressed: bool = false;
        let mut mouse_left_clicked: bool = false;
        
        // read font data and use in ttf_context
        let ttf_context = sdl2::ttf::init().unwrap();
        let mut font = ttf_context.load_font_from_rwops(sdl2::rwops::RWops::from_bytes(FONT).unwrap(), 8).unwrap();
        font.set_style(sdl2::ttf::FontStyle::BOLD);

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

        let mut event_pump = sdl.event_pump().expect("Failed to create event pump");
        // start game loop

        // draw appropriate textures
        let zoomed_texture_creator = canvas.texture_creator();
        let mut zoomed_texture = zoomed_texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, zoom as u32, zoom as u32)
            .unwrap();

        loop {
            let start_time = std::time::Instant::now();
            
            for event in event_pump.poll_iter() {
                match event {

                    sdl2::event::Event::MouseButtonDown { mouse_btn, .. } => {
                        if mouse_btn == sdl2::mouse::MouseButton::Left {
                            mouse_left_clicked = true;
                        }
                    }
                    sdl2::event::Event::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } => match keycode {

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
                        if ctrl_pressed {
                        if y > 0 {
                            cursor_size = (cursor_size + 1).min(MAX_CURSOR_SIZE);
                        } else {
                            cursor_size = (cursor_size - 1).max(1);
                        }
                    } else {
                        // set selected index to the index of the cell species based on scroll direction, wrapping around if necessary
                        if y > 0 {
                            selected_index = (selected_index + 1) % cell_species.len();
                        } else {
                            selected_index = (selected_index + cell_species.len() - 1) % cell_species.len();
                        }
                    }
                    }
                    _ => {
                        mouse_left_clicked = false;
                    }
                }
            }

            let keyboard_state = event_pump.keyboard_state();
            if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::LCtrl) {
               ctrl_pressed = true;
            } else {
                ctrl_pressed = false;
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
            
            

            if mouse_state.left() {

                if is_in_window(mouse_x, mouse_y, (WIDTH) as i32, (HEIGHT) as i32) {
                    for y in 0..cursor_size {
                        for x in 0..cursor_size {
                            // draw to the center of the cursor
                            engine_.world.set(
                                ((mouse_x  as i32) as usize + x).saturating_sub(cursor_size / 2),
                                ((mouse_y as i32) as usize + y).saturating_sub(cursor_size / 2),
                                Cell::new(cell_species[selected_index]),
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
            
            

            // draw selected cell at the end of the screen
            draw_text(&mut canvas, &font, format!("{:?}", cell_species[selected_index]).as_str(), vwidth as i32 - UI_X as i32, 0);
            

            // draw a zoomed in view of the nxn grid around the mouse at bottom left of the screen
            // check if z pressed 
            if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Z) {

            let zoomed_view = Rect::new(
                mouse_x - zoom as i32 / 2,
                mouse_y - zoom as i32 / 2,
                zoom as u32,
                zoom as u32,
            );
            let zoomed_view = zoomed_view
                .intersection(Rect::new(0, 0, WIDTH as u32, HEIGHT as u32))
                .unwrap();


            // actual zoom logic goes here
            let mut zoomed_pixels = Vec::new();
            for y in zoomed_view.y()..zoomed_view.y() + zoomed_view.height() as i32 {
                for x in zoomed_view.x()..zoomed_view.x() + zoomed_view.width()  as i32{
                    let cell = engine_.world.get(x as usize, y as usize);
                    let color = match cell.get_species() {
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
                        
                    };
                    zoomed_pixels.push(color);
                }
            }
            // convert zoomed_pixels to u8
            let mut zoomed_pixels_u8 = Vec::new();
            for pixel in zoomed_pixels {
                zoomed_pixels_u8.push(pixel.r);
                zoomed_pixels_u8.push(pixel.g);
                zoomed_pixels_u8.push(pixel.b);
            }

            zoomed_texture
                .update(None, &zoomed_pixels_u8, zoom as usize * 3)
                .unwrap();
                
            // draw outline of around zoomed view
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas
                .draw_rect(Rect::new(
                    mouse_x - zoom as i32 / 2,
                    mouse_y - zoom as i32 / 2,
                    zoom as u32,
                    zoom as u32,
                ))
                .expect("Failed to draw zoomed view");

            // draw outline around the zoom rect in the bottom left
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas
                .draw_rect(Rect::new(
                    0+1,
                    vheight as i32 - zoom as i32 -1,
                    zoom as u32,
                    zoom as u32,
                ))
                .expect("Failed to draw zoomed view");
            canvas
                .copy(
                    &zoomed_texture,
                    None,
                    Rect::new(
                        0,
                        vheight as i32 - zoom as i32,
                        zoom as u32,
                        zoom as u32,
                    ),
                )
                .unwrap();
            }
                
            if !paused {
                engine_.world.tick();
            }

            let end_time = std::time::Instant::now();
            let text = format!("{:?}, Temp: {} C, FPS: {:.2}",cell.get_species(), cell.get_temperature(), 1.0 / end_time.duration_since(start_time).as_secs_f32());
            
            draw_text(&mut canvas, &font, text.as_str(), (0) as i32,  (0) as i32);

            selected_index = draw_scrollbar(&mut canvas, &font, 0, vheight as i32 -UI_Y as i32, vwidth-UI_X as u32, UI_Y as u32, selected_index, cell_species, (mouse_x, mouse_y), mouse_left_clicked);

            canvas.present();
        }
    }
}


fn draw_text(canvas: &mut Canvas<sdl2::video::Window>, font: &sdl2::ttf::Font, text: &str, x: i32, y: i32) {
    let surface = font.render(text).blended(Color::RGB(150, 150, 150)).unwrap();
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

//implement horizontal scrollbar which displays clickable list of species
fn draw_scrollbar(canvas: &mut Canvas<sdl2::video::Window>, font: &sdl2::ttf::Font, x: i32, y: i32, width: u32, height: u32,mut selected_index: usize, species: &[Species], mouse_coords : (i32, i32), clicked: bool) -> usize {
    // draw scrollbar
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas
        .fill_rect(Rect::new(x, y, width, height))
        .expect("Failed to draw scrollbar");
    // draw scrollbar slider
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas
        .fill_rect(Rect::new(
            x + 1 + (selected_index as i32) * width as i32 / species.len() as i32,
            y + 1,
            width / species.len() as u32 - 2,
            height -1,
        ))
        .expect("Failed to draw scrollbar slider");
    // draw scrollbar text
    for (i, species) in species.iter().enumerate() {
        draw_text(
            canvas,
            font,
            format!("{:?}", species).as_str(),
            x as i32 + (i as u32 * width / SPECIES_COUNT as u32 ) as i32 + 5,
            y as i32 + 5,
        );
    }

    // if mouse on hover, draw some outline 
    if mouse_coords.0 > x && mouse_coords.0 < x + width as i32 && mouse_coords.1 > y && mouse_coords.1 < y + height as i32 {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas
            .draw_rect(Rect::new(x, y, width, height))
            .expect("Failed to draw scrollbar outline");
    }

    // check if mouse is hovering over scrollbar
    if mouse_coords.0 > x && mouse_coords.0 < x + width as i32 && mouse_coords.1 > y && mouse_coords.1 < y + height as i32 {
        // check if mouse is clicked
        if clicked {
            // get index at where we clicked
            selected_index = (mouse_coords.0 - x) as usize * SPECIES_COUNT / width as usize;
            // set selected_index to the index of the species that the mouse is hovering over
        }
        
    }
    return selected_index
}
