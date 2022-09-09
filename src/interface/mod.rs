use crate::engine::Engine;
use sdl2::pixels::Color;
use crate::engine::element;

const BACKGROUND_COLOR: Color = Color::RGB(0, 0, 0);

pub struct Interface {
    engine: Engine,
}

impl Interface {

    pub fn run(mut engine_: Engine)  {
        let sdl = sdl2::init().expect("Failed to initialize SDL2");
        let mut canvas = {
            let video = sdl.video().expect("Failed to initialize video subsystem");
            let window = video.window("Crumb", 800, 600)
                .position_centered()
                .build()
                .expect("Failed to create window");
            window.into_canvas().accelerated().present_vsync().build().expect("Failed to create canvas")
        };

        let mut event_pump = sdl.event_pump().expect("Failed to create event pump");

        loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit {..} => return,
                    _ => {}
                }
            }
            canvas.set_draw_color(BACKGROUND_COLOR);
            canvas.clear();

            // Draw the buffer
            for x in 0..600 {
                for y in 0..600 {
                    let element = engine_.buffer.get(x, y);
                    let color = match element {
                        element::ElementType::Empty => Color::RGB(0, 0, 0),
                        element::ElementType::Dust => Color::RGB(240, 230, 255),
                        element::ElementType::Wall => Color::RGB(50, 50, 50),
                    };
                    canvas.set_draw_color(color);
                    canvas.draw_point(sdl2::rect::Point::new(x as i32, y as i32)).expect("Failed to draw point");
                }
            }

            // draw one pixel of dust
            engine_.buffer.draw();
            // add movement

            // get mouse position
            // if mouse is pressed, add dust
            let mouse_state = event_pump.mouse_state();
            if mouse_state.left() {
                let (x, y) = (mouse_state.x(), mouse_state.y());
                engine_.buffer.set(x as usize, y as usize, element::ElementType::Dust);
            }
            canvas.present();

        }

    }
}