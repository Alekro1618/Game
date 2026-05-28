use engine::{app::plugin::Plugin, ecs::{change_detection::params::Mut, observer::system_param::On, systems::commands::Commands}, math::{transform::Transform, vec::vec2}};

use crate::{items::{Item::{self}, food::Food}, minigames::{ minigame::{MiniGame, MiniGameFailed, MiniGameFailureReason, MiniGameInstance, StartMiniGame}, mixing::spoon::Spoon}};

mod systems;
mod spoon;

pub enum MixingFailureReason {
    Wrong,
    Canceled,
}

impl MiniGameFailureReason for MixingFailureReason {
    fn to_failed(self) -> MiniGameFailed {
        match self {
            MixingFailureReason::Wrong => MiniGameFailed::ReturnItem,
            MixingFailureReason::Canceled => MiniGameFailed::Nothing,
        }
    }
}

#[derive(Default)]
pub struct MixingMinigame {
    input : Option<Item>
}

impl MiniGame for MixingMinigame {
    fn on_setup(&mut self, commands : &mut Commands, input : Option<Item>) {
        self.input = input;
        on_minigame_setup(commands);
    }

    fn on_end(&self) -> Option<Item> {
        match self.input.unwrap() {
            Item::Food(food) => {
                match food {
                    Food::Bread => Some(Item::Food(Food::CuttedBread)),
                    _ => {
                        None
                    }
                }
            },
            _ => {
                None
            }
            
        }
    }
}

fn on_minigame_setup(commands : &mut Commands) {
    commands.spawn((
        Spoon,
        Transform::from_position(vec2(0.0, 0.0)))
    );
}

fn on_minigame_start(
    on : On<StartMiniGame<MixingMinigame>>, 
    mut commands : Commands, 
    mut minigame : Mut<MiniGameInstance<MixingMinigame>>
) {
    
}