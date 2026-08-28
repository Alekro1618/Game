use std::marker::PhantomData;

use engine::{app::plugin::Plugin, ecs::{change_detection::params::Mut, observer::system_param::On, systems::commands::Commands}, math::{transform::Transform, vec::vec2}};

use crate::{items::{Item::{self}, food::Food}, minigames::{cutter::knife::Knife, minigame::{EndMiniGame, MiniGame, MiniGameFailed, MiniGameFailureReason, MiniGameInstance, StartMiniGame}}};

mod systems;
mod knife;

pub enum CutterFailureReason {
    Wrong,
    Canceled,
}

impl MiniGameFailureReason for CutterFailureReason {
    fn to_failed(self) -> MiniGameFailed {
        match self {
            CutterFailureReason::Wrong => MiniGameFailed::ReturnItem,
            CutterFailureReason::Canceled => MiniGameFailed::Nothing,
        }
    }
}

#[derive(Default)]
pub struct CutterMinigame {
    input : Option<Item>
}

impl MiniGame for CutterMinigame {
    fn on_setup(&mut self, commands : &mut Commands, input : Option<Item>) {
        self.input = input;
        on_minigame_setup(commands);
    }

    fn on_unload(&self, commands : &mut Commands) {
        
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
        Knife,
        Transform::from_position(vec2(0.0, 0.0))),
    );
}

fn on_minigame_start(
    on : On<StartMiniGame<CutterMinigame>>, 
    mut commands : Commands,
    mut minigame : Mut<MiniGameInstance<CutterMinigame>>
) {
    let d = on.event();

    commands.trigger(EndMiniGame::<CutterMinigame>(Ok(minigame.end()), PhantomData));
}

fn move_knife() {
    
}

fn on_cut_knife() {

}

