use engine::render::color::Color;
use engine_macros::Resource;

use crate::players::player_type::Client;

#[derive(Resource)]
pub struct ClientList {
    pub clients : Vec<Client>
}

impl Default for ClientList {
    fn default() -> Self {
        Self { clients: vec![
            Client::First,
            Client::Second,
            Client::Third,
            Client::Fourth
        ]}
    }
}

pub fn client_color(client : &Client) -> Color {
    match client {
        Client::First => Color::BLACK,
        Client::Second => Color::BLUE,
        Client::Third => Color::GREEN,
        Client::Fourth => Color::YELLOW,
    }
}