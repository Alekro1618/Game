use engine::ecs::entities::id::EntityId;
use engine_macros::Event;

use crate::items::Item;

/// 1. [`EntityId`] of who is interacting
/// 2. [`MachineType`] of machine
/// 3. [`Option<Item>`] input item
#[derive(Event)]
pub struct Outputed(pub EntityId, pub EntityId, pub Option<Item>);

/// 1. [`EntityId`] of who is interacting
/// 2. [`MachineType`] of machine
/// 3. [`Option<Item>`] output item
#[derive(Event)]
pub struct Inputed(pub EntityId, pub EntityId, pub Option<Item>);

/// 1) [`EntityId`] of [`Player`]
/// 2) [`EntityId`] of [`Machine`]
#[derive(Event)]
pub struct InteractWithMachine(pub EntityId, pub EntityId, pub Option<Item>);

/// 1) [`EntityId`] of [`Player`]
/// 2) [`EntityId`] of [`Machine`]
#[derive(Event)]
pub struct DisruptMachine(pub EntityId, pub EntityId);

/// 1) [`EntityId`] of [`Machine`]
#[derive(Event)]
pub struct FinishedMachine(pub EntityId);

