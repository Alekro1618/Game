
use core::f32;

use engine::{app::{main_schedule::ScheduleLabel::{self, PostStartup, PreStartup}, *}, asset::AssetServer, camera::{Camera, SetMainCamera}, ecs::{change_detection::params::Mut, query::filter::Changed, systems::{commands::Commands, query::Query}}, math::{rect::Rect, vec::vec2}, utils::{Either::Left, profiler::ProfilerPlugin}, window::Window};
use engine_macros::Component;
use macroquad::{miniquad::conf::Platform, window::Conf};

use crate::{client::queue::Queue, cutscenes::{Level::Top, RelativePosition, TransitionOption, actor::ActorBundle, script::{ActorDefinition, Command, CurrentScript, Script, ScriptNode}}, instructions::{components::{InstructionLabel, SpecialLabel, Step::{Interact, MoveTo, WaitUntil}}, manager::InstructionManager}, items::{Item::{self}, food::Food}, machines::{functions::MachineFunction, machine::Machine, machine_type::MachineType, state::MachineState}, players::{events::Spawn, player_type::Service}, plugins::AllPlugins, utils::grid::{self, Grid, GridParams, SpecialKeys}};
mod algos;
mod utils;
mod players;
mod machines;
mod items;
mod client;
mod service;
mod order;
mod enemy;
mod minigames;
mod ui;
mod actions;
mod bundles;
mod cutscenes;
mod plugins;
mod instructions;
mod game_state;
mod scenes;

pub fn window_conf() -> Conf {
    Conf { 
        window_title: "MyGame".to_string(), 
        window_width: 800, 
        window_height: 800,
        high_dpi: false, 
        fullscreen: false, 
        sample_count: 1, 
        window_resizable: true,
        icon: None,
        platform: Platform::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let exit = App::new()
        .add_plugin(AllPlugins)
        .add_systems(ScheduleLabel::PreStartup, 
            setup
        )
        .run() 
        .await;

    if let AppExit::Error(error) =  exit {
        println!("{}", error);
    }
}


fn setup(
    commands : Commands,
) {
    commands
        .spawn_with_id(Camera::new(Rect::new(vec2(0., 0.), vec2(800., 800.))))
        .then(|mut commands, entity| {
            commands.trigger(SetMainCamera(entity));
        });
}


#[cfg(test)]
mod tests {
use std::{collections::HashMap, fs};

use engine::{animation::{manager::{AnimationManager, AnimationManagerErased, Transition, TransitionStyle}, recording::Recording, sprite::{TextureFrame, TextureRecording, TextureRecordingDeserializer, TextureState}}, asset::AssetPath, math::{rect::Rect, vec::vec2}};
use postcard::{to_allocvec, to_vec};

    #[test]
    fn save_recording() {
        let recording = TextureRecordingDeserializer::new(
            "res/images/ui/button.png".into(), 
            vec![
                TextureFrame::new(TextureState::new(Rect::new(vec2(10., 10.), vec2(40., 40.)), None), 0.0)
            ], 
            4.0
        );
        
        let data = to_allocvec(&recording).unwrap();
        fs::write("test_idle.bin", data).expect("Failed to write file");
    }

    #[test]
    fn save_animation_manager() {
        
        let recording = AnimationManagerErased {
            recordings: HashMap::from_iter([
                ("idle".into(), "res/animations/test_idle.bin".into()),
                ("happy".into(), "res/animations/test_happy.bin".into()),
            ]),
            transitions: vec![
                Transition::new("idle", "happy", TransitionStyle::OnEnd),
                Transition::new("happy", "idle", TransitionStyle::OnEnd),
            ],
        };
        
        let data = to_allocvec(&recording).unwrap();
        fs::write("test_manager.bin", data).expect("Failed to write file");
    }
}
