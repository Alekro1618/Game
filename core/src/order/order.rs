use std::collections::HashMap;

use engine::ecs::entities::{id::EntityId};
use engine_macros::{Event, Resource};

use crate::items::Item;


#[derive(Resource, Default)]
pub struct Orders {
    pub current : HashMap<EntityId, Order>
}

impl Orders {
    pub fn add(&mut self, enity : EntityId, order: Order) {
        self.current.insert(enity, order);
    }

    pub fn remove(&mut self, entity : EntityId)  {
        self.current.remove(&entity);
    }
}

#[derive(Clone, Copy)]
pub struct Order {
    pub item : Item,
    pub wait_time : f32,
    pub taken : bool
}

impl Order {
    pub fn new(item :Item, wait_time : f32) -> Self {
        Self { item : item, wait_time, taken: false }
    }

    pub fn take(&mut self) {
        self.taken = true;
    }
}

#[derive(Event)]
pub struct NewOrder(pub EntityId, pub Order);

#[derive(Event, Clone, Copy)]
pub struct CompletedOrder(pub EntityId, pub Order);

#[derive(Event, Clone, Copy)]
pub struct FailedOrder(pub EntityId, pub Order);