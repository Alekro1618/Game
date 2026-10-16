use std::marker::PhantomData;

use engine_macros::{Component, Reflect, reflect};

use crate::ecs::components::{Component, Mutable, StorageType};

#[derive(Component)]
pub struct EventWrapperComponent<E: Event>(PhantomData<E>);

pub trait Event: Send + Sync + Sized + 'static {}
