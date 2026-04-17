use std::rc::Rc;

use Game::component;
use crate::{core::objects::player::ClientState, engine::{components::component::Component, id::ObjectId, objects::object::{Object, ObjectActions}, prefabs::manager::ObjectManager, systems::system::System}};

#[component(name = "client_manager")]
#[derive(Clone)]
pub struct ClientManager {
    clients : Vec<ObjectId>,
    queue : Vec<(usize, usize)>,
    current_clients : Vec<ObjectId>,
    current_amount : usize,
    end_of_day : bool,
    waits_for_order : bool,
    max_amount : usize,
    timer : f32,
}

impl ClientManager {
    pub fn new() -> Self {
        Self {
            clients: Vec::new(),
            queue : Vec::new(),
            current_clients : Vec::new(),
            current_amount : 0,
            end_of_day : false,
            waits_for_order : false,
            max_amount: 1,
            timer : 0.0
        }
    }
}

impl Component for ClientManager {
    fn on_start(&mut self, object : &mut Object, other : &mut ObjectManager, system : &mut System) {
        self.queue = vec![
            (2, 2), (1, 2), (0, 2)
        ];

        self.clients = other
            .find_all_by_tag("client")
            .iter()
            .map(|go| go.obj().id())
            .collect();
    }

    fn update(&mut self, object : &mut Object, other : &mut ObjectManager, system : &mut System, delta : f32) {
        if self.timer < 5.0 {
            self.timer += delta;
            return;
        }
        
        self.timer = 0.0;

        for i in 0..self.max_amount {
            if let Some(id) = self.clients.get(i) {
                system.params.grid.set_special(&format!("queue_{}", i), self.queue[i]);
            } else {
                system.params.grid.remove_special(&format!("queue_{}", i));
            }
        }

        if self.current_amount < self.max_amount {
            let it = rand::random_range(..self.clients.len());
            let id = self.clients[it];
            let client = other.get_mut(id).unwrap();
            client.obj_mut().set_state_t(ClientState::Start);
        }
    }

}
