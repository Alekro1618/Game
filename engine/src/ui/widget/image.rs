use crate::{ecs::components::{Component, Mutable, StorageType}, math::rect::Rect, render::color::Color};


pub struct Image {
    pub color: Color,
    pub image:  macroquad::texture::Image,
    pub flip_x: bool,
    pub flip_y: bool,
    pub rect: Option<Rect>,
    pub image_mode: NodeImageMode,
}


impl Component for Image {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Mutable;
}


impl Default for Image {
    fn default() -> Self {
        Image {
            color: Color::WHITE,
            image: macroquad::texture::Image::empty(),
            flip_x: false,
            flip_y: false,
            rect: None,
            image_mode: NodeImageMode::Auto,
        }
    }
}

impl Image {
    /// Create a new [`ImageNode`] with the given texture.
    pub fn new(texture: macroquad::texture::Image) -> Self {
        Self {
            image: texture,
            color: Color::WHITE,
            ..Default::default()
        }
    }

    /// Create a solid color [`ImageNode`].
    ///
    /// This is primarily useful for debugging / mocking the extents of your image.
    pub fn solid_color(color: Color) -> Self {
        Self {
            image: macroquad::texture::Image::empty(),
            color,
            flip_x: false,
            flip_y: false,
            rect: None,
            image_mode: NodeImageMode::Auto,
        }
    }

    /// Set the color tint
    #[must_use]
    pub const fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Flip the image along its x-axis
    #[must_use]
    pub const fn with_flip_x(mut self) -> Self {
        self.flip_x = true;
        self
    }

    /// Flip the image along its y-axis
    #[must_use]
    pub const fn with_flip_y(mut self) -> Self {
        self.flip_y = true;
        self
    }

    #[must_use]
    pub const fn with_rect(mut self, rect: Rect) -> Self {
        self.rect = Some(rect);
        self
    }

    #[must_use]
    pub const fn with_mode(mut self, mode: NodeImageMode) -> Self {
        self.image_mode = mode;
        self
    }
}

/// Controls how the image is altered to fit within the layout and how the layout algorithm determines the space in the layout for the image
#[derive(Default, Debug, Clone, PartialEq)]
pub enum NodeImageMode {
    /// The image will be sized automatically by taking the size of the source image and applying any layout constraints.
    #[default]
    Auto,
    /// The image will be resized to match the size of the node. The image's original size and aspect ratio will be ignored.
    Stretch,
    /// The texture will be repeated if stretched beyond `stretched_value`
    Tiled {
        /// Should the image repeat horizontally
        tile_x: bool,
        /// Should the image repeat vertically
        tile_y: bool,
        /// The texture will repeat when the ratio between the *drawing dimensions* of texture and the
        /// *original texture size* are above this value.
        stretch_value: f32,
    },
}