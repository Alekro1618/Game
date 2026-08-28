use std::marker::PhantomData;

use engine::ecs::entities::id::EntityId;
use engine_macros::Event;

use crate::actions::action::{Action, IntoBoxedAction};

#[derive(Event)]
pub struct AddAction(pub EntityId, pub Box<dyn IntoBoxedAction>);

impl AddAction {
    pub fn from<A: Action>(entity : EntityId, into_boxed : impl IntoBoxedAction) -> Self {
        Self(entity, Box::new(into_boxed))
    }
}

#[derive(Event)]
pub struct QueueActions(pub EntityId, pub Vec<Box<dyn IntoBoxedAction>>);

impl QueueActions {
    pub fn new(entity : EntityId) -> Self {
        Self(entity, Vec::new())
    }

    pub fn add<A : Action>(&mut self, into_boxed : impl IntoBoxedAction) {
        self.1.push(Box::new(into_boxed));
    }
}

#[derive(Event)]
pub struct RemoveAction(pub EntityId);

pub struct Start;
impl ActionEventType for Start {}

pub struct Stop;
impl ActionEventType for Stop {}

pub struct Finish;
impl ActionEventType for Finish {}

pub trait ActionEventType : Sync + Send + 'static {}

#[derive(Event)]
pub struct ActionEvent<Marker : ActionEventType>(pub EntityId, pub PhantomData<Marker>);

impl<M: ActionEventType> ActionEvent<M> {
    pub fn new(entity : EntityId) -> Self {
        Self(entity, PhantomData)
    }
}


