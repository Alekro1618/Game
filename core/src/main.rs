
use core::f32;

use engine::{animation::{recording::RecordPlayer}, app::{main_schedule::ScheduleLabel::{PostStartup, PreStartup, Startup}, *}, asset::{AssetServer, handle::Handle}, ecs::{name::Name, systems::commands::Commands}, image::Image, math::{transform::Transform, vec::vec2}, window::Window};
use macroquad::{miniquad::conf::Platform, window::Conf};

use crate::{actions::components::{ActionQueue, CurrentAction}, client::queue::{Queue, SpawnRandomClient}, cutscenes::{RelativePosition, TransitionOption, actor::{Actor, ActorBundle}, script::{ActorDefinition, Command, CurrentScript, Script, ScriptNode}}, instructions::{components::{InstructionLabel, SpecialLabel, Step::{Interact, MoveTo, WaitUntil}}, manager::InstructionManager}, items::{Item::{self}, food::Food}, machines::{functions::MachineFunction, machine::Machine, machine_type::MachineType, state::MachineState}, players::{events::Spawn, player_type::Service}, plugins::AllPlugins, ui::menu::main_menu, utils::grid::{self, Grid, GridParams, SpecialKeys}};
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
        vec2(800.0, 800.0), 
        false
    );

    App::new()
        .set_window(window)
        .add_plugin(AllPlugins)
        .add_systems(
            PreStartup, 
            setup
        )
        .add_systems(
            PostStartup, 
            then
        )
        .run() 
        .await;
}
 
fn setup(
    mut commands : Commands,
    asset_server : &AssetServer
) {
    let mut script = Script::default()
        .push_node(ScriptNode::ActorDefinition {
                name: "t".into(), 
                definition: ActorDefinition::new(
                    "test", 
                    "res/animations/test_manager.bin", 
                    "idle"
                ) 
            }
        )
        .push_node(ScriptNode::Command(Command::ShowCharacter("t".into(), RelativePosition::Center, TransitionOption::Smooth)))
        .push_node(ScriptNode::Replica { text: "hello it is test".into(), speaker: Some("t".into()) })
        .push_node(ScriptNode::Pause)
        .push_node(ScriptNode::Replica { text: "next test".into(), speaker: Some("t".into()) })
        .push_node(ScriptNode::Pause);

    script.prepare();

    let animation_manager = asset_server.load("res/animations/test_manager.bin");
    commands.spawn(ActorBundle::new("test", "idle", animation_manager));
    commands.insert(CurrentScript(script));

    let mut grid = Grid::new(
        20,
        20,
        GridParams::new(
            vec2(0., 0.),
            100., 
            100., 
            f32::consts::PI / 3.0, 
            vec2(400.0, 0.0),
    400.)
    );

    grid.create_map(grid::GridName::Interactables);
    grid.create_map(grid::GridName::Players);
    grid.create_map(grid::GridName::Walls);

    grid.add_special(SpecialKeys::CashRegister, (1, 3));
    grid.add_special(SpecialKeys::Entrence, (0, 0));
    grid.add_special(SpecialKeys::Exit, (3, 7));
    grid.add_special(SpecialKeys::ServicePlace, (4, 4));

    commands.insert(grid);

    let mut instruction_manager = InstructionManager::default();

    commands.trigger(Spawn(Service::First, (3, 0).into()));

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
    
    commands.spawn(
        Queue::new(vec![
            (2, 2).into(),
            (2, 1).into(),
            (1, 1).into(),
            (0, 1).into()
        ])
    );

    commands.spawn(
        (
            Machine::from(MachineType::CoffeeBag),
            MachineType::CoffeeBag,
            MachineState::Idle,
            MachineFunction::new()
                .add(None, Some(Item::Food(Food::CoffeeBeans))),
        )
    );

    commands.spawn(
        (
            Machine::from(MachineType::CoffeeMachine),
            MachineType::CoffeeMachine,
            MachineState::Idle,
            MachineFunction::new()
                .add(Some(Item::Food(Food::CoffeeBeans)), Some(Item::Food(Food::Espresso))),
        )
    );

    commands.spawn(
        (
            Machine::from(MachineType::CashRegister),
            MachineType::CashRegister,
            MachineState::Idle,
            MachineFunction::new()
                .add(None, None)
                .add(Some(Item::Food(Food::Espresso)), None),
        )
    );

    commands.spawn(
        (
            Machine::from(MachineType::MilkStation),
            MachineType::MilkStation,
            MachineState::Idle,
            MachineFunction::new()
                .add(Some(Item::Food(Food::Espresso)), Some(Item::Food(Food::Latte))),
        )
    );

    
}

fn then(
    mut commands : Commands
) {
    
}

// fn move_camera_with_mouse(
//     mouse_pos : &MousePos,
//     window : &Window,
//     query_main_camera : Query<&mut Camera, With<MainCamera>>
// ) {
//     for mut camera in query_main_camera {
//         camera.translate(6. * vec2(4. * urn_function( 2. * mouse_pos.pos.x / window.size.x - 1., 0.3), 0.));
//     }
// }

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

    #[test]
    fn ttt() {
        let mut a = 0;
        let mut b = 0;
        let mut c= 0;
        for i in 1..10 {
            c = b;
            b = a;
            a = i;
            println!("a : {}, b: {}, c: {}", a, b, c);
        }
    }
}