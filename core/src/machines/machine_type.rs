use std::marker::PhantomData;

use crate::{items::Item, minigames::minigame::SetupMiniGame, players::player_type::PlayerType};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum MachineType {
    CoffeeMachine,
    RubbishCan,
    CoffeeBag,
    CashRegister,
    Fridge,
    Cutter
}