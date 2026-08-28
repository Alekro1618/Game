use engine::{asset::AssetServer, ecs::{entities::id::EntityId, observer::system_param::On, query::filter::Without, systems::{commands::{self, Commands}, query::Query}}, math::{transform::Transform, vec::Vec2}, render::{color::Color, sprite::Sprite}};

use crate::{actions::components::{ActionQueue, CurrentAction}, bundles::PlayerBundle, items::Inventory, machines::{machine::WaitingFor, machine_type::MachineType}, order::order::CompleteOrder, players::{events::{Inputed, Spawned}, player::{ Player, PlayerSpeed}, player_actions::SetPath, player_state::PlayerState, player_type::{PlayerType, Service}}, queue, utils::grid::{Grid, GridName, Point, SmoothToGrid, SpecialKeys}};



pub fn on_service_spawned(
    on : On<Spawned<Service>>,
    mut commands: Commands,
    grid : &Grid,
    asset_server : &AssetServer
) {
    let spawned_service = on.event();
    let entity = spawned_service.0;
    let service = spawned_service.1;

    let position = grid.get_special(SpecialKeys::Entrence);
    let image = asset_server.load("res/images/sprites/x.png");
    
    commands.add(entity, 
        PlayerBundle {
            smooth_to_grid: SmoothToGrid::new(GridName::Players, position, position),
            player_speed: PlayerSpeed(2.0), 
            transform: Transform::IDENTITY, 
            sprite: Sprite::from_image(image ,Vec2 { x: 100.0, y: 100.0 }),
            ..Default::default() 
        });
}


pub fn on_machine_input(
    on : On<Inputed<Service>>,
    mut commands: Commands,
    query : Query<&MachineType>

) {
    let inputed = on.event();
    let Inputed::<Service>(_, _, machine_id, item) = inputed;

    let Ok(machine_type) = query.get_mut(*machine_id) else {
        panic!("");
    };
    
    if matches!(machine_type, MachineType::CashRegister) {
        if let Some(item) = item {
            commands.trigger(CompleteOrder(*item));
        }
    }
}