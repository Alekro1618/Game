use std::collections::VecDeque;

use engine::{ecs::{entities::id::EntityId, observer::system_param::On, systems::{commands::Commands, query::Query}}, input::{button_input::ButtonInput, keyboard::KeyCode}, utils::{resources::GameSpeed, timer::Timer}};
use engine_macros::{Component, Event};

use crate::{machines::machine::InteractWithMachine, utils::grid::{self, Point, SmoothToGrid}};



#[derive(Component)]
pub struct Player {
    actions: VecDeque<PlayerAction>,
    pub current_action: Option<PlayerAction>,
    pub point : Point,
}

impl Player {
    pub fn new(point : impl Into<Point>) -> Self {
        Self { 
            actions: VecDeque::new(),
            current_action: None,
            point : point.into()
        }
    }

    pub fn from(point : impl Into<Point>, actions : impl Into<VecDeque<PlayerAction>>) -> Self {
        let actions: VecDeque<PlayerAction>  = actions.into();
        Self {
            actions,
            current_action: None,
            point: point.into()
        }
    }

    pub fn is_next_action_interact(&self) -> bool {
        self.actions.front().map(|a| match a {
            PlayerAction::Interact(_) => true,
            _ => false,
        }).unwrap_or(false)
    }

    pub fn interact(&mut self, entity : EntityId) {
        self.add_action(PlayerAction::Interact(entity));
    }

    pub fn add_path(&mut self, path : Vec<Point>) {
        path.into_iter().for_each(|p| self.add_action(PlayerAction::WalkTo(p)));
    }

    pub fn add_action(&mut self, action : PlayerAction) {
        self.actions.push_back(action);
    }

    pub fn view_next_action(&self) -> Option<&PlayerAction>{
        self.actions.front()
    }

    pub fn next_action(&mut self){
        self.current_action = self.actions.pop_front();
    }
}

// Component for `Player`. If `Player` contains it then it waits for the next action
#[derive(Event)]
pub struct PendingAction(pub EntityId);


#[derive(Event)] 
pub struct SetPath {
    pub entity : EntityId,
    pub from: Point,
    pub to: Point
}


#[derive(Event)] 
pub struct Interact {
    pub entity: EntityId,
    pub machine: EntityId
}


/// Different actions of `Player`
#[derive(Clone, Copy)]
pub enum PlayerAction {
    WalkTo(Point),
    Interact(EntityId),
    Direction(Point)
}