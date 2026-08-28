use std::{collections::HashMap, error::Error, fmt::Display};

use engine::{ecs::entities::id::EntityId};
use engine_macros::{Component, Event};
use crate::{items::Item, machines::{functions::MachinePrepearedResult::NotPrepeared, machine_type::MachineType}};

#[derive(Default)]
pub enum MachinePrepearedResult {
    Prepeared(Option<Item>),
    #[default]
    NotPrepeared
}

///Component for `Machine` that describes what machine takes and what it may output. Output is wrapped in option to configure 2 results
/// - If items match with input then it will set last input to [`Option<Item>`]
/// - When we call [`get_ready_output()`] it will take the last input, match it and return  [`Result<Option<Item>>`]). Returns error if the item wasn't prepeared or there is no match, which is specified by [`MachineError`]
#[derive(Component, Default)]
pub struct MachineFunction
{
    pub map : HashMap<Option<Item>, Option<Item>>,
    pub prepared : MachinePrepearedResult
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

    pub fn get_output(&self, item : Option<Item>) -> Result<Option<Item>, MachineError> {
        match self.map.get(&item) {
            Some(item) => Ok(item.clone()),
            None => Err(MachineError::NoMatch),
        }
    }

    pub fn prepeare_for(&mut self, item : Option<Item>) -> Result<(), MachineError> {
        self.prepared = MachinePrepearedResult::Prepeared(self.get_output(item)?);
        Ok(())
    }

    pub fn clear(&mut self) {
        self.prepared = MachinePrepearedResult::NotPrepeared
    }

    pub fn get_prepeared(&mut self) -> Result<Option<Item>, MachineError> {
        match self.prepared {
            MachinePrepearedResult::Prepeared(item) => Ok(item),
            MachinePrepearedResult::NotPrepeared => Err(MachineError::NotPrepeared),
        }
    }

    pub fn new() -> Self {
        Self {
            map : HashMap::new(),
            prepared : MachinePrepearedResult::NotPrepeared
        }
    }
}

