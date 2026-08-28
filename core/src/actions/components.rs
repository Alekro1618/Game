use std::collections::VecDeque;

use engine_macros::Component;

use crate::actions::action::{Action, IntoBoxedAction};


#[derive(Component, Default)]
pub struct CurrentAction(pub Option<Box<dyn Action>>);

impl CurrentAction {
    pub fn new() -> Self {
        Self(None)
    }
}

#[derive(Component, Default)]
pub struct ActionQueue(pub VecDeque<Box<dyn Action>>);

impl ActionQueue {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn add(mut self, action : impl IntoBoxedAction) -> Self {
        self.0.push_back(action.into_boxed());
        self
    }

    pub fn push(&mut self, action : Box<dyn Action>) {
        self.0.push_back(action);
    }

    pub fn pop(&mut self) -> Option<Box<dyn Action>> {
        self.0.pop_front()
    }
}

