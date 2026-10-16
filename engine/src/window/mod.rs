use engine_macros::{Reflect, Resource, reflect};
use macroquad::{
    miniquad::window::set_fullscreen,
    window::request_new_screen_size,
};
use serde::{Deserialize, Serialize};

use crate::{
    app::{main_schedule::ScheduleLabel, plugin::Plugin}, ecs::{
        change_detection::params::Mut, components::{Component, Mutable, StorageType}, event::Event, observer::system_param::On, resource::Resource, systems::commands::Commands,
    }, math::vec::{Vec2, vec2},
};

#[derive(Resource, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Resource, Component, Serialize, Deserialize)]
pub struct Window {
    pub size: Vec2,
    pub fullscreen: bool,
    pub is_resizable: bool
}

impl Window {
    pub fn new(size: Vec2, fullscreen: bool, is_resizable : bool) -> Self {
        Self { size, fullscreen, is_resizable }
    }
}

impl Default for Window {
    fn default() -> Self {
        Self {
            size: Vec2::new(0.0, 0.0),
            fullscreen: false,
            is_resizable: false
        }
    }
}

pub struct ApplyWindowSettings {
    new_size: Option<Vec2>,
    is_fullscreen: Option<bool>,
}

impl Event for ApplyWindowSettings {}

fn setup_window(mut commands : Commands) {
    let height = macroquad::window::screen_height();
    let width = macroquad::window::screen_width();

    commands.spawn(Window::new(vec2(height, width), false, true));
}

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

fn detect_macro_window_change_and_apply(mut window: Mut<Window>) {
    let height = macroquad::window::screen_height();
    let width = macroquad::window::screen_width();
    
    if height != window.size.y {
        window.size.y = height;
    }

    if width != window.size.x {
        window.size.x = width;
    }
}

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(self, app: &mut crate::app::App) {
        app
            .register_type::<Window>()
            .add_observer(
                apply_window_settings,
            )
            .add_systems(
                ScheduleLabel::PreStartup, 
                setup_window
            )
            .add_systems(
                ScheduleLabel::PreUpdate, 
                detect_macro_window_change_and_apply
            );
    }
}
