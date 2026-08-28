use crate::{
    ecs::{
        components::{Component, Mutable}, entities::id::EntityId, hierarchy::{ChildOf, Children}, query::filter::{With, Without}, systems::query::Query,
    }, math::transform::Transform,
};

#[derive(Clone, Copy)]
pub struct RelativeTransform(Transform);

impl RelativeTransform {
    pub fn relate(&self, parent: &Transform, child: &mut Transform) {
        child.position = parent.position + self.0.position;
        child.rotation = parent.rotation + self.0.rotation;
        child.scale = parent.scale.mult_by(self.0.scale);
    }
}

impl Component for RelativeTransform {
    type Mutability = Mutable;
}
