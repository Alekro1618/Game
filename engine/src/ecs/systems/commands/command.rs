use crate::ecs::worlds::World;

type Command = Box<dyn FnOnce(&mut World) + Send + Sync>;

#[derive(Default)]
pub struct CommandQueue {
    commands: Vec<Command>,
}

impl CommandQueue {
    pub fn push(&mut self, cmd: impl FnOnce(&mut World) + Send + Sync + 'static) {
        self.commands.push(Box::new(cmd));
    }

    pub fn apply(&mut self, world: &mut World) {
        for cmd in self.commands.drain(..) {
            cmd(world)
        }
    }
}
