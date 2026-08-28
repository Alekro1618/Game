use std::collections::HashMap;

use engine_macros::Resource;

use crate::{machines::machine::Machine, players::{ player_type::{Enemy, PlayerType, Service}}, utils::grid::Point};

///Controlls the service commands. Specifically stores future of each service action to use it later
#[derive(Resource)]
pub struct EnemyController {
    future_point : HashMap<Enemy, Point>,
    future_machine : HashMap<Enemy, Machine>
}

impl EnemyController {
    
}