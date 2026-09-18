use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::{
    animation::{
        loader::{AnimatableManagerMediaLoader, AnimatableMediaLoader}, manager::{AnimationManager, AnimationManagerPlayer}, player::{ Player, PlayerInstance}, recording::{RecordPlayer, Recording}, sprite::{TextureRecording, TextureRecordingPlayer},
    }, app::{main_schedule::ScheduleLabel, plugin::Plugin}, asset::{AssetServer, asset::Assets}, ecs::{
        change_detection::params::Mut, components::{Component, Mutable}, entities::id::EntityId, event::Event, observer::system_param::On, systems::query::Query,
    }, image::Image, math::{transform::Transform, vec::Vec2}, utils::resources::{GameSpeed, Time},
};

mod loader;
pub mod manager;
mod media;
pub mod player;
pub mod recording;
pub mod sprite;
mod transitions;

pub struct AnimationState<A: Animatable>(pub Option<A>);

impl<A: Animatable> Component for AnimationState<A> {
    type Mutability = Mutable;
}

pub trait Animatable:
    Serialize + for<'de> Deserialize<'de> + Clone + Sync + Send + 'static
{
    fn advance(self, rhs: Self, time: f32) -> Self;
}

impl<A: Animatable, B: Animatable> Animatable for (A, B) {
    fn advance(self, rhs: Self, time: f32) -> Self {
        (self.0.advance(rhs.0, time), self.1.advance(rhs.1, time))
    }
}

impl<A: Animatable, B: Animatable, C: Animatable> Animatable for (A, B, C) {
    fn advance(self, rhs: Self, time: f32) -> Self {
        (
            self.0.advance(rhs.0, time),
            self.1.advance(rhs.1, time),
            self.2.advance(rhs.2, time),
        )
    }
}

impl<A: Animatable, B: Animatable, C: Animatable, D: Animatable> Animatable for (A, B, C, D) {
    fn advance(self, rhs: Self, time: f32) -> Self {
        (
            self.0.advance(rhs.0, time),
            self.1.advance(rhs.1, time),
            self.2.advance(rhs.2, time),
            self.3.advance(rhs.3, time),
        )
    }
}

fn advance_animation<P: Player>(
    recordings: &Assets<P::Recording>,
    query: Query<(&mut PlayerInstance<P>, &mut AnimationState<P::State>)>,
    time: &Time,
) {
    for (mut player_instance, mut animation_state) in query {
        player_instance.is_end = false;
        let Some(handle) = player_instance.player.handle() else {
            continue;
        };
        let Some(recording) = recordings.get(handle.index()) else {
            continue;
        };
        animation_state.0 = Some(player_instance.player.advance(recording, time.delta()));

        if player_instance.player.is_end(recording) {
            player_instance.is_end = true;
            player_instance.player.on_end();
        }
    }
}

fn advance_animation_manager<P: Player>(
    animation_managers: &Assets<AnimationManager<P::State, P>>,
    query: Query<(&mut PlayerInstance<P>, &mut AnimationManagerPlayer<P::State, P>)>
) {
    for (mut player_instance, mut animation_manager_player) in query {
        let current_recording = animation_manager_player.current_recording_name.clone();
        let Some(animation_manager) 
            = animation_managers.get(animation_manager_player.handle().index()) else {
                continue;
            };

        let Some(handle) = animation_manager_player.advance_transition(
                player_instance.is_end, 
                animation_manager
            ) else {
                let Some(recording) = animation_manager_player.pick_recording(animation_manager, &current_recording) else {
                    panic!("Couldn't pick recording since it doesn't defined in animation manager {}", current_recording);
                };

                player_instance.player.replace_handle(recording);

                continue;
            };

        player_instance.player.replace_handle(handle);
    }
}

pub struct NextTransition<A : Animatable>(EntityId, String, PhantomData<A>);

impl<A : Animatable> NextTransition<A> {
    pub fn new<S : ToString>(entity: EntityId, to : S) -> NextTransition<A> {
        NextTransition(entity, to.to_string(), PhantomData)
    }
}

impl<A : Animatable> Event for NextTransition<A> {}


fn on_next_transition<P: Player>(
    on : On<NextTransition<P::State>>,
    animation_managers: &Assets<AnimationManager<P::State, P>>,
    query: Query<&mut AnimationManagerPlayer<P::State, P>>
) {
    let NextTransition(entity, to, _ ) = on.event();

    let mut animation_manager_player = query.get(*entity).unwrap();
    let handle = animation_manager_player.handle();

    let animation_manager = animation_managers.get(handle.index()).unwrap();

    animation_manager_player.pick_next_transition(animation_manager, to);
}

fn initiate_loading_for_texture_recording(
    mut recordings: Mut<Assets<TextureRecording>>,
    players : Query<&mut PlayerInstance<TextureRecordingPlayer>>,
    asset_server : &AssetServer
) {
    for handle in players
        .iter()
        .filter_map(
            |player| player.player.handle().cloned()
        ) 
    {
        let Some(recording) = recordings.get_mut(handle.index()) else {
            continue;
        };

        if recording.image.is_lazy() {
            recording.image.unlazify();
            asset_server.finally_load::<Image>(recording.image.path());
        }
    }
}

pub struct AnimationManagerPlugin<P : Player>(PhantomData<P>);


pub struct PlayerPlugin<P>(PhantomData<P>);

impl<P> PlayerPlugin<P> {
    pub fn new() -> Self {
        PlayerPlugin(PhantomData)
    }
}

impl<P: Player> Plugin for PlayerPlugin<P> {
    fn build(self, app: &mut crate::app::App) {
        app
            .init_asset::<P::Recording>()
            .init_asset::<AnimationManager<P::State, P>>()
            .init_asset_loader::<AnimatableMediaLoader<P::Recording, P::State>>()
            .init_asset_loader::<AnimatableManagerMediaLoader<P>>()
            .add_systems(
                ScheduleLabel::AnimationSystems, 
                advance_animation::<P>
            )
            .add_systems(
                ScheduleLabel::AnimationSystems, 
                advance_animation_manager::<P>
            )
            .add_observer(
                on_next_transition::<P>
            );
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(self, app: &mut crate::app::App) {
        app.add_plugin(PlayerPlugin::<RecordPlayer<f32>>::new())
            .add_plugin(PlayerPlugin::<RecordPlayer<Vec2>>::new())
            .add_plugin(PlayerPlugin::<RecordPlayer<Transform>>::new())
            .add_plugin(PlayerPlugin::<TextureRecordingPlayer>::new())
            .add_systems(ScheduleLabel::AnimationSystems, initiate_loading_for_texture_recording);
    }
}
