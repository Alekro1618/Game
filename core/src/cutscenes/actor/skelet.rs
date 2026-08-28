
use engine::{ecs::{entities::id::EntityId, observer::system_param::On, query::filter::With, systems::{commands::Commands, query::Query}}, math::transform::Transform, render::sprite::Sprite};
use engine_macros::{Component, Event};


#[derive(Component, Clone, Hash, PartialEq, Eq)]
pub enum Layer {
    Body,
    Eyes,
    Mouth
}

impl Layer {
    pub const ALL : [Self; 3] = [Self::Body, Self::Eyes, Self::Mouth];
}

pub struct LayerBundle {
    bone : Layer,
    sprite : Sprite,
    transform : Transform
}