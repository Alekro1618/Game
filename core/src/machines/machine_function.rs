use std::{collections::HashMap, error::Error, fmt::Display};

use engine::{ecs::entities::id::EntityId};
use engine_macros::{Component, Event};
use crate::{items::Item, machines::machine_type::MachineType, players::player_type::PlayerType};




///Component for `Machine` that describes what machine takes and what it may output. Output is wrapped in option to configure 2 results
/// - If items match with input then it will set last input to [`Option<Item>`]
/// - When we call [`get_ready_output()`] it will take the last input, match it and return  [`Result<Option<Item>>`]). Returns error if the item wasn't prepeared or there is no match, which is specified by [`MachineError`]
#[derive(Component)]
pub struct MachineFunction
{
    pub map : HashMap<Option<Item>, Option<Item>>,
    pub last_input : Option<Option<Item>>
}

impl Error for MachineError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl Display for MachineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MachineError::NoMatch =>  write!(f, "Machine doesn't have output for this item"),
            MachineError::NotPrepeared =>  write!(f, "Machine wasn't even prepeared to give output"),
        }
       
    }
}

#[derive(Debug)]
pub enum MachineError {
    NoMatch,
    NotPrepeared
}


impl MachineFunction {
    pub fn add(mut self, input : Option<Item>, output : Option<Item>) -> Self {
        self.map.insert(input, output);
        self
    }

    pub fn input(&mut self, item : Option<Item>) -> bool {
        if self.map.contains_key(&item) {
            self.last_input = Some(item);
            true
        } else {
           false
        }
    }

    pub fn get_ready_output(&self) -> Result<Option<Item>, MachineError> {
        match self.last_input {
            Some(last_input) => {
                match self.map.get(&last_input) {
                    Some(item) => Ok(item.clone()),
                    None => Err(MachineError::NoMatch),
                }
            }
            None => Err(MachineError::NotPrepeared)
        }
    }

    pub fn new() -> Self {
        Self {
            map : HashMap::new(),
            last_input: None
        }
    }
}

/// 1. [`EntityId`] of who is interacting
/// 2. [`EntityId`] of machine
/// 3. [`Option<Item>`] input item
#[derive(Event)]
pub struct Output(pub PlayerType, pub MachineType, pub Option<Item>);


/// 1. [`EntityId`] of who is interacting
/// 2. [`EntityId`] of machine
/// 3. [`Option<Item>`] output item
#[derive(Event)]
pub struct Input(pub PlayerType, pub MachineType, pub Option<Item>);