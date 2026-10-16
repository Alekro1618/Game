use engine::{animation::{AnimationState, manager::{AnimationManager, AnimationManagerPlayer}, player::{PlayerInstance, recording_player, texture_recording_player}, recording::{RecordPlayer, Recording}, sprite::{TextureRecordingPlayer, TextureState}}, app::{main_schedule::ScheduleLabel, plugin::Plugin}, asset::handle::Handle, camera::{Camera, MainCamera}, ecs::{name::Name, query::filter::With, worlds::World}, math::{transform::Transform, vec::Vec2}, render::{color::Color, sprite::{Anchor, Sprite}}};
use engine_macros::{Bundle, Component, Reflect, reflect};
use serde::{Deserialize, Serialize};

use crate::{actions::components::{ActionQueue, CurrentAction}, cutscenes::{Level, RelativePosition, actor::{animation::{ActorAnimationManager, ActorAnimationPlayer, ActorAnimationState}, systems::{animate_actor, appear_effect, hide_effect, pose_to_transform, transition_from_one_pos_to_another}}}};

mod skelet;
mod events;
pub mod actions;
mod animation;
mod systems;

#[derive(Component, Serialize, Deserialize, Reflect, Clone)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Actor;

#[derive(Component, Serialize, Deserialize, Reflect, Clone)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Pos(RelativePosition);

#[derive(Component)]
pub struct TransitionPos {
    pub from : Vec2,
    pub to : Vec2,
    time : f32,
    is_end : bool
}


#[derive(Component)]
pub struct HideEffect {
    time : f32,
    is_end : bool,
    despawn_on_end : bool
}

impl HideEffect {
    pub fn new(despawn_on_end : bool) -> Self {
        Self { time : 0.0, is_end : false, despawn_on_end}
    }
}

#[derive(Component)]
pub struct AppearEffect {
    time : f32,
    is_end : bool,
}

impl AppearEffect {
    pub fn new() -> Self {
        Self { time : 0.0, is_end : false}
    }
}

impl TransitionPos {
    pub fn lerp(&self) -> Vec2 {
        self.from.lerp(self.to, self.time)
    }

    pub fn from_camera(from : RelativePosition, to : RelativePosition, world : &mut World) -> Self {
        let mut query_state = world.query_filtered::<&Camera, With<MainCamera>>();
        let query = query_state.query(world);
        let camera = query.iter().next().unwrap();

        let rect = camera.rect();
        let width = rect.width();
        let height = rect.height();

        Self::new(
            Vec2 {
                x: width * from.relative_to_percent(), 
                y: height
            },
            Vec2 {
                x: width * to.relative_to_percent(), 
                y: height
            }
        )
    }

    pub fn new(from : Vec2, to : Vec2) -> Self {
        Self { from, to, time: 0.0, is_end: false }
    }
}

#[derive(Bundle)]
pub struct ActorBundle {
    actor : Actor,
    name : Name,
    pos : Pos,
    sprite : Sprite,
    anchor : Anchor,
    transform : Transform,
    animation_state : ActorAnimationState,
    animation_player : ActorAnimationPlayer,
    animation_manager : ActorAnimationManager,
    current_action : CurrentAction,
    action_queue : ActionQueue
} 

impl ActorBundle {
    pub fn new<S1 : ToString, S2 : ToString>(
        name : S1, 
        start : S2, 
        animation_manager : Handle<AnimationManager<TextureState, TextureRecordingPlayer>>
    ) -> Self {
        Self {
            actor: Actor,
            name: Name::from(name),
            pos: Pos(RelativePosition::OutOfBounds), 
            sprite: Sprite::from_color(Color::RED, Vec2::new(300.0, 300.0)),
            anchor: Anchor::BOTTOM_CENTER,
            animation_state: AnimationState(None), 
            animation_player: texture_recording_player(), 
            animation_manager: AnimationManagerPlayer::new(animation_manager, start),
            current_action : CurrentAction::new(),
            action_queue: ActionQueue::new(),
            transform: Transform::IDENTITY
        }
    }
}

pub struct ActorPlugin;


impl Plugin for ActorPlugin {
    fn build(self, app: &mut engine::app::App) {
        app
            .add_systems(ScheduleLabel::Update, transition_from_one_pos_to_another)
            .add_systems(ScheduleLabel::Update, hide_effect)
            .add_systems(ScheduleLabel::Update, appear_effect)
            .add_systems(ScheduleLabel::Update, animate_actor)
            .add_systems(ScheduleLabel::Update, pose_to_transform);
    }
}