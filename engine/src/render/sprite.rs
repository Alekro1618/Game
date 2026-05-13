use macroquad::texture::Texture2D;

use crate::{ecs::components::{Component, Mutable, StorageType}, math::{rect::Rect, vec::Vec2}, render::color::Color};

#[derive(Default)]
pub struct Sprite {
    pub texture : Option<Texture2D>,
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
    pub fn from_color(color: impl Into<Color>, size: Vec2) -> Self {
        Self {
            color: color.into(),
            custom_size: Some(size),
            ..Default::default()
        }
    }

    pub fn size(&self) -> Vec2 {
        if let Some(txt) = &self.texture {
            let d = txt.size();
            return Vec2::new(d.x, d.y);
        }

        if let Some(custom) = &self.custom_size {
            return *custom;
        } else {
            panic!("Sprite doesn't have size");
        }
    }
}