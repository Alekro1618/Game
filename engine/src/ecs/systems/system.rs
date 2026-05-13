use crate::ecs::worlds::World;

pub trait System: Send + Sync + 'static {
    type In;
    type Out;
    fn name(&self) -> &str;
    fn init(&mut self, world: &mut World);
    fn run(&mut self, world: &mut World);
    fn trigger(&mut self, world: &mut World) {}
    fn apply(&mut self, _world: &mut World) {}
}
