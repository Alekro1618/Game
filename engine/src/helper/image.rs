impl Into<macroquad::texture::Image> for crate::image::Image {
    fn into(self) -> macroquad::texture::Image {
        macroquad::texture::Image {
            bytes: self.data,
            width: self.width as u16,
            height: self.height as u16,
        }
    }
}
