use std::collections::VecDeque;

use engine_macros::{Component, Reflect, reflect};
use serde::{Deserialize, Serialize};

use crate::actions::action::{Action, IntoBoxedAction};


#[derive(Component, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct CurrentAction {
    #[serde(skip)] 
    pub action : Option<Box<dyn Action>>
}

impl Clone for CurrentAction {
    fn clone(&self) -> Self {
        Self::new()
    }
}


impl CurrentAction {
    pub fn new() -> Self {
        Self {
            action : None
        }
    }
}

#[derive(Component, Default, Deserialize, Serialize, Reflect)]
#[reflect(Component, Serialize, Deserialize)]
pub struct ActionQueue {
    #[serde(skip)] 
    pub actions :  VecDeque<Box<dyn Action>>
}

impl Clone for ActionQueue {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl ActionQueue {
    pub fn new() -> Self {
        Self {
            actions : VecDeque::new()
        }
    }

    pub fn add(mut self, action : impl IntoBoxedAction) -> Self {
        self.actions.push_back(action.into_boxed());
        self
    }

    pub fn push(&mut self, action : Box<dyn Action>) {
        self.actions.push_back(action);
    }

    pub fn pop(&mut self) -> Option<Box<dyn Action>> {
        self.actions.pop_front()
    }
}

