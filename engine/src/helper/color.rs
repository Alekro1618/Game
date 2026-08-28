impl Into<macroquad::color::Color> for crate::render::color::Color {
    fn into(self) -> macroquad::color::Color {
        match self {
            crate::render::color::Color::RGBA(rgba) => {
                macroquad::color::Color::new(rgba.r, rgba.g, rgba.b, rgba.a)
            }
        }
    }
}

impl Into<macroquad::color::Color> for &crate::render::color::Color {
    fn into(self) -> macroquad::color::Color {
        match self {
            crate::render::color::Color::RGBA(rgba) => {
                macroquad::color::Color::new(rgba.r, rgba.g, rgba.b, rgba.a)
            }
        }
    }
}
