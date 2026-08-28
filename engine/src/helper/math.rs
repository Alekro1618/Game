impl Into<macroquad::math::Rect> for crate::math::rect::Rect {
    fn into(self) -> macroquad::math::Rect {
        macroquad::math::Rect::new(
            self.min.x,
            self.min.y,
            self.min.x + self.max.x,
            self.min.y + self.max.y,
        )
    }
}

impl Into<macroquad::math::Vec2> for crate::math::vec::Vec2 {
    fn into(self) -> macroquad::math::Vec2 {
        macroquad::math::Vec2::new(self.x, self.y)
    }
}
