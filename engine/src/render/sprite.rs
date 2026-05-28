use crate::{asset::handle::Handle, ecs::components::{Component, Mutable, StorageType}, image::{BLANK_IMAGE_HANDLE, Image}, math::{rect::Rect, vec::Vec2}, render::color::Color};

pub struct Sprite {
    pub texture : Handle<Image>,
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
            texture: BLANK_IMAGE_HANDLE.to_owned(),
            color: color.into(),
            custom_size: Some(size),
            flip_x: false,
            flip_y: false,
            rect: None,
            image_mode: Default::default(),
        }
    }

    pub fn size(&self) -> Vec2 {

        if let Some(custom) = &self.custom_size {
            return *custom;
        } else {
            panic!("Sprite doesn't have size");
        }
    }
}