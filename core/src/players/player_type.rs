use engine_macros::Component;


#[derive(Component, Clone, Copy)]
pub enum Client {
    First, 
    Second
}

#[derive(Component, Clone, Copy)]
pub enum Service {
    First,
    Second
}

#[derive(Component, Clone, Copy)]
pub enum Enemy {
    First, 
    Second
}