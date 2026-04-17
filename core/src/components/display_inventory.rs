use std::{collections::HashMap, rc::Rc};

use Game::component;
use crate::{core::{structures::food::Food}, engine::{components::component::Component, objects::object::{Object, ObjectActions}, prefabs::manager::ObjectManager, systems::{params::ParamsActions, system::System}}};

#[component(name = "display_inventory")]
#[derive(Clone)]
pub struct DisplayInventory {
    inventory : HashMap<String, Food>,
    arr : [u8; 3]
}

impl DisplayInventory {
    pub fn new() -> Self {
        Self {
            inventory : HashMap::new(),
            arr: [2, 3, 4]
        }
    }
}

impl Component for DisplayInventory {
    fn update(&mut self, object : &mut Object, other : &mut ObjectManager, system : &mut System, delta : f32) {
        let d= self.inventory.get(&String::new());
        let b = self.arr[2];
    }
}

impl DisplayInventory {
    fn check_for_changes() {

    }
}
