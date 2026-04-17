use ::Game::object;
use crate::engine::game::manager::Game;


#[object(name = "client_manager", texture = "pink", components = ["client_manager"], visible = false)]
pub struct ClientManager;

pub trait Managers {
    fn add_managers(&mut self) -> &mut Self;
}

impl Managers for Game {
    fn add_managers(&mut self) -> &mut Self {
        self
            .add_prefab(ClientManager::new())
    }
}
