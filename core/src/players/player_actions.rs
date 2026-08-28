use engine::ecs::{change_detection::params::Mut, entities::id::EntityId, worlds::World};
use engine_macros::Event;

use crate::{actions::action::{Action, IntoBoxedAction}, algos::path_finding::find_path, items::Item, machines::{events::{DisruptMachine, InteractWithMachine}, machine::{self, Machine}, machine_type::MachineType, state::MachineState}, players::{player_state::PlayerState, player_type::PlayerType}, utils::grid::{Grid, GridName, Point, SmoothToGrid}};



#[derive(Clone)] 
pub struct SetPath {
    pub from : Point,
    pub to : Point,
}

impl Action for SetPath {
    fn on_start(&mut self, agent : EntityId, world : &mut World) -> bool {
        if let Some(player_state) = world.get_component_mut::<PlayerState>(agent) {
            player_state.set_walking();
        }
        world.insert(agent, (SmoothToGrid::new(GridName::Players, self.from, self.to), ));
        false
    }

    fn on_end(&mut self, agent : EntityId, world : &mut World) {
        if let Some(player_state) = world.get_component_mut::<PlayerState>(agent) {
            player_state.set_idle();
        }
    }

    fn on_stop(&mut self, agent : EntityId, world :  &mut World) {
        if let Some(player_state) = world.get_component_mut::<PlayerState>(agent) {
            player_state.set_idle();
        }

        world.insert(agent, (SmoothToGrid::new(GridName::Players, self.from, self.from), ));
    }

    fn is_finished(&self, agent : EntityId, world : &World) -> bool {
        world
            .get_component::<SmoothToGrid>(agent)
            .map_or(false, |smooth| smooth.is_end())
    }

    fn name(&self) -> String {
        format!("Walk from {} to {}", self.from, self.to)
    }
}

#[derive(Clone)] 
pub struct Interact {
    pub machine: EntityId,
    pub item : Option<Item>
}


impl Action for Interact {
    fn on_start(&mut self, agent : EntityId, world : &mut World) -> bool {
        world.trigger_ref(&mut InteractWithMachine(agent, self.machine, self.item));
        let machine = world.get_component::<Machine>(self.machine).unwrap();

        let final_state = match machine.is_holding {
            true => PlayerState::Active,
            false => PlayerState::Idle,
        };

        let player_state = world.get_component_mut::<PlayerState>(agent).unwrap();
        *player_state = final_state;

       true
    }

    fn on_end(&mut self, agent : EntityId, world : &mut World) {
        if let Some(player_state) = world.get_component_mut::<PlayerState>(agent) {
            player_state.set_idle();
        }
    }   

    fn on_stop(&mut self, agent : EntityId, world :&mut World) {
        if let Some(player_state) = world.get_component_mut::<PlayerState>(agent) {
            player_state.set_idle();
        }
    }

    fn is_finished(&self, agent : EntityId, world : &World) -> bool {
        true
    }

    fn name(&self) -> String {
        format!("Interact {}, with {:#?}", self.machine, self.item)
    }
}

#[derive(Event, Clone)] 
pub struct Disrupt {
    pub machine: EntityId,
}

impl Action for Disrupt {
    fn on_start(&mut self, agent : EntityId, world : &mut World) -> bool {
        if let Some(player_state) = world.get_component_mut::<PlayerState>(agent) {
            player_state.set_idle();
        }

        world.trigger_ref(&mut DisruptMachine(agent, self.machine));
        false
    }

    fn on_end(&mut self, agent : EntityId, world : &mut World) {
        if let Some(player_state) = world.get_component_mut::<PlayerState>(agent) {
            player_state.set_idle();
        }
    }
    fn on_stop(&mut self, agent : EntityId, world :  &mut World) {
        if let Some(player_state) = world.get_component_mut::<PlayerState>(agent) {
            player_state.set_idle();
        }
    }

    fn is_finished(&self, agent : EntityId, world : &World) -> bool {
        true
    }
}



#[derive(Event, Clone)] 
pub struct WaitUntil {
    pub machine: EntityId,
}

impl Action for WaitUntil {
    fn on_start(&mut self, agent : EntityId, world : &mut World) -> bool {
        if let Some(player_state) = world.get_component_mut::<PlayerState>(agent) {
            player_state.set_waiting();
        }

        false
    }

    fn on_end(&mut self, agent : EntityId, world : &mut World) {
        if let Some(player_state) = world.get_component_mut::<PlayerState>(agent) {
            player_state.set_idle();
        }
    }

    fn on_stop(&mut self, agent : EntityId, world :  &mut World) {
        if let Some(player_state) = world.get_component_mut::<PlayerState>(agent) {
            player_state.set_idle();
        }
    }

    fn is_finished(&self, agent : EntityId, world : &World) -> bool {
        let Some(machine_state) = world.get_component::<MachineState>(self.machine) else {
            panic!("Machine doesn't exist")
        };

        machine_state.is(MachineState::Finished) || machine_state.is(MachineState::Idle) 
    }

    fn name(&self) -> String {
        format!("Wait for {}", self.machine)
    }
}



pub fn add_path(
    grid : &Grid, 
    queue : &mut Vec<Box<dyn IntoBoxedAction>>, 
    start : Point, 
    finish : Point) {
    let path = 
        find_path(grid, start, finish);

    queue.extend(
        path.windows(2)
        .map(|slice| {
            let a = slice[0];
            let b = slice[1];
            Box::new(SetPath { from : a, to : b}) as Box<dyn IntoBoxedAction>
        })
    );
}