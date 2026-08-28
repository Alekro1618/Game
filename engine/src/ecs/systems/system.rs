use crate::ecs::worlds::World;

pub trait System: Send + Sync + 'static {
    type In : SystemInput;
    type Out;
    fn name(&self) -> &str;
    fn init(&mut self, world: &mut World);
    fn run(&mut self, input : Self::In, world: &mut World) -> Self::Out;
    fn trigger(&mut self, _world: &mut World) {}
    fn apply(&mut self, _world: &mut World) {}
}

pub trait SystemInput: Sized {}

impl SystemInput for () {}
