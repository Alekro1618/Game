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

