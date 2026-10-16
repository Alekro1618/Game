use engine_macros::Component;
use serde::{Deserialize, Serialize};

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlayerType {
    Client(Client),
    Service(Service),
    Enemy(Enemy)
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Client {
    First, 
    Second,
    Third,
    Fourth
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Service {
    First,
    Second
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Enemy {
    First, 
    Second,
    Third,
    Fourth
}