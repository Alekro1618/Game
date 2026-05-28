use engine::ecs::entities::id::EntityId;
use engine_macros::Event;

use crate::{players::{player::Player, player_type::{Client, Enemy, Service}}, utils::grid::Point};

#[derive(Event)]
pub struct SpawnClient(pub Client);

#[derive(Event)]
pub struct SpawnService(pub Service);

#[derive(Event)]
pub struct SpawnEnemy(pub Enemy);

#[derive(Event)]
pub struct SpawnedClient(pub EntityId, pub Client);

#[derive(Event)]
pub struct SpawnedService(pub EntityId, pub Service);

#[derive(Event)]
pub struct SpawnedEnemy(pub EntityId, pub Enemy);


#[derive(Event)]
pub struct DespawnClient(pub Client);

#[derive(Event)]
pub struct DespawnService(pub Service);

#[derive(Event)]
pub struct DespawnEnemy(pub Enemy);

#[derive(Event)]
pub struct DespawnedClient(pub EntityId, pub Client);

#[derive(Event)]
pub struct DespawnedService(pub EntityId, pub Service);

#[derive(Event)]
pub struct DespawnedEnemy(pub EntityId, pub Enemy);


