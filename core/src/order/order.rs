use std::collections::{HashMap, VecDeque};

use engine::ecs::entities::{id::EntityId};
use engine_macros::{Event, Resource};

use crate::items::Item;


#[derive(Resource, Default)]
pub struct Orders {
    pub current : HashMap<EntityId, (Order, usize)>,
}

impl Orders {
    pub fn add(&mut self, entity : EntityId, order: Order) {
        let len = self.current.len();
        self.current.insert(entity, (order, len));
    }

    pub fn remove(&mut self, entity : EntityId) -> Option<Order> {
        self
            .current
            .remove(&entity)
            .map(|(order, _)| order)
    }

    pub fn get_first_match(&self, item : Item) -> Option<EntityId> {
        self
            .current
            .iter()
            .filter(|(_, (order, _))| 
                order.item == item
            )
            .map(|(entity, (_, i))|
                (entity, i)
            )
            .min_by(|(_, a), (_, b)|
                a.cmp(b)
            )
            .map(|(entity, _)|
                *entity
            )
    }
}

#[derive(Clone, Copy)]
pub struct Order {
    pub item : Item,
    pub wait_time : f32,
}

impl Order {
    pub fn new(item : Item, wait_time : f32) -> Self {
        Self { item : item, wait_time }
    }
}

#[derive(Event)]
pub struct NewRandomOrder(pub EntityId);

#[derive(Event)]
pub struct NewOrder(pub EntityId, pub Order);

#[derive(Event, Clone, Copy)]
pub struct CompleteOrder(pub Item);

#[derive(Event, Clone, Copy)]
pub struct CompletedOrder(pub EntityId, pub Order);

#[derive(Event, Clone, Copy)]
pub struct FailedOrder(pub EntityId, pub Order);