use std::marker::PhantomData;

use engine_macros::Component;


#[derive(Component)]
pub struct EventWrapperComponent<E: Event>(PhantomData<E>);

pub trait Event: Send + Sync + Sized + 'static {}
