use std::collections::VecDeque;

use engine::{ecs::{entities::id::EntityId, observer::system_param::On, systems::{commands::Commands, query::Query}}, input::{button_input::ButtonInput, keyboard::KeyCode}, utils::*};
use engine_macros::{Component, Event, Message};

use crate::{items::Item, machines::machine_type::MachineType, players::player_type::PlayerType, utils::grid::{self, Point, SmoothToGrid}};


#[derive(Component)]
pub struct PlayerSpeed(pub f32);

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

    pub fn from(point : impl Into<Point>, actions : impl Into<VecDeque<PlayerAction>>, speed : f32) -> Self {
        let actions: VecDeque<PlayerAction>  = actions.into();
        Self {
            actions,
            current_action: None,
            point: point.into(),
        }
    }

    pub fn is_next_action_interact(&self) -> bool {
        self.actions.front().map(|a| match a {
            PlayerAction::Interact(_, _) => true,
            _ => false,
        }).unwrap_or(false)
    }

    pub fn disrupt(&mut self, entity : EntityId) {
        self.add_action(PlayerAction::Disrupt(entity));
    }

    pub fn interact(&mut self, entity : EntityId, item : Option<Item>) {
        self.add_action(PlayerAction::Interact(entity, item));
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

/// When called will proceed with next action.
#[derive(Event)]
pub struct PendingAction(pub EntityId);

/// Called on end of the step. Used to decide next action.
#[derive(Event)]
pub struct StepEnd(pub EntityId);


#[derive(Event, Clone)] 
pub struct SetPath {
    pub player : PlayerType,
    pub from : Point,
    pub to : Point
}

#[derive(Event, Clone)] 
pub struct Interact {
    pub player: PlayerType,
    pub machine: MachineType,
    pub item : Option<Item>
}

#[derive(Event, Clone)] 
pub struct Disrupt {
    pub player: PlayerType,
    pub machine: MachineType,
}

#[derive(Message)]
pub enum Command {
    SetPath(PlayerType, Point),
    Interact(PlayerType, MachineType),
}


/// Different actions of `Player`
#[derive(Clone, Copy)]
pub enum PlayerAction {
    WalkTo(Point),
    Interact(EntityId, Option<Item>),
    Disrupt(EntityId),
    Direction(Point)
}