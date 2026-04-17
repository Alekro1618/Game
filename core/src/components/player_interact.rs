use std::{collections::HashMap};

use Game::component;
use crate::{core::{objects::{blocks::{CoffeeMachineState, DisplayState, RubbishCanState}, player::PlayerState}, structures::{food::Food, machine::Machine}}, engine::{components::component::Component, id::ObjectId, objects::object::{Object, ObjectActions}, prefabs::manager::ObjectManager, systems::{params::ParamsActions, system::{self, System}}, utils::grid::Grid}};

#[component(name = "interact")]
#[derive(Clone)]
pub struct PlayerInteract {
    machines : HashMap<Machine, (usize, usize)>,
    machines_id : HashMap<Machine, ObjectId>,
    busy : bool,
    inventory : Option<Food>,
    timer : f32,
}

impl PlayerInteract {
    pub fn new() -> Self {
        Self {
            inventory: None,
            machines: HashMap::new(),
            machines_id: HashMap::new(),
            busy : false,
            timer : 0.0
        }
    }
}

impl Component for PlayerInteract {
    fn on_start(&mut self, object : &mut Object, other : &mut ObjectManager, system : &mut System) {
        self.machines = other.find_all_by_tag("machine")
            .iter()
            .map(|m| { 
                let pos = m.obj().pos();
                let name = m.obj().name();
                (Machine::from(name), system.params.grid.what_cell(pos))
            })
            .collect();

        self.machines_id = other.find_all_by_tag("machine")
            .iter()
            .map(|m| {
                let name = m.obj().name();
                (Machine::from(name), m.obj().id())
            })
            .collect();
    }
    
    fn update(&mut self, object : &mut Object, other : &mut ObjectManager, system : &mut System, delta : f32) {
        if object.state() != PlayerState::Stop {
            return;
        }

        let mut machine_int = None;
        for (machine, cell) in self.machines.iter() {
            if let Some(obj_cell) = system.params.grid.get_special(object.name()) {
                if obj_cell == cell {
                    machine_int = Some(machine.clone());
                }
            }
        }

        if let Some(machine) = machine_int {
            self.interact_with(&machine, other, system, delta);
        }

        if let Some(food) = &self.inventory {
            system.set_str(object.name().to_string() + "_inv",  food.to());
        }
        
    }
}

impl PlayerInteract {
    fn interact_with(&mut self, machine : &Machine, other : &mut ObjectManager, system : &mut System, delta : f32) {
        let id = self.machines_id[machine];
        match machine {
            Machine::CoffeeBag => {
                self.coffee_bag(id, other, system, delta);
            },
            Machine::CoffeMachine => {
                self.coffee_machine(id, other, system, delta);
            },
            Machine::RubbishCan => {
                self.rubbish_can(id, other, system, delta);
            },
            Machine::Display => {
                self.display(id, other, system, delta);
            },
            Machine::Cutter => {
                self.cutter(id, other, system, delta);
            }

        }
    }

    fn coffee_bag(&mut self, id : ObjectId, other : &mut ObjectManager, system : &mut System, delta : f32) {
        if self.inventory.is_none() {
            self.inventory = Some(Food::CoffeeBeans);
        }
    }

    fn coffee_machine(&mut self, id : ObjectId, other : &mut ObjectManager, system : &mut System, delta : f32) {
        let coffee_machine = other.get_mut(id).unwrap();
        match CoffeeMachineState::from_usize(coffee_machine.obj().state()).unwrap() {
            CoffeeMachineState::Idle => {
                if self.inventory != Some(Food::CoffeeBeans) {
                    println!("you don't have coffee beans");
                    return;
                }
                println!("disposed of coffee beans");
                self.inventory = None;
                coffee_machine
                    .obj_mut()
                    .set_state_t(CoffeeMachineState::Making);
            },
            CoffeeMachineState::Ready => {
                if self.inventory != None {
                    println!("your inventory is full");
                    return;
                }
                println!("collected coffee!");
                self.inventory = Some(Food::Espresso);
                coffee_machine
                    .obj_mut()
                    .set_state_t(CoffeeMachineState::Idle);
            }
            _ => {
                if self.inventory != Some(Food::CoffeeBeans) {
                    println!("you don't have coffee beans");
                    return;
                }
                println!("disposed of coffee beans");
                self.inventory = None;
                coffee_machine
                    .obj_mut()
                    .set_state_t(CoffeeMachineState::Making);
            }
        }
    }

    fn rubbish_can(&mut self, id : ObjectId, other : &mut ObjectManager, system : &mut System, delta : f32) {
        let rubbish_can = other.get_mut(id).unwrap();
        if self.inventory.is_none() {
            println!("nothing to throw away");
        } else {
            println!("throw away: {:?}", self.inventory);
            self.inventory = None;
        }
        
        rubbish_can
            .obj_mut()
            .set_state_t(RubbishCanState::Throw);
    }

    fn display(&mut self, id : ObjectId, other : &mut ObjectManager, system : &mut System, delta : f32) {
        let display = other.get_mut(id).unwrap();
        match &self.inventory {
            Some(food) => {
                if !food.is_display() {
                    println!("{:?} can't be put for display", food);
                    return;
                }
                let name = food.to();
                let count = system
                    .get_int(name)
                    .unwrap_or(&0);

                system
                    .set_int(name.to_string(), count + 1 );

                display
                    .obj_mut()
                    .set_state_t(DisplayState::Insert);
                println!("display: {:?}", self.inventory);
            },
            None => {
                println!("nothing to put to display")
            },
        }
    }

    fn cutter(&mut self, id : ObjectId, other : &mut ObjectManager, system : &mut System, delta : f32) {
        let cutter = other.get_mut(id).unwrap();

        let d = cutter.to_game_obj_ref();  
    }
}
