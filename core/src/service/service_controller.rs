use std::collections::HashMap;

use engine_macros::Resource;

use crate::{machines::machine::Machine, players::{ player_type::{PlayerType, Service}}, utils::grid::Point};

///Controlls the service commands. Specifically stores future of each service action to use it later
#[derive(Resource, Default)]
pub struct ServiceController {
    future_point : HashMap<Service, Point>,
    future_machine : HashMap<Service, Machine>
}

impl ServiceController {
    
}