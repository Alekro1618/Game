use std::{
    fmt::Display,
    sync::{
        Arc, OnceLock,
        atomic::{AtomicU32, Ordering},
    },
};

use crate::ecs::{
    bundle::Bundle,
    components::Component,
    entities::id::EntityId,
    event::Event,
    resource::Resource,
    systems::commands::command::CommandQueue,
    worlds::{World, log::print_component_of},
};

pub mod command;

static COUNTER: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PromiseId {
    pub id: u32,
}

impl PromiseId {
    fn new() -> Self {
        Self {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }
}

pub struct PromiseSpawn<B: Bundle> {
    pub id: PromiseId,
    pub cell: Arc<OnceLock<EntityId>>,
    pub bundle: Option<B>,
}

impl<B: Bundle> PromiseSpawn<B> {
    fn new(bundle: B) -> Self {
        Self {
            id: PromiseId::new(),
            cell: Arc::new(OnceLock::new()),
            bundle: Some(bundle),
        }
    }
}

impl<B: Bundle> Promise<EntityId> for PromiseSpawn<B> {
    fn get(self: Box<Self>, world: &mut World) -> EntityId {
        let bundle = self.bundle;
        *self.cell.get_or_init(|| {
            //The first caller of promise pointing to the same cell will always have bundle not empty.
            //The second caller doesn't need bundle.
            world.spawn(bundle.unwrap())
        })
    }

    fn transfer(&mut self) -> Box<dyn Promise<EntityId>> {
        Box::new(PromiseSpawn {
            id: self.id,
            cell: Arc::clone(&self.cell),
            bundle: self.bundle.take(),
        })
    }

    fn get_cell(&self) -> Option<EntityId> {
        self.cell.get().map(|e| *e)
    }

    fn is_trasnfered(&self) -> bool {
        self.bundle.is_some()
    }
}

pub trait Promise<T>: Send + Sync + 'static {
    ///Consumes promise to get value from the world
    fn get(self: Box<Self>, world: &mut World) -> T;
    ///Transfers the ownership of arguments to calculate the promise.
    fn transfer(&mut self) -> Box<dyn Promise<T>>;

    fn get_cell(&self) -> Option<T>;

    fn is_trasnfered(&self) -> bool;
}

pub struct Commands<'w> {
    pub queue: &'w mut CommandQueue,
}

pub struct CommandsPromise<'w, T> {
    pub queue: &'w mut CommandQueue,
    pub promise: Box<dyn Promise<T>>,
}

impl<'w, T: Send + Sync + 'static> CommandsPromise<'w, T> {
    pub fn then(
        mut self,
        fun: impl FnOnce(Commands, T) -> () + Send + Sync + 'static,
    ) -> Commands<'w> {
        let promise = self.promise.transfer();
        self.queue.push(move |world| {
            let value = promise.get(world);

            let mut queue = CommandQueue::default();
            {
                let commands = Commands::from(&mut queue);
                fun(commands, value);
            }

            queue.apply(world);
        });

        Commands { queue: self.queue }
    }

    pub fn and(self) -> CommandsPromiseVec<'w, T> {
        CommandsPromiseVec {
            queue: self.queue,
            promises: vec![self.promise],
        }
    }
}

pub struct CommandsPromiseVec<'w, T> {
    pub queue: &'w mut CommandQueue,
    pub promises: Vec<Box<dyn Promise<T>>>,
}

impl<'w, T: Send + Sync + 'static> CommandsPromiseVec<'w, T> {
    pub fn then(
        mut self,
        fun: impl Fn(Commands, Vec<T>) -> () + Send + Sync + 'static,
    ) -> Commands<'w> {
        let promises: Vec<Box<dyn Promise<T>>> =
            self.promises.iter_mut().map(|p| p.transfer()).collect();

        self.queue.push(move |world| {
            let value = promises.into_iter().map(|p| p.get(world)).collect();

            let mut queue = CommandQueue::default();
            {
                let commands = Commands::from(&mut queue);
                fun(commands, value);
            }

            queue.apply(world);
        });

        Commands { queue: self.queue }
    }

    fn promise(self, promise: impl Promise<T>) -> CommandsPromiseVec<'w, T> {
        let mut promises = self.promises;
        promises.push(Box::new(promise));

        CommandsPromiseVec {
            queue: self.queue,
            promises: promises,
        }
    }
}

impl<'w> CommandsPromiseVec<'w, EntityId> {
    pub fn spawn_with_id<B: Bundle>(self, bundle: B) -> CommandsPromiseVec<'w, EntityId> {
        self.promise(PromiseSpawn::new(bundle))
    }
}

impl<'w> Commands<'w> {
    pub fn from(queue: &'w mut CommandQueue) -> Self {
        Self { queue }
    }

