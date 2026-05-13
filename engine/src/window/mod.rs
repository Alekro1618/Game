use macroquad::{experimental::scene::Handle, miniquad::window::set_fullscreen, window::request_new_screen_size};

use crate::{app::{main_schedule::ScheduleLabel, plugin::Plugin}, ecs::{change_detection::params::Mut, components::{Component, Mutable, StorageType}, event::Event, observer::system_param::On, resource::Resource, schedule::set, systems::{commands::Commands, function_system::F1}}, math::vec::Vec2};

pub struct Window {
    size : Vec2,
    fullscreen : bool
}

impl Component for Window {
    const STORAGE_TYPE: crate::ecs::components::StorageType = StorageType::Table;

    type Mutability = Mutable;
}

impl Resource for Window {}

pub struct ApplyWindowSettings  {
    new_size : Option<Vec2>,
    is_fullscreen : Option<bool>
}

impl Event for ApplyWindowSettings {}

fn apply_window_settings(
    on : On<ApplyWindowSettings>,
    window : Mut<'_, Window>
) {
    let settings = on.event();

    if let Some(new_size)= settings.new_size {
        request_new_screen_size(new_size.x, new_size.y);
    }

    if let Some(is_fullscreen) = settings.is_fullscreen {
        set_fullscreen(is_fullscreen);
    }
}

pub struct WindowPlugin {
    pub window : Window
}

impl Plugin for WindowPlugin {
    fn build(&self, app:  &mut crate::app::App) {
        //let window = self.window;
        // app.add_systems::<F1<Commands>>(ScheduleLabel::PreStartup,  |mut commands: Commands| {
        //     commands.spawn((window, ));
        // });
    }
}