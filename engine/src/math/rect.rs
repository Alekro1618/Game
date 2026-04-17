use super::vec::Vec2;

pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

impl Rect {
    pub fn new(min: Vec2, max : Vec2) -> Self {
        Self {min, max}
    }

    pub fn new_xy(x : f32, y : f32, width : f32, height : f32) -> Self {
        Self { 
            min: Vec2::new(x, y),
            max: Vec2::new(x + width, y + height)
        }
    }
}
