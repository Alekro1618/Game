use std::collections::VecDeque;

use engine::{ecs::{entities::id::EntityId, observer::system_param::On, systems::{commands::Commands, query::Query}}, input::{button_input::ButtonInput, keyboard::KeyCode}, utils::*};
use engine_macros::{Component, Event, Message};

use crate::{items::Item, machines::machine_type::MachineType, players::player_type::PlayerType, utils::grid::{self, Point, SmoothToGrid}};


#[derive(Component, Default)]
pub struct PlayerSpeed(pub f32);

#[derive(Component, Default)]
pub struct Player;


