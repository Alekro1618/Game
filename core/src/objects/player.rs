use ::Game::object;
use crate::engine::game::manager::Game;

#[object(
    name = "eyvind",
    texture = "pink", 
    components = ["controller", "interact"], 
    states = [
        Idle,
        Making,
        Stop,
        Wait,
        Up,
        Down,
        Right,
        Left
    ],
    visible = true
)]
pub struct Player;

#[object(
    name = "gregory", 
    texture = "pink",
    components = ["controller", "interact"],
    visible = true
)]
pub struct Gregory;

#[object(
    name = "client",
    texture = "yellow", 
    components = ["client_controller"], 
    tags = ["client"], 
    visible = true,
    states = [
        Start,
        Walking,
        Proceed,
        End,
        Up,
        Down,
        Right,
        Left
    ]
)]
pub struct Client;

pub trait Players {
    fn add_players(&mut self) -> &mut Self;
}

impl Players for Game {
    fn add_players(&mut self) -> &mut Self {
        self
            .add_prefab(Player::new())
            .add_prefab(Gregory::new())
            .add_prefab(Client::new())
    }
}
