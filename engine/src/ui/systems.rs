use crate::{
    camera::Camera, ecs::{
        change_detection::params::Mut, entities::id::EntityId, hierarchy::{ChildOf, Children}, query::filter::{Changed, With, Without}, systems::{commands::Commands, query::{Query, Single}}, worlds::World,
    }, input::mouse::MousePos, ui::{
        layout::{Available, UiLayout}, render::interact::{Hovering, setup_hovering}, tree::NodeEdge, ui_camera::UiCamera, ui_node::Node,
    }, 
};

pub fn setup_nodes(mut commands: Commands, root_nodes: Query<EntityId, With<Node>>) {
    setup_hovering(&mut commands, &root_nodes);
}

pub fn update_hovering_state(
    tree : &UiLayout,
    node_query: Query< &mut Hovering, With<Node>>,
    mouse_pos: &MousePos,
) {
    let (hovering, not_hovering) = tree.check_hovering(mouse_pos.pos);
    
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

pub fn place_nodes_on_camera_view(
    mut commands: Commands,
    main_camera: Single<&Camera, With<UiCamera>>,
    root_node: Single<(EntityId, &Node), Without<ChildOf>>,
    world: &World,
) {
    let mut tree = UiLayout::default();
    let (entity, node) = *root_node;

    let node_edges = create_node_edge(world, entity, node);

    let default_node = Node::VIEWPORT;
    let mut node_root = NodeEdge::new(EntityId::PLACEHOLDER, &default_node);
    node_root.add_children(vec![node_edges]);

    tree.create_from_root(node_root, &Available { size: main_camera.rect().size() });

    commands.insert(tree);
}

pub fn create_node_edge<'a>(
    world: &'a World, 
    parent: EntityId, 
    node: &'a Node
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
    mut tree: Mut<UiLayout>, 
    query : Query<(EntityId, &Node), Changed<Node>>,
    main_camera: Single<&mut Camera, With<UiCamera>>,
) {
    let mut needs_recompute = main_camera.is_changed();

    for (entity, node) in query.iter() {
        needs_recompute = true;
        tree.update_node(entity, node, &Available { size: main_camera.rect().size() });
    }

    if needs_recompute {
        tree.recompute(&main_camera.rect().size())
    }
}




