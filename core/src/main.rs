
use std::marker::PhantomData;

use engine::{app::{main_schedule::ScheduleLabel, *}, ecs::{event::Event, message::messages::Messages, observer::system_param::On, systems::{commands::Commands, function_system::F1}}, input::InputPlugin, math::{transform::Transform, vec::{Vec2, vec2}}, render::{color::Color, sprite::Sprite, sprite_renderer::RenderPlugin}, utils::{resources::ResourcePlugin, timer::TimerPlugin}};
use engine_macros::Event;
use macroquad::{file::{set_pc_assets_folder}, miniquad::conf::Platform, window::Conf};

use crate::{items::{Item, food::Food}, machines::{machine::Machine, machine_function::MachineFunction, machine_state::MachineState, machine_type::MachineType}, players::{PlayerPlugin, player::{PendingAction, Player, PlayerAction}}, utils::{GridPlugin, grid::{self, Grid, LockToGrid}}};
mod algos;
mod utils;
mod manager;
mod players;
mod machines;
mod items;
mod client;

pub fn window_conf() -> Conf {
    Conf { 
        window_title: "MyGame".to_string(), 
        window_width: 1200, 
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
    set_pc_assets_folder("res");

    App::new()
        .add_plugin(GridPlugin)
        .add_plugin(RenderPlugin)
        .add_plugin(InputPlugin)
        //.add_plugin(ProfilerPlugin)
        .add_plugin(ResourcePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(TimerPlugin::<MachineType>(PhantomData))
        .add_systems::<F1<Commands>>(
            ScheduleLabel::Startup, setup
        )
        .run() 
        .await;
}
 
fn setup(
    mut commands : Commands
) {
    commands.insert(Grid::new(
            12, 
            5,
            100.0,
            100.0
        )
    );

    commands.spawn(
        (
            Player::from((0, 0),vec![
                    PlayerAction::WalkTo((1, 0).into()),
                    PlayerAction::WalkTo((2, 0).into()),
                    PlayerAction::WalkTo((2, 1).into()),
                    PlayerAction::WalkTo((2, 2).into()),
                    PlayerAction::WalkTo((3, 2).into()),
                    PlayerAction::WalkTo((4, 2).into()),
                ]
            ),
            Transform::from_position(vec2(0.0, 0.0)),
            Sprite::from_color(Color::RED, Vec2 { x: 100.0, y: 100.0 })
        )
    );

    commands.spawn(
        (
            Player::from((1, 1),vec![
                PlayerAction::WalkTo((1, 0).into()),
                PlayerAction::WalkTo((2, 0).into())
                ]
            ),
            Transform::from_position(vec2(100.0, 100.0)),
            Sprite::from_color(Color::RED, vec2(100.0, 100.0))
        )
    );

    commands.spawn(
        (
            Machine::from(MachineType::CoffeeBag),
            MachineState::Idle,
            MachineFunction::new(Item::None, Item::Food(Food::CoffeeBeans)),
            Transform::from_position(vec2(400.0, 200.0)),
            LockToGrid::new(grid::GridName::Interactables, (4, 2))
        )
    );
}




