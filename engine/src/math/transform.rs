use engine_macros::{Component, Reflect, reflect};
use serde::{Deserialize, Serialize};

use crate::{
    app::plugin::Plugin, math::vec::Vec2,
};

pub struct TransformPlugin;

impl Plugin for TransformPlugin {
    fn build(self, app: &mut crate::app::App) {
        app.register_type::<Transform>();
    }
}

#[derive(Clone, Copy, Serialize, Component, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize, Component)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
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
    pub fn translate(&mut self, vector: Vec2) {
        self.position += vector;
    }

    #[inline]
    pub fn rotate(&mut self, angle: f32) {
        self.rotation += angle;
    }

    #[inline]
    pub fn scale(&mut self, vector: Vec2) {
        self.scale += vector;
    }
}
