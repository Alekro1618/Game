mod splash_screen;

use std::f32;

use engine::{asset::{AssetServer, handle::Handle}, camera::{Camera, SetMainCamera}, ecs::systems::commands::Commands, math::{rect::Rect, transform::Transform, vec::{Vec2, vec2}}, render::{color::Color, sprite::{Sprite, SpriteImageMode}}, scene::{dynamic::DynamicScene, scene::Scene}};
use engine_macros::Resource;

use crate::{client::queue::Queue, items::{Item, food::Food}, machines::{functions::MachineFunction, machine::Machine, machine_type::MachineType, state::MachineState}, utils::grid::{self, Grid, GridParams, SpecialKeys}};


pub enum Phase {
    Idle,
    In,
    Out
}

#[derive(Resource)]
pub struct SceneTransition {
    phase : Phase,
    timer : f32,
    target : Option<Handle<DynamicScene>>
}

pub fn spawn_cafe_scene(
    mut commands : Commands,
    asset_server : &AssetServer,
) {
    commands = commands
        .spawn_with_id(Camera::new(Rect::new(vec2(0., 0.), vec2(800., 800.))))
        .then(|mut commands, entity| {
            commands.trigger(SetMainCamera(entity));
        });

    
    let mut grid = Grid::new(
        20,
        20,
        GridParams::new(
            vec2(0.0_f32, 0.0_f32),
            100., 
            100., 
            f32::consts::PI / 3.0, 
            vec2(400.0_f32, 0.0_f32),
            400.
        )
    );

    grid.create_map(grid::GridName::Interactables);
    grid.create_map(grid::GridName::Players);
    grid.create_map(grid::GridName::Walls);

    grid.add_special(SpecialKeys::CashRegister, (1, 3));
    grid.add_special(SpecialKeys::Entrence, (0, 0));
    grid.add_special(SpecialKeys::Exit, (3, 7));
    grid.add_special(SpecialKeys::ServicePlace, (4, 4));

    commands.insert(grid);

    commands.spawn(
        Queue::new(
            vec![
                (2, 2).into(),
                (2, 1).into(),
                (1, 1).into(),
                (0, 1).into()
            ]
        )
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

    let tilemap = asset_server.load("res/images/sprites/tilemap.png");

    commands.spawn(
        (
            Sprite {
                texture: tilemap,
                color: Color::WHITE,
                flip_x: false,
                flip_y: false,
                custom_size: Some(vec2(800., 100.)),
                rect: None,
                image_mode: SpriteImageMode::Tiled { tile_x: true, tile_y: true, stretch_value: 1.0 },
            },
            Transform {
                position: vec2(0_f32 ,0_f32),
                rotation: 0.0,
                scale: vec2(1_f32, 1_f32),
            },
            
        )
    );
}