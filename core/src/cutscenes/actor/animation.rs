use std::collections::HashMap;

use engine::{animation::{Animatable, AnimationState, manager::{AnimationManager, AnimationManagerPlayer}, player::PlayerInstance, recording::RecordPlayer, sprite::{TextureRecordingPlayer, TextureState}}, math::{transform::Transform, vec::Vec2}};
use serde::{Deserialize, Serialize};




pub type ActorAnimationState = AnimationState<TextureState>;
pub type ActorAnimationManager = AnimationManagerPlayer<TextureState, TextureRecordingPlayer>;
pub type ActorAnimationPlayer = PlayerInstance<TextureRecordingPlayer>;
