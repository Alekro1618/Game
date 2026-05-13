use std::fmt::Display;

use crate::ecs::{bundle::Bundle, components::Component, entities::{Entity, id::EntityId}, worlds::World};

pub fn print_component_of<T : Component + Display>(world : &World, entity : EntityId) {
    if let Some(component) = world.get_component::<T>(entity) {
        println!("{}:{}", entity, component);
    }
}
