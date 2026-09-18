use macroquad::{
    miniquad::window::set_fullscreen,
    window::request_new_screen_size,
};

use crate::{
    app::plugin::Plugin, ecs::{
        change_detection::params::Mut, components::{Component, Mutable, StorageType}, event::Event, observer::system_param::On, resource::Resource,
    }, math::vec::Vec2,
};

pub struct Window {
    pub size: Vec2,
    pub fullscreen: bool,
}

impl Window {
    pub fn new(size: Vec2, fullscreen: bool) -> Self {
        Self { size, fullscreen }
    }
}

impl Component for Window {
    const STORAGE_TYPE: crate::ecs::components::StorageType = StorageType::Table;

    type Mutability = Mutable;
}

impl Resource for Window {}

impl Default for Window {
    fn default() -> Self {
        Self {
            size: Vec2::new(0.0, 0.0),
            fullscreen: false,
        }
    }
}

pub struct ApplyWindowSettings {
    new_size: Option<Vec2>,
    is_fullscreen: Option<bool>,
}

impl Event for ApplyWindowSettings {}

fn apply_window_settings(on: On<ApplyWindowSettings>, mut window: Mut<Window>) {
    let settings = on.event();

    if let Some(new_size) = settings.new_size {
        request_new_screen_size(new_size.x, new_size.y);
        window.size = new_size;
    }

    if let Some(is_fullscreen) = settings.is_fullscreen {
        set_fullscreen(is_fullscreen);
        window.fullscreen = is_fullscreen;
    }
}

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(self, app: &mut crate::app::App) {
        app
            .add_observer(
                apply_window_settings,
            );
    }
}
