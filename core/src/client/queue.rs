use std::collections::{HashMap, VecDeque};

use engine::ecs::entities::id::EntityId;
use engine_macros::{Component, Event, Resource};

use crate::{players::player_type::Client, utils::grid::Point};


#[derive(Event)]
pub struct MoveQueue(pub EntityId);

#[derive(Event)]
pub struct SpawnRandomClient;

#[derive(Component)]
pub struct Queue {
    order : Vec<Point>,
    map : HashMap<Client, usize>,
    len : usize,
}

#[derive(Component)]
pub struct InQueue(pub EntityId);


impl Queue {
    pub fn new(order : Vec<Point>) -> Self {
        Self { order, map: HashMap::new(), len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn add(&mut self, client : Client) {
        self.map.insert(client, self.len);
        self.len += 1;
    }

    pub fn get_free(&mut self) -> Option<Point> {
        self.order.get(self.len).map(|p| *p)
    }

    pub fn remove(&mut self, client : &Client) {
        self.map.remove(client);
        self.len -= 1;
    }

    pub fn clients(&self) -> Vec<Client> {
        self.map.keys().map(|k| *k).collect()
    }

    pub fn move_entity(&mut self, client : &Client) -> (Option<Point>, bool) {
        let d = self.map.get_mut(&client).unwrap();
        if *d == 0 {
            self.remove(client);
            (None, false)
        } else {
            *d -= 1;
            (Some(*self.order.get(*d).unwrap()), *d == 0)
        }
    }

    pub fn match_point(&self, client : &Client) -> Option<Point> {
        if let Some(index) = self.map.get(&client) {
            self.order.get(*index).map(|p| *p)
        } else {
            None
        }
    }
}