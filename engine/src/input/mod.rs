use crate::{app::{App, main_schedule::ScheduleLabel, plugin::Plugin}, ecs::systems::function_system::F1, input::{button_input::ButtonInput, keyboard::{KeyCode, keyboard_input_system}, mouse::{MouseButton, MousePos, mouse_button_input_system, mouse_position_system}}};

pub mod button_input;
pub mod keyboard;
pub mod mouse;



#[derive(Default)]
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(self, app:  &mut App) {
        app.add_systems::<F1<&mut ButtonInput<KeyCode>>>(ScheduleLabel::PreUpdate, keyboard_input_system);
        app.add_systems::<F1<&mut ButtonInput<MouseButton>>>(ScheduleLabel::PreUpdate, mouse_button_input_system);
        app.add_systems::<F1<&mut MousePos>>(ScheduleLabel::PreUpdate, mouse_position_system);
    }   
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ButtonState {
    /// The button is pressed.
    Pressed,
    /// The button is not pressed.
    Released,
}

impl ButtonState {
    /// Is this button pressed?
    pub fn is_pressed(&self) -> bool {
        matches!(self, ButtonState::Pressed)
    }
}