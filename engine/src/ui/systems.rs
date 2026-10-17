use std::collections::HashSet;

use crate::{
    camera::Camera, ecs::{
        entities::id::EntityId, hierarchy::{ChildOf, Children}, query::filter::{Added, Changed, With, Without}, systems::{commands::Commands, query::{Query, Single}}, worlds::World,
    }, input::mouse::MousePos, ui::{
        layout::{Available, UiLayout, UiLayoutChildOf}, render::interact::{Hovering, setup_hovering}, tree::NodeEdge, ui_camera::UiCamera, ui_node::Node,
    }, 
};

pub fn setup_nodes(mut commands: Commands, root_nodes: Query<EntityId, (With<Node>, Added<Node>)>) {
    setup_hovering(&mut commands, &root_nodes);
}

pub fn update_hovering_state(
    ui_layouts : Query<&UiLayout>,
    node_query: Query< &mut Hovering, With<Node>>,
    mouse_pos: &MousePos,
) {
    for ui_layout in ui_layouts {
        let (hovering, not_hovering) 
            = ui_layout.check_hovering(mouse_pos.pos);
    
        for entity in hovering {
            if let Ok(mut hovering) = node_query.get(entity) {
                if !hovering.0 {
                    hovering.0 = true;
                }
            }
        }

        for entity in not_hovering {
            if let Ok(mut hovering) = node_query.get(entity) {
                if hovering.0 {
                    hovering.0 = false;
                }
            }
        }
    }

    
}

pub fn place_nodes_on_camera_view(
    mut commands: Commands,
    ui_camera: Single<&Camera, With<UiCamera>>,
    root_nodes: Query<(EntityId, &Node), (Without<ChildOf>, Added<Node>)>,
    world: &World,
) {
    let available = Available { size: ui_camera.rect().size() };
    for (entity, root_node) in root_nodes {
        let mut tree = UiLayout::default();

        let node_edges = create_node_edge(world, entity, &root_node);

        let default_node = Node::VIEWPORT;
        let mut node_root = NodeEdge::new(EntityId::PLACEHOLDER, &default_node);
        node_root.add_children(vec![node_edges]);

        tree.create_from_root(node_root, &available.clone());

        commands.queue(move |world|{
            let entites : Vec<EntityId> = tree.entities().collect();
            let entity_id = world.spawn(tree);
            entites
                .iter()
                .for_each(|e| {
                    world.insert(*e, UiLayoutChildOf(entity_id));
                });
        });
    }
}

pub fn create_node_edge<'a>(
    world: &'a World, 
    parent: EntityId, 
    node: &'a Node,
) -> NodeEdge<'a> {
    let mut node_edge = NodeEdge::new(parent, node);

    let Some(children) = world.get_component::<Children>(parent) else {
        return node_edge;
    };

    let mut node_edge_children = Vec::new();
    for &child in &children.0 {
        let child_node = world.get_component::<Node>(child).unwrap();
        node_edge_children.push(create_node_edge(world, child, child_node));
    }

    node_edge.add_children(node_edge_children);

    node_edge
}


pub fn on_layout_tree_update(
    ui_layouts : Query<&mut UiLayout>, 
    query : Query<(EntityId, &Node, &UiLayoutChildOf), Changed<Node>>,
    main_camera: Single<&mut Camera, With<UiCamera>>,
) {
    let mut needs_recompute = HashSet::new();
    for (entity, node, child_of) in query.iter() {
        let mut ui_layout = ui_layouts.get(child_of.0).unwrap();
        ui_layout.update_node(entity, node, &Available { size: main_camera.rect().size() });
        needs_recompute.insert(child_of.0);
    }

    for entity_id in needs_recompute {
        let mut ui_layout = ui_layouts.get(entity_id).unwrap();
        ui_layout.recompute(&main_camera.rect().size())
    }
}




