use std::marker::PhantomData;

use engine::ecs::{observer::system_param::On, systems::commands::{self, Commands}};
use macroquad::miniquad::CursorIcon::Default;

use crate::{machines::{machine_function::Input, machine_type::MachineType}, minigames::{cutter::CutterMinigame, minigame::{MiniGame, MiniGameInstance, SetupMiniGame, StartMiniGame}}};


pub fn on_minigame_setup<M : MiniGame>(on : On<SetupMiniGame<M>>, mut commands : Commands) {
    let setup = on.event();
    let SetupMiniGame( input, machine_id, _) = setup;
    let mut instance = MiniGameInstance::new(M::default());
    instance.setup(&mut commands, *input);

    commands.insert(instance);
    commands.trigger(StartMiniGame::<M>(*input, *machine_id, PhantomData));
}

pub fn on_machine_input(on : On<Input>, mut commands: Commands) {
    let input = on.event();
    let Input(player_type, machine_type, input) = input;
    
    let setup = match machine_type {
        MachineType::Cutter =>   {
            Some(SetupMiniGame::<CutterMinigame>(*input, *player_type, PhantomData))
        },
        _ => {
            None
        }
    };

    let Some(setup) = setup else {
        return;
    };

    commands.trigger(setup);
}