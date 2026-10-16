use std::{collections::HashMap, fmt::Display, process::Command};

use engine::{app::plugin::Plugin, ecs::{entities::id::EntityId, message::{Message, message_reader::MessageReader}, observer::system_param::On, systems::{commands::Commands, query::Query}}};
use engine_macros::{Component, Event, Message};
use serde::{Deserialize, Serialize};

use crate::items::{food::Food, system::{on_add_to_inventory, on_remove_from_inventory}};

pub mod food;
mod system;


pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(self, app:  &mut engine::app::App) {
        app
            .add_observer(
                on_add_to_inventory
            )
            .add_observer(
                on_remove_from_inventory
            );
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Serialize, Deserialize)]
pub enum Item {
    Food(Food),
    Money(u32)
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Food(food) => write!(f, "{}", food),
            Item::Money(amount) => write!(f, "{}$", amount),
        }
    }
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


