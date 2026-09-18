use engine::{animation::{AnimationState, sprite::TextureState}, camera::{Camera, MainCamera}, ecs::{entities::id::EntityId, query::filter::{With, Without}, systems::{commands::Commands, query::Query}}, math::{transform::Transform, vec::vec2}, render::sprite::Sprite, utils::resources::{GameSpeed, Time}};

use crate::cutscenes::{actor::{Actor, AppearEffect, HideEffect, Pos, TransitionPos, animation::{ActorAnimationManager, ActorAnimationPlayer}}};


pub fn pose_to_transform(
    query: Query<(& Pos, &mut Transform), Without<TransitionPos>>,
    camera : Query<&Camera, With<MainCamera>>
) {
    let camera = camera.iter().next().unwrap();
    
    let rect = camera.rect();
    let width = rect.width();

    for (pos, mut transform) in query {
        transform.position = vec2( width * pos.0.relative_to_percent(), 0.);
    }
}

pub fn transition_from_one_pos_to_another(
    query: Query<(&mut TransitionPos, &mut Transform)>,
    time : &Time
) {
    for (mut transition_pos, mut transform) in query {
        transition_pos.time += time.delta();
        if transition_pos.time >= 1.0 {
            transition_pos.is_end = true;
            continue;
        }
        transform.position = transition_pos.lerp();
    }
}

pub fn hide_effect(
    query: Query<(EntityId, &mut HideEffect, &mut Sprite)>,
    time : &Time,
    mut commands: Commands
) {
    for (entity_id, mut effect, mut sprite)  in query {
        effect.time += time.delta();
        if effect.time >= 1.0 {
            effect.is_end = true;
            if effect.despawn_on_end {
                commands.despawn(entity_id);
            }
            continue;
        }
        sprite.set_alpha(1.0 - effect.time);
    }
}

pub fn appear_effect(
    query: Query<(&mut AppearEffect, &mut Sprite)>,
    time : &Time
) {
    for (mut effect, mut sprite)  in query {
        effect.time += time.delta();
        if effect.time >= 1.0 {
            effect.is_end = true;
            continue;
        }
        sprite.set_alpha(effect.time);
    }
}

pub fn animate_actor(
    query: Query<(& AnimationState<TextureState>, &mut Sprite, &ActorAnimationPlayer), With<Actor>>
) {
    for (state, mut sprite, player) in query {
        let Some(ref state) = state.0 else {
            continue;
        };

        sprite.set_target(state.rect.clone());

        if let Some(ref image) = state.image {
            sprite.set_texture(image.clone());
        }
    }
}