pub mod splash_screen;


use engine::{app::{main_schedule::ScheduleLabel, plugin::Plugin}, asset::{AssetServer, handle::Handle}, ecs::{change_detection::params::Mut, entities::id::EntityId, observer::system_param::On, query::filter::With, schedule::config::{IntoSchedule, resource_changed}, systems::{commands::Commands, query::Query}}, scene::{components::DynamicSceneRoot, dynamic::DynamicScene}, utils::{Timer, TimerPlugin}};
use engine_macros::{Component, Event, Resource};

use crate::{game_state::GameState, scenes::splash_screen::{fading_in_splash, fading_out_splash}};


pub struct SceneTransitionPlugin;

impl Plugin for SceneTransitionPlugin {
    fn build(self, app: &mut engine::app::App) {
        app
            .add_plugin(TimerPlugin::<FadedOut>::new())
            .add_plugin(TimerPlugin::<FadedIn>::new())
            .add_systems(
                ScheduleLabel::PostUpdate, 
                request_scene.run_if(resource_changed::<GameState>())
            )
            .add_observer(transition_faded_out)
            .add_systems(
                ScheduleLabel::Update,(
                set_fading_in.run_if(scene_loaded),
                fading_in_splash,
                fading_out_splash
            ))
            .add_observer(transition_faded_in);
    }
}

#[derive(Component)]
pub struct LevelRoot;

#[derive(Event)]
pub struct FadedOut;

#[derive(Event)]
pub struct FadedIn;

#[derive(Default)]
pub enum Phase {
    //No transition is happening
    #[default]
    Idle,
    //Waiting until the scene loads
    Loading,
    //The process of loading screen disappearing
    FadingIn,
    //The process of loading screen appearing
    FadingOut
}

#[derive(Resource, Default)]
pub struct SceneTransition {
    phase : Phase,
    target : Option<Handle<DynamicScene>>
}


fn request_scene(
    game_state : &GameState, 
    mut transition : Mut<SceneTransition>,
    asset_server : & AssetServer,
    mut commands: Commands
) {
    let path = match game_state {
        GameState::Cutscene => "res/scenes/room.ron",
        GameState::Menu => "res/scenes/menu.ron",
        GameState::Simulator => "res/scenes/room.ron",
    };

    let scene = asset_server.load(path);

    transition.target = Some(scene);
    transition.phase = Phase::FadingOut;

    let mut timer = Timer::<_, ()>::new(None, 0., 1.);
    timer.set_result(FadedOut);

    commands.spawn(timer);
}


fn transition_faded_out(
    _ : On<FadedOut>,
    mut commands : Commands,
    query : Query<EntityId, With<LevelRoot>>,
    mut transition : Mut<SceneTransition>,
) {
    for entity in query {
        commands.despawn(entity);
    }

    if let Some(ref next_scene) = transition.target {
        commands.spawn((DynamicSceneRoot(next_scene.clone()), LevelRoot));
    }

    transition.phase = Phase::Loading;
}

fn scene_loaded(
    transition : &SceneTransition,
    asset_server : &AssetServer
) -> bool {
    if  let Phase::Loading = transition.phase && 
        let Some(ref target) = transition.target
    {
        asset_server.is_loaded(target)
    } else {
        false
    }
}

fn set_fading_in(
    mut transition : Mut<SceneTransition>,
    mut commands : Commands
) {
    transition.phase = Phase::FadingIn;
    let mut timer = Timer::<_, ()>::new(None, 0., 1.);
    timer.set_result(FadedIn);

    commands.spawn(timer);
}

fn transition_faded_in(
    _ : On<FadedIn>,
    mut transition : Mut<SceneTransition>,
) {
    transition.phase = Phase::Idle;
}