    fn promise<T>(self, promise: impl Promise<T>) -> CommandsPromise<'w, T> {
        CommandsPromise {
            queue: self.queue,
            promise: Box::new(promise),
        }
    }

    pub fn spawn<B: Bundle>(&mut self, bundle: B) {
        self.queue.push(move |world| {
            world.spawn(bundle);
        });
    }

    pub fn spawn_with_id<B: Bundle>(self, bundle: B) -> CommandsPromise<'w, EntityId> {
        self.promise(PromiseSpawn::new(bundle))
    }

    pub fn despawn(&mut self, entity: EntityId) {
        self.queue.push(move |world| {
            world.despawn(entity);
        });
    }

    pub fn insert<R: Resource>(&mut self, resource: R) {
        self.queue.push(move |world| {
            world.insert_resource(resource);
        });
    }

    pub fn delete<R: Resource>(&mut self) {
        self.queue.push(move |world| {
            world.remove_resource::<R>();
        });
    }

    pub fn add<B: Bundle>(&mut self, entity: EntityId, bundle: B) {
        self.queue.push(move |world| {
            world.insert(entity, bundle);
        });
    }

    pub fn remove<B: Bundle>(&mut self, entity: EntityId) {
        self.queue.push(move |world| {
            world.remove::<B>(entity);
        });
    }

    pub fn trigger<'a, E: Event>(&mut self, mut event: E) {
        self.queue.push(move |world| {
            world.trigger_ref(&mut event);
        });
    }

    pub fn queue(&mut self, fun: impl Fn(&mut World) -> () + Send + Sync + 'static) {
        self.queue.push(move |world| {
            fun(world);
        });
    }

    pub fn log_archetype(&mut self) {
        self.queue
            .push(move |world| println!("{}", world.archetypes()));
    }

    pub fn log<I: Display + Send + Sync + 'static>(&mut self, value: I) {
        self.queue.push(move |_world| println!("{}", value));
    }

    pub fn log_component<T: Component + Display>(&mut self, entity: EntityId) {
        self.queue.push(move |world| {
            print_component_of::<T>(world, entity);
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::ecs::{
        components::{Component, Mutable, StorageType},
        entities::id::EntityId,
        hierarchy::{ChildOf, Children},
        name::Name,
        systems::commands::Commands,
        worlds::World,
    };

    #[derive(PartialEq, Eq, Debug)]
    pub struct A;

    impl Component for A {
        const STORAGE_TYPE: StorageType = StorageType::Table;

        type Mutability = Mutable;
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct B;
    impl Component for B {
        const STORAGE_TYPE: StorageType = StorageType::Table;

        type Mutability = Mutable;
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct C;
    impl Component for C {
        const STORAGE_TYPE: StorageType = StorageType::Table;

        type Mutability = Mutable;
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct D;
    impl Component for D {
        const STORAGE_TYPE: StorageType = StorageType::Table;

        type Mutability = Mutable;
    }

    #[test]
    pub fn test_children() {
        let mut world = World::new();
        let a = world.register_component::<A>();
        let b = world.register_component::<B>();
        let c = world.register_component::<C>();
        let d = world.register_component::<D>();

        println!("A = {}", a);
        println!("B = {}", b);
        println!("C = {}", c);
        println!("D = {}", d);

        let mut command_queue = world.command_queue();
        let commands = Commands::from(&mut command_queue);

        commands
            .spawn_with_id((Name::from("A"), A))
            .add_children(|commands| {
                commands
                    .spawn_with_id((Name::from("B"), B))
                    .and()
                    .spawn_with_id((Name::from("C"), C))
                    .add_children(|commands| commands.spawn_with_id((Name::from("D"), D)).and())
            });

        command_queue.apply(&mut world);

        let mut query = world.query::<(EntityId, &A, &Children)>();

        let mut parent = EntityId::new(0, 0);
        let mut children_ = Vec::<EntityId>::new();
        for (entity, _, children) in query.iter(&mut world) {
            parent = entity.clone();
            children_ = children.0.clone();
        }

        for child in children_ {
            let name = world.get_component::<Name>(child).unwrap();
            assert!(name.equals("B") || name.equals("C"));
            let child_of = world.get_component::<ChildOf>(child).unwrap();
            assert_eq!(child_of.0, parent);
        }

        let mut query = world.query::<(EntityId, &C, &Children)>();

        let mut parent = EntityId::new(0, 0);
        let mut children_ = Vec::<EntityId>::new();
        for (entity, _, children) in query.iter(&mut world) {
            parent = entity.clone();
            children_ = children.0.clone();
        }

        for child in children_ {
            let name = world.get_component::<Name>(child).unwrap();
            assert!(name.equals("D"));
            let child_of = world.get_component::<ChildOf>(child).unwrap();
            assert_eq!(child_of.0, parent);
        }
    }
}
