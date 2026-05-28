
use std::marker::PhantomData;

use engine::{app::{main_schedule::ScheduleLabel, *}, ecs::systems::{commands::Commands, function_system::{F1, F2}}, input::{InputPlugin, button_input::ButtonInput, keyboard::KeyCode}, math::{transform::Transform, vec::{Vec2, vec2}}, render::{color::Color, sprite::Sprite, sprite_renderer::RenderPlugin}, utils::{resources::ResourcePlugin, *}, window::Window};
use engine_macros::Event;
use macroquad::{file::{set_pc_assets_folder}, miniquad::conf::Platform, window::Conf};

use crate::{client::{ClientPlugin, queue::{MoveQueue, SpawnRandomClient}}, enemy::{EnemyPlugin, enemy::SpawnRandomEnemy}, items::{Inventory, InventoryPlugin, Item, food::Food}, machines::{MachinePlugins, machine::Machine, machine_function::MachineFunction, machine_state::MachineState, machine_type::MachineType}, order::OrderPlugin, players::{PlayerPlugin, player::{Interact, PendingAction, Player, PlayerAction, SetPath}, player_type::{Client, PlayerType, Service}, spawner::{SpawnClient, SpawnService}}, service::ServicePlugin, utils::{GridPlugin, grid::{self, Grid, LockToGrid, SpecialKeys}}};
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


pub fn window_conf() -> Conf {
    Conf { 
        window_title: "MyGame".to_string(), 
        window_width: 0, 
        window_height: 0,
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
        .add_plugin(GridPlugin)
        .add_plugin(RenderPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(ResourcePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(MachinePlugins)
        .add_plugin(ClientPlugin)
        .add_plugin(ServicePlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(InventoryPlugin)
        .add_plugin(OrderPlugin)
        .add_systems::<F1<Commands>>(
            ScheduleLabel::PreStartup, setup
        )
        .add_systems::<F1<Commands>>(
            ScheduleLabel::PostStartup, post_setup
        )
        .add_systems::<F2<Commands, &ButtonInput<KeyCode>>>(
            ScheduleLabel::FixedUpdate, cont
        )
        .run() 
        .await;
}
 
fn setup(
    mut commands : Commands,
) {
    let mut grid = Grid::new(
        200, 
        200,
        100.0,
        100.0
    );

    grid.create_map(grid::GridName::Interactables);
    grid.create_map(grid::GridName::Players);
    grid.create_map(grid::GridName::Walls);

    grid.add_special(SpecialKeys::CashRegister, (1, 3));
    grid.add_special(SpecialKeys::Entrence, (0, 0));
    grid.add_special(SpecialKeys::Exit, (3, 7));
    grid.add_special(SpecialKeys::ServicePlace, (4, 4));

    commands.insert(
        grid
    );

    commands.spawn(
        (
            Machine::from(MachineType::CoffeeBag),
            MachineState::Idle,
            MachineFunction::new()
                .add(None, Some(Item::Food(Food::CoffeeBeans))),
            Transform::from_position(vec2(0.0, 0.0)),
            Sprite::from_color(Color::GREEN, Vec2 { x: 100.0, y: 100.0 }),
            LockToGrid::new(grid::GridName::Interactables, (4, 2))
        )
    );

    commands.spawn(
        (
            Machine::from(MachineType::CoffeeMachine),
            MachineState::Idle,
            MachineFunction::new()
                .add(Some(Item::Food(Food::CoffeeBeans)), Some(Item::Food(Food::Espresso))),
            Transform::from_position(vec2(0.0, 0.0)),
            Sprite::from_color(Color::BLACK, Vec2 { x: 100.0, y: 100.0 }),
            LockToGrid::new(grid::GridName::Interactables, (2, 0))
        )
    );

    commands.spawn(
        (
            Machine::from(MachineType::CashRegister),
            MachineState::Idle,
            MachineFunction::new()
                .add(None, None),
            Transform::from_position(vec2(0.0, 0.0)),
            Sprite::from_color(Color::BLUE, Vec2 { x: 100.0, y: 100.0 }),
            LockToGrid::new(grid::GridName::Interactables, (1, 3))
        )
    );

    commands.spawn(
        (
            Machine::from(MachineType::Fridge),
            MachineState::Idle,
            MachineFunction::new()
                .add(Some(Item::Food(Food::CoffeeBeans)), None)
                .add(Some(Item::Food(Food::Espresso)), None),
            Transform::from_position(vec2(0.0, 0.0)),
            Sprite::from_color(Color::YELLOW, Vec2 { x: 100.0, y: 100.0 }),
            LockToGrid::new(grid::GridName::Interactables, (3, 3))
        )
    );
}

fn post_setup(
    mut commands: Commands
)
{
    commands.trigger(SpawnRandomClient);
    commands.trigger(SpawnService(Service::First));
    
    commands.trigger(
        SetPath { 
            player: PlayerType::Service(Service::First), 
            from: (4, 4).into(), 
            to: (4, 2).into() 
        }
    );

    commands.trigger(
        Interact { 
            player: PlayerType::Service(Service::First), 
            machine: MachineType::CoffeeBag,
            item: None
        }
    );

    commands.trigger(
        SetPath { 
            player: PlayerType::Service(Service::First), 
            from: (4, 2).into(), 
            to: (2, 0).into() 
        }
    );

    commands.trigger(
        Interact { 
            player: PlayerType::Service(Service::First), 
            machine: MachineType::CoffeeMachine,
            item: Some(Item::Food(Food::CoffeeBeans))
        }
    );
}

fn cont(
    mut commands : Commands,
    input : &ButtonInput<KeyCode>
) {
    if input.just_released(KeyCode::KeyA) {
        commands.trigger(SpawnRandomEnemy);
    }
    if input.just_released(KeyCode::KeyW) {
        commands.trigger(MoveQueue);
    }
    if input.just_released(KeyCode::KeyD) {
        commands.trigger(SpawnRandomClient);
    }
    if input.just_released(KeyCode::Space) {
        commands.trigger(
            Interact { 
                player: PlayerType::Service(Service::First), 
                machine: MachineType::CoffeeMachine,
                item: None
            }
        );

        commands.trigger(
            SetPath { 
                player: PlayerType::Service(Service::First), 
                from: (2, 0).into(), 
                to: (3, 3).into()
            }
        );

        commands.trigger(
            Interact { 
                player: PlayerType::Service(Service::First), 
                machine: MachineType::Fridge,
                item: Some(Item::Food(Food::Espresso))
            }
        );

        commands.trigger(
            Interact { 
                player: PlayerType::Service(Service::First), 
                machine: MachineType::CashRegister,
                item: Some(Item::Food(Food::Espresso))
            }
        );
    }
}