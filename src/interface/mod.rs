use crate::engine::Engine;
use sdl2::{pixels::Color, mouse};
use crate::engine::element;
use rand::Rng;

const BACKGROUND_COLOR: Color = Color::RGB(0, 0, 0);
const CELL_SIZE: usize = 8;
const CURSOR_SIZE: i32 = 1;

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

const DrawableList: [element::ElementType; 2] = [
    element::ElementType::Dust,
    element::ElementType::Wall,
];


impl Interface {
    pub fn run(mut engine_: Engine)  {
        let selected_index = 0;
        let mut selected_element = DrawableList[selected_index%DrawableList.len()];

        let sdl = sdl2::init().expect("Failed to initialize SDL2");
        println!("{}", engine_.buffer.elements.len());
        
        let mut canvas = {
            let video = sdl.video().expect("Failed to initialize video subsystem");
            let window = video.window("Crumb", 800, 600)
                .position_centered().vulkan()
                .build()
                .expect("Failed to create window");
            window.into_canvas().accelerated().present_vsync().build().expect("Failed to create canvas")
        };

        let mut event_pump = sdl.event_pump().expect("Failed to create event pump");
        // create a grid texture


        loop {
            engine_.buffer.set(300, 300, element::ElementType::Dust);
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit {..} => return,
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::C), .. } => {
                        engine_.buffer.clear();
                    },
                    sdl2::event::Event::MouseWheel { y, .. } => {
                        let mut new_index = selected_index as i32 + y;
                        if new_index < 0 {
                            new_index = DrawableList.len() as i32 - 1;
                        }
                        selected_element = DrawableList[new_index as usize % DrawableList.len()];
                        println!("Selected element: {:?}", selected_element);
                    },
                    _ => {}
                }
            }
            canvas.set_draw_color(BACKGROUND_COLOR);
            canvas.clear();
            
            // paint the buffer
            let mouse_state = event_pump.mouse_state();
            let mouse_x = mouse_state.x();
            let mouse_y = mouse_state.y();
            for y in (0..canvas.viewport().height()-1).rev() {
                for x in 0..canvas.viewport().width() {
                    let element = engine_.buffer.get(x as usize, y as usize);
                    let color = match element {
                        element::ElementType::Empty => Color::RGB(0, 0, 0),
                        element::ElementType::Dust => varyColor(Color::RGB(255, 255, 224)),
                        element::ElementType::Wall => Color::RGB(100, 50, 50),
                    };
                    canvas.set_draw_color(color);
                    // draw a pixel using rect
                    canvas.fill_rect(sdl2::rect::Rect::new(x as i32 * CELL_SIZE as i32, y as i32 * CELL_SIZE as i32, CELL_SIZE as u32, CELL_SIZE as u32)).expect("Failed to draw pixel");
                    // incrememnt the particle count

                    
                }
            }

            if mouse_state.left() {
                if is_in_window(mouse_x, mouse_y, 800, 600) {
                    // draw multiple particles at once to make it look like a spray
                    // offset 5 pixels in each direction from the mouse
                    for x in mouse_x-CURSOR_SIZE..mouse_x+CURSOR_SIZE {
                        for y in mouse_y-CURSOR_SIZE..mouse_y+CURSOR_SIZE {
                            if is_in_window(x, y, 800, 600) {
                            engine_.buffer.set(x as usize, y as usize, selected_element);
                            }
                        }
                        
                    }
                    
                }
            }
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            // draw a cursor
            canvas.draw_rect(sdl2::rect::Rect::new(mouse_x, mouse_y, 10, 10)).expect("Failed to draw rect");            
            engine_.buffer.draw();
            canvas.present();

        }

    }
}