use core::{
    iter::FusedIterator,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};
use std::collections::{HashMap, hash_map};

use super::{Entity, EntityHash};


pub struct EntityHashMap<V>(pub(crate) HashMap<Entity, V, EntityHash>);

impl<V> EntityHashMap<V> {
    pub const fn new() -> Self {
        Self(HashMap::with_hasher(EntityHash))
    }

    pub fn with_capacity(n: usize) -> Self {
        Self(HashMap::with_capacity_and_hasher(n, EntityHash))
    }

    pub fn into_inner(self) -> HashMap<Entity, V, EntityHash> {
        self.0
    }

    pub fn keys(&self) -> Keys<'_, V> {
        Keys(self.0.keys(), PhantomData)
    }

    pub fn into_keys(self) -> IntoKeys<V> {
        IntoKeys(self.0.into_keys(), PhantomData)
    }
}

impl<V> Default for EntityHashMap<V> {
    fn default() -> Self {
        Self(HashMap<Entity, V, EntityHash>::new())
    }
}

impl<V> Deref for EntityHashMap<V> {
    type Target = HashMap<Entity, V, EntityHash>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<V> DerefMut for EntityHashMap<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, V: Copy> Extend<&'a (Entity, V)> for EntityHashMap<V> {
    fn extend<T: IntoIterator<Item = &'a (Entity, V)>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl<'a, V: Copy> Extend<(&'a Entity, &'a V)> for EntityHashMap<V> {
    fn extend<T: IntoIterator<Item = (&'a Entity, &'a V)>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl<V> Extend<(Entity, V)> for EntityHashMap<V> {
    fn extend<T: IntoIterator<Item = (Entity, V)>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

// impl<V, const N: usize> From<[(Entity, V); N]> for EntityHashMap<V> {
//     fn from(value: [(Entity, V); N]) -> Self {
//         Self(HashMap::from_iter(value))
//     }
// }

// impl<V> FromIterator<(Entity, V)> for EntityHashMap<V> {
//     fn from_iter<I: IntoIterator<Item = (Entity, V)>>(iterable: I) -> Self {
//         Self(HashMap::from_iter(iterable))
//     }
// }


impl<'a, V> IntoIterator for &'a EntityHashMap<V> {
    type Item = (&'a Entity, &'a V);
    type IntoIter = hash_map::Iter<'a, Entity, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, V> IntoIterator for &'a mut EntityHashMap<V> {
    type Item = (&'a Entity, &'a mut V);
    type IntoIter = hash_map::IterMut<'a, Entity, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<V> IntoIterator for EntityHashMap<V> {
    type Item = (Entity, V);
    type IntoIter = hash_map::IntoIter<Entity, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub struct Keys<'a, V, S = EntityHash>(hash_map::Keys<'a, Entity, V>, PhantomData<S>);

impl<'a, V> Keys<'a, V> {
    /// Returns the inner [`Keys`](hash_map::Keys).
    pub fn into_inner(self) -> hash_map::Keys<'a, Entity, V> {
        self.0
    }
}

impl<'a, V> Deref for Keys<'a, V> {
    type Target = hash_map::Keys<'a, Entity, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, V> Iterator for Keys<'a, V> {
    type Item = &'a Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<V> ExactSizeIterator for Keys<'_, V> {}

impl<V> FusedIterator for Keys<'_, V> {}

impl<V> Clone for Keys<'_, V> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<V> Default for Keys<'_, V> {
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}


pub struct IntoKeys<V, S = EntityHash>(hash_map::IntoKeys<Entity, V>, PhantomData<S>);

impl<V> IntoKeys<V> {
    /// Returns the inner [`IntoKeys`](hash_map::IntoKeys).
    pub fn into_inner(self) -> hash_map::IntoKeys<Entity, V> {
        self.0
    }
}

impl<V> Deref for IntoKeys<V> {
    type Target = hash_map::IntoKeys<Entity, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<V> Iterator for IntoKeys<V> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<V> ExactSizeIterator for IntoKeys<V> {}

impl<V> FusedIterator for IntoKeys<V> {}

impl<V> Default for IntoKeys<V> {
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}


