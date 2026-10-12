use engine_macros::Component;

use crate::{
    asset::handle::Handle,
    ecs::components::{Component, Mutable, StorageType},
    image::{BLANK_IMAGE_HANDLE, Image},
    math::{rect::Rect, vec::Vec2},
    render::color::Color,
};

pub struct Sprite {
    pub texture: Handle<Image>,
    pub color: Color,
    pub flip_x: bool,
    pub flip_y: bool,
    pub custom_size: Option<Vec2>,
    pub rect: Option<Rect>,
    pub image_mode: SpriteImageMode,
}

#[derive(Default)]
pub enum SpriteImageMode {
    #[default]
    Auto,
    Scale,
    Tiled {
        tile_x: bool,
        tile_y: bool,
        stretch_value: f32,
    },
}

impl Component for Sprite {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Mutable;
}

impl Sprite {
    pub fn none() -> Self {
        Self::from_color(Color::NONE, Vec2::ZERO)
    }

    pub fn from_image(image : Handle<Image>, size: Vec2) -> Self {
        Self { 
            texture: image, 
            color: Color::WHITE, 
            flip_x: false,
            flip_y: false, 
            custom_size: Some(size), 
            rect: None, 
            image_mode: Default::default()
        }
    }

    pub fn from_color(color: impl Into<Color>, size: Vec2) -> Self {
        Self {
            texture: BLANK_IMAGE_HANDLE.to_owned(),
            color: color.into(),
            custom_size: Some(size),
            flip_x: false,
            flip_y: false,
            rect: None,
            image_mode: Default::default(),
        }
    }

    pub fn set_texture(&mut self, image : Handle<Image>) {
        self.texture = image;
    }

    pub fn set_target(&mut self, target : Rect) {
        self.rect = Some(target)
    }

    pub fn set_alpha(&mut self, alpha : f32) {
        self.color.set_alpha(alpha);
    }

    pub fn size(&self) -> Vec2 {
        if let Some(custom) = &self.custom_size {
            return *custom;
        } else {
            panic!("Sprite doesn't have size");
        }
    }
}


#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Anchor(pub Vec2);

impl Eq for Anchor {}

impl Anchor {
    pub const BOTTOM_LEFT: Self = Self(Vec2::new(-0.5, -0.5));
    pub const BOTTOM_CENTER: Self = Self(Vec2::new(0.0, -0.5));
    pub const BOTTOM_RIGHT: Self = Self(Vec2::new(0.5, -0.5));
    pub const CENTER_LEFT: Self = Self(Vec2::new(-0.5, 0.0));
    pub const CENTER: Self = Self(Vec2::ZERO);
    pub const CENTER_RIGHT: Self = Self(Vec2::new(0.5, 0.0));
    pub const TOP_LEFT: Self = Self(Vec2::new(-0.5, 0.5));
    pub const TOP_CENTER: Self = Self(Vec2::new(0.0, 0.5));
    pub const TOP_RIGHT: Self = Self(Vec2::new(0.5, 0.5));

    pub fn as_vec(&self) -> Vec2 {
        self.0
    }
}

impl Default for Anchor {
    fn default() -> Self {
        Self::CENTER
    }
}

impl From<Vec2> for Anchor {
    fn from(value: Vec2) -> Self {
        Self(value)
    }
}