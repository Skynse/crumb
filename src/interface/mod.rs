use std::default;

use crate::engine::element;
use crate::engine::Engine;

pub mod defaults;

use rand::Rng;
use sdl2::{mouse, pixels::Color};

const BACKGROUND_COLOR: Color = Color::RGB(0, 0, 0);
const CELL_SIZE: usize = 5;
const MAX_CURSOR_SIZE: usize = 20;

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

const DrawableList: [element::ElementType; 2] =
    [element::ElementType::Dust, element::ElementType::Wall];

impl Interface {
    pub fn run(mut engine_: Engine) {
        let selected_index = 0;
        let mut selected_element = DrawableList[0];
        let mut cursor_size = 3;
        

        let sdl = sdl2::init().expect("Failed to initialize SDL2");

        let mut canvas = {
            let video = sdl.video().expect("Failed to initialize video subsystem");
            let window = video
                .window("Crumb", defaults::WIDTH as u32, defaults::HEIGHT as u32)
                .position_centered()
                .vulkan()
                .build()
                .expect("Failed to create window");
            window
                .into_canvas()
                .accelerated()
                .present_vsync()
                .build()
                .expect("Failed to create canvas")
        };

        let mut event_pump = sdl.event_pump().expect("Failed to create event pump");
        // create a grid texture
        // get fps value
        

        loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => return,
                    sdl2::event::Event::KeyDown {
                        keycode: Some(sdl2::keyboard::Keycode::C),
                        ..
                    } => {
                        engine_.buffer.clear();
                    }
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

            // paint the buffer
            for y in (0..canvas.viewport().height() - 1).rev() {
                for x in 0..canvas.viewport().width() {
                    let element = engine_.buffer.get(x as usize, y as usize);
                    let color = match element {
                        element::ElementType::Empty => Color::RGB(0, 0, 0),
                        element::ElementType::Dust => varyColor(Color::RGB(255, 230, 224)),
                        element::ElementType::Wall => Color::RGB(160, 50, 50),
                    };

                    canvas.set_draw_color(color);
                    // draw a pixel using rect
                    canvas
                        .fill_rect(sdl2::rect::Rect::new(
                            x as i32 * CELL_SIZE as i32,
                            y as i32 * CELL_SIZE as i32,
                            CELL_SIZE as u32,
                            CELL_SIZE as u32,
                        ))
                        .expect("Failed to draw pixel");
                }
            }

            let mouse_state = event_pump.mouse_state();
            let mouse_x = mouse_state.x();
            let mouse_y = mouse_state.y();
            if mouse_state.left() {
                if is_in_window(mouse_x, mouse_y, 800, 600) {
                    for y in 0..cursor_size {
                        for x in 0..cursor_size {
                            // draw to the center of the cursor
                            engine_.buffer.set(
                                (mouse_x / CELL_SIZE as i32) as usize + x,
                                (mouse_y / CELL_SIZE as i32) as usize + y,
                                selected_element,
                            );
                        }
                    }
                    println!(
                        "Set element at {}, {}",
                        mouse_x as usize / CELL_SIZE,
                        mouse_y as usize / CELL_SIZE
                    );
                }
            }
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            // draw a circle for the cursor
            canvas
                .draw_rect(sdl2::rect::Rect::new(
                    mouse_x - cursor_size as i32 * CELL_SIZE as i32 / 2,
                    mouse_y - cursor_size as i32 * CELL_SIZE as i32 / 2,
                    cursor_size as u32 * CELL_SIZE as u32,
                    cursor_size as u32 * CELL_SIZE as u32,
                ))
                .expect("Failed to draw cursor");

            engine_.buffer.draw();
            canvas.present();
        }
    }
}
