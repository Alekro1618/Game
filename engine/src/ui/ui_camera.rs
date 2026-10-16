use engine_macros::{Component, Reflect, reflect};
use serde::{Deserialize, Serialize};

use crate::{camera::Camera, ecs::{change_detection::params::Mut, query::filter::With, systems::{commands::Commands, query::Single}}, math::{rect::Rect, vec::Vec2}, window::Window};

#[derive(Component, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct UiCamera;

pub fn setup_ui_camera(
    mut commands : Commands,
    window : &Window
) {
    commands.spawn((
        Camera::new(Rect::new(Vec2::ZERO, window.size)),
        UiCamera
    ));
}

pub fn resize_to_window(
    window : Mut<Window>,
    mut camera : Single<&mut Camera, With<UiCamera>>
) {
    camera.set_rect(Rect::new(Vec2::ZERO, window.size));
}