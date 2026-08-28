use serde::{Deserialize, Serialize};

use crate::ecs::components::{Component, Mutable};

use super::vec::Vec2;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

impl Component for Rect {
    type Mutability = Mutable;
}

impl Rect {
    #[inline]
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    #[inline]
    pub fn new_xy(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            min: Vec2::new(x, y),
            max: Vec2::new(x + width, y + height),
        }
    }
    #[inline]
    pub fn new_vec(pos: Vec2, size: Vec2) -> Self {
        Self {
            min: pos,
            max: pos + size,
        }
    }

    #[inline]
    pub fn inside(&self, point: Vec2) -> bool {
        self.min.x <= point.x
            && self.min.y <= point.y
            && self.max.x >= point.x
            && self.max.y >= point.y
    }

    #[inline]
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    #[inline]
    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }

    #[inline]
    pub fn add_width(&mut self, value: f32) {
        self.max.x += value;
    }

    #[inline]
    pub fn add_height(&mut self, value: f32) {
        self.max.y += value;
    }

    #[inline]
    pub fn set_width(&mut self, value: f32) {
        self.max.x = self.min.x + value;
    }

    #[inline]
    pub fn set_height(&mut self, value: f32) {
        self.max.y = self.min.y + value;
    }

    #[inline]
    pub fn set_pos(&mut self, vec: Vec2) {
        let direction = vec - self.min;
        self.min = vec;
        self.max += direction;
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.min.x
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.min.y
    }
}
