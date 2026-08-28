use std::collections::HashSet;

use macroquad::input::{
    is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released, mouse_position,
};

use crate::{
    ecs::{
        change_detection::params::Mut,
        components::{Component, Mutable, StorageType},
        resource::Resource,
    },
    input::{ButtonState, button_input::ButtonInput},
    math::vec::Vec2,
};

pub struct MouseButtonInput {
    pub button: MouseButton,
    pub state: ButtonState,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum MouseButton {
    /// The left mouse button.
    Left,
    /// The right mouse button.
    Right,
    /// The middle mouse button.
    Middle,
    /// The back mouse button.
    Back,
    /// The forward mouse button.
    Forward,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseMotion {
    pub delta: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MousePos {
    pub pos: Vec2,
}

impl Component for MousePos {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Mutable;
}

impl Resource for MousePos {}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
pub enum MouseScrollUnit {
    Line,
    Pixel,
}

impl MouseScrollUnit {
    pub const SCROLL_UNIT_CONVERSION_FACTOR: f32 = 100.;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseWheel {
    pub unit: MouseScrollUnit,
    pub x: f32,
    pub y: f32,
}

pub fn mouse_button_input_system(mut mouse_button_input: Mut<'_, ButtonInput<MouseButton>>) {
    mouse_button_input.reset_all();

    if is_mouse_button_released(macroquad::input::MouseButton::Left) {
        let mut d = HashSet::new();
        d.insert(MouseButton::Left);
        mouse_button_input.set_just_released(d);
    }

    if is_mouse_button_down(macroquad::input::MouseButton::Left) {
        let mut d = HashSet::new();
        d.insert(MouseButton::Left);
        mouse_button_input.set_pressed(d);
    }

    if is_mouse_button_pressed(macroquad::input::MouseButton::Left) {
        let mut d = HashSet::new();
        d.insert(MouseButton::Left);
        mouse_button_input.set_just_pressed(d);
    }
}

pub fn mouse_position_system(mut mouse_pos: Mut<'_, MousePos>) {
    mouse_pos.pos = mouse_position().into();
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct AccumulatedMouseMotion {
    pub delta: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AccumulatedMouseScroll {
    pub unit: MouseScrollUnit,
    pub delta: Vec2,
}

impl Default for AccumulatedMouseScroll {
    fn default() -> Self {
        Self {
            unit: MouseScrollUnit::Line,
            delta: Vec2::ZERO,
        }
    }
}
