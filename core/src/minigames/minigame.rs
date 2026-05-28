use std::marker::PhantomData;

use engine::ecs::{entities::{Entity, id::EntityId}, systems::commands::Commands};
use engine_macros::{Event, Resource};

use crate::{items::Item, players::player_type::PlayerType};

#[derive(Resource)]
pub struct MiniGameInstance<M : MiniGame> {
    instance : M
}

impl<M : MiniGame> MiniGameInstance<M> {
    pub fn new(instance : M) -> Self {
        Self { instance }
    }

    pub fn setup(&mut self, commands : &mut Commands, input : Option<Item>) {
        self.instance.on_setup(commands, input);
    }
}

pub trait MiniGame : Sync + Send + 'static + Default {
    fn on_setup(&mut self, commands : &mut Commands, input : Option<Item>);
    fn on_fail(&self, reason : impl MiniGameFailureReason) -> MiniGameFailed { reason.to_failed() }
    fn on_end(&self) -> Option<Item>;
}

pub trait MiniGameFailureReason {
    fn to_failed(self) -> MiniGameFailed;
}

pub enum MiniGameFailed {
    OtherItem(Option<Item>),
    ReturnItem,
    Nothing
}

#[derive(Event)]
pub struct SetupMiniGame<M : MiniGame>(pub Option<Item>, pub PlayerType, pub PhantomData<M>);

#[derive(Event)]
pub struct StartMiniGame<M : MiniGame>(pub Option<Item>, pub PlayerType, pub PhantomData<M>);

#[derive(Event)]
pub struct EndMiniGame<M : MiniGame>(pub Result<Option<Item>, MiniGameFailed>, pub PhantomData<M>);
