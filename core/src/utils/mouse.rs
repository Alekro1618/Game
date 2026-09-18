use engine::{app::{main_schedule::ScheduleLabel, plugin::Plugin}, camera::{Camera, MainCamera}, ecs::{query::filter::With, schedule::config::{IntoSchedule, in_state}, systems::query::Single}, input::{button_input::ButtonInput, keyboard::KeyCode, mouse::MousePos}, math::{urn_function, vec::vec2}, utils::resources::GameSpeed, window::Window};

use crate::game_state::GameState;


fn move_camera_with_mouse(
    mouse_pos : &MousePos,
    window : &Window,
    mut main_camera : Single<&mut Camera, With<MainCamera>>,
    game_speed : &GameSpeed,
) {
    main_camera.translate(0.6 * game_speed.relative() * vec2(4. * urn_function( 2. * mouse_pos.pos.x / window.size.x - 1., 0.3), 0.));
}


fn move_camera_with_keyboard(
    keyboard : &ButtonInput<KeyCode>,
    mut main_camera : Single<&mut Camera, With<MainCamera>>,
    game_speed : &GameSpeed,
) {
    if keyboard.pressed(KeyCode::KeyA) {
        main_camera.translate(game_speed.relative() * vec2(-4., 0.));
    } 

    if keyboard.pressed(KeyCode::KeyD) {
        main_camera.translate(game_speed.relative() * vec2(4., 0.));
    }
}

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(self, app: &mut engine::app::App) {
        app
            .add_systems(
                ScheduleLabel::Update, 
                move_camera_with_mouse
                    .run_if(in_state(GameState::Simulator))
            )
            .add_systems(
                ScheduleLabel::Update, 
                move_camera_with_keyboard
                    .run_if(in_state(GameState::Simulator))
            );
    }
}