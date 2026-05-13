use super::vec::Vec2;

pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

impl Rect {
    #[inline]
    pub fn new(min: Vec2, max : Vec2) -> Self {
        Self {min, max}
    }

    #[inline]
    pub fn new_xy(x : f32, y : f32, width : f32, height : f32) -> Self {
        Self { 
            min: Vec2::new(x, y),
            max: Vec2::new(x + width, y + height)
        }
    }
    #[inline]
    pub fn new_vec(pos : Vec2, size : Vec2) -> Self {
        Self { 
            min: pos, 
            max: pos + size
        }
    }

    #[inline]
    pub fn inside(&self, point : Vec2) -> bool {
        self.min.x <= point.x && self.min.y <= point.y &&
        self.max.x >= point.x && self.max.y >= point.y
    }
}
