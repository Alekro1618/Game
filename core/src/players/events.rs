use engine::ecs::entities::id::EntityId;
use engine_macros::Event;

use crate::{items::Item, machines::machine_type::MachineType, players::{player::Player, player_type::{Client, Enemy, Service}}, utils::grid::Point};


#[derive(Event)]
pub struct GoToPoint(pub Client);


#[derive(Event)]
pub struct Spawn<M : Sync + Send + 'static>(pub M, pub Point);

#[derive(Event)]
pub struct Spawned<M : Sync + Send + 'static>(pub EntityId, pub M);


#[derive(Event)]
pub struct Despawn<M : Sync + Send + 'static>(pub M);

#[derive(Event)]
pub struct Despawned<M : Sync + Send + 'static>(pub EntityId, pub M);


#[derive(Event)]
pub struct Inputed<M : Sync + Send + 'static>(pub EntityId, pub M, pub EntityId, pub Option<Item>);

#[derive(Event)]
pub struct Outputed<M : Sync + Send + 'static>(pub EntityId, pub M, pub EntityId, pub Option<Item>);



