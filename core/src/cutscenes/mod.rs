use std::collections::HashMap;

use engine::{app::{main_schedule::ScheduleLabel, plugin::Plugin}, ecs::entities::id::EntityId, math::vec::Vec2};
use engine_macros::{Component, Resource};

use crate::cutscenes::{actor::ActorPlugin, script::ScriptPlugin, system::{read_text, update_cutscene}};

pub mod actor;
pub mod script;
pub mod other;
mod system;



#[derive(Clone, Default, PartialEq, Eq)]
pub enum RelativePosition {
    #[default]
    Center,
    OutOfBounds,
    Left(Level),
    Right(Level),
}

#[derive(Clone)]
pub enum TransitionOption {
    Immidiate,
    Smooth,
}

#[derive(Clone, Default, PartialEq, Eq)]
pub enum Level {
    Top,
    #[default]
    Center, 
    Bottom,
    Corner
}

pub fn relative_to_percent(relative : RelativePosition) -> f32 {
    match relative {
        RelativePosition::Center => {
            0.5
        },
        RelativePosition::Left(level) => {
            match level {
                Level::Top => 0.4,
                Level::Center => 0.3,
                Level::Bottom => 0.2,
                Level::Corner => -0.1,
            }
        },
        RelativePosition::Right(level) => {
            match level {
                Level::Top => 0.6,
                Level::Center => 0.7,
                Level::Bottom => 0.8,
                Level::Corner => 1.1
            }
        },
        RelativePosition::OutOfBounds => -1.0,
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
            )
            .add_systems(
                ScheduleLabel::FixedUpdate, 
                read_text
            );
    }
}