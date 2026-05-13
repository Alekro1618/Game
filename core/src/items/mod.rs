use std::{collections::HashMap, process::Command};

use engine::ecs::{entities::id::EntityId, message::{Message, message_reader::MessageReader}, systems::commands::Commands};
use engine_macros::{Component, Event, Message};

use crate::items::food::Food;

pub mod food;
mod system;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Item {
    Food(Food),
    Money(u32),
    None
}

#[derive(Event)]
pub struct AddToInventory(pub EntityId, pub Item);

#[derive(Event)]
pub struct RemoveFromInventory(pub EntityId, pub Item);

#[derive(Component, Default)]
pub struct Inventory {
    items : HashMap<Item, u8>
}

#[derive(Component, Default)]
pub struct TempStorage {
    item : Option<Item>
}

impl TempStorage {
    #[inline]
    pub fn new() -> Self {
        Self { item: None }
    }

    #[inline]
    pub fn with(item : Item) -> Self {
        Self { item: Some(item) }
    }
}

impl Inventory {
    #[inline]
    pub fn new() -> Self {
        Self { items: HashMap::new() }
    }

    pub fn add(&mut self, item : &Item) {
        if let Some(amount) = self.items.get_mut(item) {
            if *amount < u8::MAX {
                *amount += 1u8;
            } else {
                panic!("Error while trying to add item {:#?}. Max amount of items is {}.", item, u8::MAX);
            }
        }
    }

    #[inline]
    pub fn amount(&self, item : &Item) -> u8 {
        self.items[&item]
    }

    pub fn remove(&mut self, item : &Item) {
        if let Some(amount) = self.items.get_mut(item) {
            if *amount > 0u8 {
                *amount -= 1u8;
            } else {
                panic!("Error while trying to take item {:#?}. There is no such item.", item);
            }
        }
    }

    #[inline]
    pub fn contains(&self, item : &Item) -> bool {
        self.amount(item) > 0u8
    }

    pub fn move_to(&mut self, to : &mut Self, item : &Item) {
        self.remove(item);
        to.add(item);
    }
}


