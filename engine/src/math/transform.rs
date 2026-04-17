use crate::{ecs::components::{Component, Mutable, StorageType}, math::vec::Vec2};

pub struct Transform {
    pub position : Vec2,
    pub rotation : f32,
    pub scale : Vec2
}

impl Component for Transform {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Mutable;
}


impl Transform {
    pub const IDENTITY: Self = Transform {
        position: Vec2::ZERO,
        rotation: 0.0,
        scale: Vec2::ONE,
    };

    #[inline]
    pub const fn from_xy(x: f32, y: f32) -> Self {
        Self::from_position(Vec2::new(x, y))
    }

    #[inline]
    pub const fn from_position(position: Vec2) -> Self {
        Transform {
            position,
            ..Self::IDENTITY
        }
    }

    #[inline]
    pub const fn from_rotation(rotation: f32) -> Self {
        Transform {
            rotation,
            ..Self::IDENTITY
        }
    }

    #[inline]
    pub const fn from_scale(scale: Vec2) -> Self {
        Transform {
            scale,
            ..Self::IDENTITY
        }
    }

    #[inline]
    pub fn translate(&mut self, vector : Vec2) {
        self.position += vector;
    }

    #[inline]
    pub fn rotate(&mut self, angle : f32) {
        self.rotation += angle;
    }

    #[inline]
    pub fn scale(&mut self, vector : Vec2) {
        self.scale += vector;
    }
}