use std::{collections::HashMap, ops::{Deref, DerefMut}};

use engine_macros::{Component, Resource};
use taffy::{Layout, NodeId, TaffyTree};

use crate::{
    ecs::entities::id::EntityId, math::{
        rect::Rect,
        vec::{Vec2, vec2},
    }, ui::{
        taffy::to_style, tree::NodeEdge, ui_node::Node,
    },
};

#[derive(Component)]
pub struct UiLayoutChildOf(pub EntityId);

#[derive(Default, Component)]
pub struct UiLayout {
    pub tree : UiTree,
    pub root : Option<NodeId>,
    pub node_to_entity : HashMap<NodeId, EntityId>,
    pub entity_to_node : HashMap<EntityId, NodeId>,
}

impl UiLayout {
    pub fn new_leaf(&mut self, layout : taffy::Style) -> Result<NodeId, taffy::TaffyError> {
        self.tree.0.new_leaf(layout)
    }

    pub fn add_with_children(&mut self, layout : taffy::Style, children : &[NodeId]) -> Result<NodeId, taffy::TaffyError>{
        self.tree.0.new_with_children(layout, children)
    }

    pub fn create_from_root(&mut self, node_edge : NodeEdge, available : &Available) {
        self.root = Some(self.add_node_edge(node_edge, available));
    }

    pub fn add_node_edge(&mut self, node_edge : NodeEdge, available : &Available) -> NodeId {
        let children : Vec<NodeId> = node_edge.children.into_iter().map(|c| self.add_node_edge(c, available) ).collect();
        
        let node_id = match children.is_empty() {
            true => self.new_leaf(to_style(node_edge.node, available)).unwrap(),
            false => self.add_with_children(to_style(node_edge.node, &available), children.as_slice()).unwrap()
        };

        self.node_to_entity.insert(node_id, node_edge.entity);
        self.entity_to_node.insert(node_edge.entity, node_id);
        node_id
    }

    pub fn recompute(&mut self, size : &Vec2) {
        let available_space = taffy::geometry::Size {
            width: taffy::style::AvailableSpace::Definite(size.x as f32),
            height: taffy::style::AvailableSpace::Definite(size.y as f32),
        };

        let _ = self.tree.0.compute_layout(self.root.unwrap(), available_space).expect("Failed to recompute layout");
    }

    pub fn update_node(&mut self, entity : EntityId, node : &Node, available : &Available) {
        let node_id = self.entity_to_node.get(&entity).unwrap();
    
        self.tree.0.set_style(*node_id, to_style(node, available)).unwrap();
    }

    pub fn get_layout(&self, entity : EntityId) -> &Layout  {
        let node_id = self.entity_to_node.get(&entity).unwrap();

        self.tree.0.layout(*node_id).unwrap()
    }

    pub fn check_hovering(&self, point : Vec2) -> (impl Iterator<Item=EntityId>, impl Iterator<Item=EntityId>) {
        let (hovering, not) = get_hovering(&self.tree, point, self.root.unwrap(), vec2(0_f32, 0_f32));
        
        (
            hovering.into_iter()
                .map(|n| *self.node_to_entity.get(&n).unwrap()),
            not.into_iter()
                .map(|n| *self.node_to_entity.get(&n).unwrap())  
        )
    }

    pub fn entities(&self) -> impl Iterator<Item=EntityId> {
        self.entity_to_node.iter().map(|(key, _)| *key)
    }
}

fn get_hovering<'a>(tree : &UiTree, point : Vec2, node_id : NodeId, parent : Vec2) -> (Vec<NodeId>, Vec<NodeId>) {
    let layout = tree.0.layout(node_id).unwrap();
    let x = layout.location.x + parent.x;
    let y = layout.location.y + parent.y;
    let width = layout.size.width;
    let height = layout.size.height;
    let rect = Rect::new_xy(x, y, width, height);

    let mut hovering: Vec<NodeId> = Vec::new();
    let mut not_hovering: Vec<NodeId> = Vec::new();

    for child in tree.0.children(node_id).unwrap() {
        let (mut child_hovering, mut child_not_hovering) 
            = get_hovering(&tree, point, child, rect.min);

        hovering.append(&mut child_hovering);
        not_hovering.append(&mut child_not_hovering);
    }

    if rect.inside(point) {
        hovering.push(node_id);
    } else {
        not_hovering.push(node_id);
    }

    (hovering, not_hovering)
}

pub struct UiTree(TaffyTree<NodeMeasure>);

// SAFETY: we don't use calc feature
unsafe impl Send for UiTree {}
unsafe impl Sync for UiTree {}

impl Deref for UiTree {
    type Target = TaffyTree<NodeMeasure>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UiTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for UiTree {
    fn default() -> Self {
        Self(TaffyTree::new())
    }
}

#[derive(Default)]
pub struct NodeMeasure;

#[derive(Debug, Default, Clone)]
pub struct Available {
    pub size: Vec2
}