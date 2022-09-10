use sdl2::pixels::Color;
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum ElementType {
    #[default]
    Empty,
    Wall,
    Dust,
}

pub struct Element {
    name: String,
    element_type: ElementType,
    color: Color,
    gravity: f32,
    static_: u32,

    flammable: bool,
    explosive: bool,
    meltable: bool,
    hardness: f32,

    air_drag: f32,
    air_loss: f32,

    weight: f32,
}

impl Element {
    pub fn new(element_type: ElementType) -> Self {
        match element_type {
            ElementType::Empty => Self {
                name: "Empty".to_string(),
                element_type,
                color: Color::RGB(0, 0, 0),
                gravity: 0.0,
                static_: 0,
                flammable: false,
                explosive: false,
                meltable: false,
                hardness: 0.0,
                weight: 0.0,
                air_drag: 0.0,
                air_loss: 0.0,
            },
            ElementType::Dust => Self {
                name: "Dust".to_string(),
                element_type,
                color: Color::RGB(240, 230, 255),
                gravity: 0.0,
                static_: 0,
                flammable: true,
                explosive: false,
                meltable: false,
                hardness: 0.0,
                weight: 0.0,
                air_drag: 0.0,
                air_loss: 0.0,
            },
            ElementType::Wall => Self {
                name: "Wall".to_string(),
                element_type,
                color: Color::RGB(10, 10, 10),
                gravity: 0.0,
                static_: 0,
                flammable: false,
                explosive: false,
                meltable: false,
                hardness: 0.0,
                weight: 0.0,
                air_drag: 0.0,
                air_loss: 0.0,
            },
        }
    }
}
