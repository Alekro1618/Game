use engine_macros::Component;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerType {
    Client(Client),
    Service(Service),
    Enemy(Enemy)
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Client {
    First, 
    Second,
    Third,
    Fourth
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Service {
    First,
    Second
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Enemy {
    First, 
    Second,
    Third,
    Fourth
}