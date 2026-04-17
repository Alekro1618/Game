use Game::component;
use raylib::{ffi::{MouseButton}, math::{Vector2}};
use crate::{core::{algos::path_finding::path, objects::player::ClientState, structures::clients::Client}, engine::{components::component::Component, input::manager::InputManager, objects::object::{Object, ObjectActions}, prefabs::manager::ObjectManager, systems::system::System}};

#[component(name = "client_controller")]
#[derive(Clone)]
pub struct ClientController {
    start : (usize, usize),
    exit : (usize, usize),
    order : usize,
    path : Vec<(usize, usize)>,
    time : f32
}

impl ClientController {
    pub fn new() -> Self {
        Self {
            order : 0,
            exit : (0, 0),
            start : (0, 0),
            path : Vec::new(),
            time : 0.0
        }
    }
}

impl Component for ClientController {
    fn on_start(&mut self, object : &mut Object, other : &mut ObjectManager, system : &mut System) {
        self.start = system.params.grid.what_cell(object.pos());
    }

    fn update(&mut self, object : &mut Object, other : &mut ObjectManager, system : &mut System, delta : f32) {
        match ClientState::from_usize(object.state()).unwrap() {
            ClientState::Start => {
                
            },
            ClientState::Walking => {

            },
            ClientState::Proceed => {

            },
            ClientState::End => {
    
            },
            _ => {}
        }

        if object.state() == ClientState::Start {
            object.set_state_t(ClientState::Walking);
            while system.params.grid.get_special(&format!("queue_{}", self.order)) != None {
                self.order += 1;
            }
            let target = system.params.grid.get_special(&format!("queue_{}", self.order)).unwrap();
            self.path = path(&system.params.grid, self.start, *target);
        } else if object.state() == ClientState::Proceed {
            object.set_state_t(ClientState::Walking);
            self.order -= 1;
            let target = system.params.grid.get_special(&format!("queue_{}", self.order)).unwrap();
            self.path = path(&system.params.grid, self.start, *target);
        } else if object.state() == ClientState::End {
            object.set_state_t(ClientState::Walking);
            self.path = path(&system.params.grid, self.start,self.exit);
        }
        
        let next_node = if self.time < 1.0 {
            self.time += delta * 5.0;
            self.path.first()
        } else {
            self.time = 0.0;         
            if self.path.len() > 0 {
                let point = self.path.get(0).unwrap().clone();
                system.params.grid.set_special(object.name(), point);
                self.start = point;
                self.path.remove(0);
            }
            self.path.first()
        };

        if let Some(node) = next_node {
            if node.0 > self.start.0 {
                object.set_state_t(ClientState::Left);
            } else if node.0 < self.start.0  {
                object.set_state_t(ClientState::Right);
            } else if node.1 < self.start.1 {
                object.set_state_t(ClientState::Up);
            } else if node.1 > self.start.1 {
                object.set_state_t(ClientState::Down);
            }
        
            object.set_pos_v(system.params.grid.translate(&self.start, node, self.time));
        } 
    }
}


