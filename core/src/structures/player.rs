pub enum Player {
    Eyvind,
    Samantha,
    Gregory
}

impl Player {
    pub fn all() -> &'static [Player] {
        static PLAYERS: [Player; 3] = [Player::Eyvind, Player::Samantha, Player::Gregory];
        &PLAYERS
    }

    pub fn to(&self) -> &str {
        match self {
            Player::Eyvind => "eyvind",
            Player::Samantha => "samantha",
            Player::Gregory => "gregory",
        }
    }
}