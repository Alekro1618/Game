use engine_macros::Component;

use crate::{
    asset::handle::Handle,
    image::{BLANK_IMAGE_HANDLE, Image, TRANSPARENT_IMAGE_HANDLE},
    math::rect::Rect,
    render::color::Color,
};

#[derive(Component)]
pub struct ImageNode {
    pub color: Color,
    pub image: Handle<Image>,
    pub flip_x: bool,
    pub flip_y: bool,
    pub rect: Option<Rect>,
    pub image_mode: NodeImageMode,
}

impl Default for ImageNode {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            image: TRANSPARENT_IMAGE_HANDLE.to_owned(),
            flip_x: false,
            flip_y: false,
            rect: None,
            image_mode: NodeImageMode::Auto,
        }
    }
}

impl ImageNode {
    /// Create a new [`ImageNode`] with the given texture.
    pub fn new(texture: Handle<Image>) -> Self {
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
            image: BLANK_IMAGE_HANDLE.to_owned(),
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
