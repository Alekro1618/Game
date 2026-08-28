use std::{ops::Deref, slice};

use crate::ecs::{
    components::{Component, Mutable},
    entities::id::EntityId,
    systems::commands::{Commands, CommandsPromise, CommandsPromiseVec, command::CommandQueue},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChildOf(pub EntityId);

impl ChildOf {
    #[inline]
    pub fn parent(&self) -> EntityId {
        self.0
    }
}

impl Component for ChildOf {
    type Mutability = Mutable;
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Children(pub Vec<EntityId>);

impl Component for Children {
    type Mutability = Mutable;
}

impl Children {
    #[inline]
    pub fn swap(&mut self, a_index: usize, b_index: usize) {
        self.0.swap(a_index, b_index);
    }

    #[inline]
    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&EntityId, &EntityId) -> core::cmp::Ordering,
    {
        self.0.sort_by(compare);
    }

    #[inline]
    pub fn sort_by_key<K, F>(&mut self, compare: F)
    where
        F: FnMut(&EntityId) -> K,
        K: Ord,
    {
        self.0.sort_by_key(compare);
    }
}

impl<'a> IntoIterator for &'a Children {
    type Item = <Self::IntoIter as Iterator>::Item;

    type IntoIter = slice::Iter<'a, EntityId>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Deref for Children {
    type Target = [EntityId];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'w, 's> CommandsPromise<'w, EntityId> {
    pub fn add_children(
        mut self,
        fun: impl Fn(Commands) -> CommandsPromiseVec<EntityId> + Send + Sync + 'static,
    ) -> Commands<'w> {
        let promise = self.promise.transfer();

        self.queue.push(move |world| {
            let parent = promise.get(world);
            let mut queue = CommandQueue::default();
            {
                let commands = Commands::from(&mut queue);
                let spawned = fun(commands);

                spawned.then(move |mut commands, children| {
                    for &child in children.iter() {
                        commands.add(child, ChildOf(parent));
                    }
                    commands.add(parent, Children(children));
                });
            }

            queue.apply(world);
        });

        Commands { queue: self.queue }
    }
}

impl<'w> CommandsPromiseVec<'w, EntityId> {
    pub fn add_children(
        mut self,
        fun: impl Fn(Commands) -> CommandsPromiseVec<EntityId> + Send + Sync + 'static,
    ) -> CommandsPromiseVec<'w, EntityId> {
        let last = self.promises.last_mut().map(|p| p.transfer()).unwrap();

        self.queue.push(move |world| {
            let parent = last.get(world);
            let mut queue = CommandQueue::default();
            {
                let commands = Commands::from(&mut queue);
                let spawned = fun(commands);
                spawned.then(move |mut commands, children| {
                    for &child in children.iter() {
                        commands.add(child, ChildOf(parent));
                    }
                    commands.add(parent, Children(children));
                });
            }
            queue.apply(world);
        });

        self
    }
}
