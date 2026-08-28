use crate::{ecs::entities::id::EntityId, ui::ui_node::Node};

pub struct NodeEdge<'a> {
    pub entity: EntityId,
    pub node: &'a Node,
    pub children: Vec<NodeEdge<'a>>,
}

impl<'a> NodeEdge<'a> {
    pub fn new(entity: EntityId, node: &'a Node) -> Self {
        Self {
            entity,
            node,
            children: Vec::new(),
        }
    }

    pub fn add_children(&mut self, children: Vec<NodeEdge<'a>>) {
        self.children = children;
    }
}
