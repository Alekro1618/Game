
use core::f32;

use engine::{app::{main_schedule::ScheduleLabel::{self, PostStartup, PreStartup}, *}, asset::AssetServer, ecs::{change_detection::params::Mut, systems::commands::Commands}, math::vec::vec2, utils::Either::Left, window::Window};
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
        window_resizable: false,
        icon: None,
        platform: Platform::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let window = Window::new(
        vec2(800.0_f32, 800.0_f32), 
        false
    );

    App::new()
        .set_window(window)
        .add_plugin(AllPlugins)
        .add_systems(ScheduleLabel::PreStartup, setup)
        .run() 
        .await;
}
 
fn setup(
    mut commands : Commands,
    asset_server : &AssetServer,
    mut current : Mut<CurrentScript>
) {
    let handle = asset_server.load::<Script>("res/scripts/test.txt");
    let mut instruction_manager = InstructionManager::default();

    current.0 = Some(handle);

    instruction_manager.create_new(
        InstructionLabel::Create(Item::Food(Food::Espresso)), 
        vec![
            MoveTo((4, 1).into()),
            Interact(MachineType::CoffeeBag, None),
            MoveTo((0, 5).into()),
            Interact(MachineType::CoffeeMachine, Some(Item::Food(Food::CoffeeBeans))),
            WaitUntil(MachineType::CoffeeMachine),
            Interact(MachineType::CoffeeMachine, None),
            MoveTo((2, 3).into()),
            Interact(MachineType::CashRegister, Some(Item::Food(Food::Espresso)))
        ].into_iter()
    );

    instruction_manager.create_new(
        InstructionLabel::Create(Item::Food(Food::Latte)), 
        vec![
            MoveTo((4, 1).into()),
            Interact(MachineType::CoffeeBag, None),
            MoveTo((0, 5).into()),
            Interact(MachineType::CoffeeMachine, Some(Item::Food(Food::CoffeeBeans))),
            WaitUntil(MachineType::CoffeeMachine),
            Interact(MachineType::CoffeeMachine, None),
            MoveTo((4, 4).into()),
            Interact(MachineType::MilkStation, Some(Item::Food(Food::Espresso))),
            WaitUntil(MachineType::MilkStation),
            MoveTo((2, 3).into()),
            Interact(MachineType::CashRegister, Some(Item::Food(Food::Latte)))
        ].into_iter()
    );

    instruction_manager.create_new(
        InstructionLabel::WalkTo(MachineType::CashRegister),
        vec![
            MoveTo((2, 2).into()),
            Interact(MachineType::CashRegister, None)
        ].into_iter(),
    );

    instruction_manager.create_new(
        InstructionLabel::Special(SpecialLabel::Exit),
        vec![
            MoveTo((0, 0).into())
        ].into_iter(),
    );

    commands.insert(instruction_manager);
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