
use crate::{
    ecs::{
        components::{Component, Mutable},
        entities::id::EntityId,
        query::filter::With,
        systems::{
            commands::Commands,
            query::Query,
        },
    },
    ui::ui_node::Node,
};

pub struct Hovering(pub bool);

impl Component for Hovering {
    type Mutability = Mutable;
}

pub fn setup_hovering(commands: &mut Commands, query: &Query<EntityId, With<Node>>) {
    for entity in query.iter() {
        commands.add(entity, Hovering(false));
    }
}
