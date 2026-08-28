use std::marker::PhantomData;

use engine::ecs::{observer::system_param::On, systems::commands::{self, Commands}};
use macroquad::miniquad::CursorIcon::Default;

use crate::{machines::{ machine_type::MachineType}, minigames::{cutter::CutterMinigame, minigame::{EndMiniGame, MiniGame, MiniGameInstance, SetupMiniGame, StartMiniGame}}};


pub fn on_minigame_setup<M : MiniGame>(on : On<SetupMiniGame<M>>, mut commands : Commands) {
    let setup = on.event();
    let SetupMiniGame( input, machine_id, _) = setup;
    let mut instance = MiniGameInstance::new(M::default());
    instance.setup(&mut commands, *input);

    commands.insert(instance);
    commands.trigger(StartMiniGame::<M>(*input, *machine_id, PhantomData));
    //Show screen
}

pub fn on_minigame_end<M : MiniGame>(on : On<EndMiniGame<M>>, instance : &MiniGameInstance<M>, mut commands : Commands) {
    let end = on.event();
    let EndMiniGame( result, _) = end;
    instance.unload(&mut commands);
    commands.delete::<MiniGameInstance<M>>();
    //Hide screen
}