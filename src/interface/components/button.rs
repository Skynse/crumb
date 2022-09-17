use sdl2::ttf;
use sdl2::pixels::Color;

#[allow(dead_code)]
pub struct Button {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    text: String,
    callback: fn(),
}

#[allow(dead_code)]
impl Button {
    pub fn new(x: i32, y: i32, width: i32, height: i32, text: String, callback: fn()) -> Button {
        Button {
            x,
            y,
            width,
            height,
            text,
            callback,
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, font: &ttf::Font) {
        let surface = font.render(self.text.as_str()).blended(Color::RGB(100, 100, 100)).unwrap();
        // use let binding to avoid borrowing issues
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        let texture_query = texture.query();
        canvas
            .copy(
                &texture,
                None,
                sdl2::rect::Rect::new(self.x, self.y, texture_query.width, texture_query.height),
            )
            .expect("Failed to copy texture");
    }

    pub fn update(&self, mouse: (i32, i32), mouse_pressed: bool) {
        if mouse.0 > self.x && mouse.0 < self.x + self.width && mouse.1 > self.y && mouse.1 < self.y + self.height {
            if mouse_pressed {
                (self.callback)();
            }
        }
    }

    


}