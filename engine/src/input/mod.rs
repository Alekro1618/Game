use crate::{
    app::{App, main_schedule::ScheduleLabel, plugin::Plugin},
    input::{
        keyboard::keyboard_input_system,
        mouse::{mouse_button_input_system, mouse_position_system},
    },
};

pub mod button_input;
pub mod keyboard;
pub mod mouse;

#[derive(Default)]
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(self, app: &mut App) {
        app.add_systems(
            ScheduleLabel::PreUpdate,
            keyboard_input_system,
        );
        app.add_systems(
            ScheduleLabel::PreUpdate,
            mouse_button_input_system,
        );
        app.add_systems(ScheduleLabel::PreUpdate, mouse_position_system);
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
