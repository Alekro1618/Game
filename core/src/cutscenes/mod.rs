use std::collections::HashMap;

use engine::{app::{main_schedule::ScheduleLabel, plugin::Plugin}, ecs::{change_detection::params::Mut, entities::id::EntityId, schedule::config::IntoSchedule}, input::{button_input::ButtonInput, keyboard::KeyCode, mouse::MouseButton}, math::vec::Vec2};
use engine_macros::{Component, Resource};
use serde::{Deserialize, Serialize};

use crate::cutscenes::{actor::ActorPlugin, script::{ScriptPlugin, script_state::ScriptState}, system::{update_cutscene}};

pub mod actor;
pub mod script;
pub mod other;
mod system;



#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelativePosition {
    #[default]
    Center,
    OutOfBounds,
    Left(Level),
    Right(Level),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionOption {
    Immidiate,
    Smooth,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Level {
    Top,
    #[default]
    Center, 
    Bottom,
    Side
}

impl RelativePosition {
    pub fn relative_to_percent(&self) -> f32 {
    match self {
        RelativePosition::Center => {
            0.5
        },
        RelativePosition::Left(level) => {
            match level {
                Level::Top => 0.4,
                Level::Center => 0.3,
                Level::Bottom => 0.2,
                Level::Side => -0.1,
            }
        },
        RelativePosition::Right(level) => {
            match level {
                Level::Top => 0.6,
                Level::Center => 0.7,
                Level::Bottom => 0.8,
                Level::Side => 1.1
            }
        },
        RelativePosition::OutOfBounds => -1.0,
    }
}
}

pub struct CutscenePlugin;

impl Plugin for CutscenePlugin {
    fn build(self, app: &mut engine::app::App) {
        app
            .add_plugin(ScriptPlugin)
            .add_plugin(ActorPlugin)
            .add_systems(
            ScheduleLabel::Update, 
            update_cutscene
            );
            
    }
}


