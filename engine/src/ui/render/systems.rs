use crate::{
    camera::{Camera, MainCamera},
    ecs::{
        entities::id::EntityId,
        hierarchy::{ChildOf, Children},
        query::filter::{With, Without},
        systems::{commands::Commands, query::Query},
        worlds::World,
    },
    input::mouse::MousePos,
    math::rect::Rect,
    ui::{
        render::{
            interact::{Hovering, setup_hovering},
            layout::{Dimensions, LayoutBoxWithoutNode, forget_nodes, layout_tree},
            render_layout::{LayoutTree, render_layout_tree},
        },
        tree::NodeEdge,
        ui_node::Node,
    },
};

pub fn setup_nodes(mut commands: Commands, root_nodes: Query<EntityId, With<Node>>) {
    setup_hovering(&mut commands, &root_nodes);
}

pub fn place_nodes_on_camera_view(
    mut commands: Commands,
    main_camera: Query<&Camera, With<MainCamera>>,
    root_nodes: Query<(EntityId, &Node), Without<ChildOf>>,
    world: &World,
) {
    let camera = main_camera.iter().next().unwrap();

    let mut node_edges = Vec::new();
    for (entity, node) in root_nodes.iter() {
        node_edges.push(create_node_edge(world, entity, node));
    }
    let default_node = Node::VIEWPORT;
    let mut node_root = NodeEdge::new(EntityId::PLACEHOLDER, &default_node);
    node_root.add_children(node_edges);

    let layout_box = layout_tree(
        &node_root,
        Dimensions::with_content(camera.rect()),
        camera.rect().size(),
    )
    .unwrap_or_else(|e| panic!("{}", e));

    let forget_box = forget_nodes(&layout_box);

    layout_box_update(&mut commands, &forget_box);

    commands.insert(LayoutTree(forget_box));
}

pub fn create_node_edge<'a>(world: &'a World, parent: EntityId, node: &'a Node) -> NodeEdge<'a> {
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

pub fn on_layout_tree_update(mut commands: Commands, layout_tree: &LayoutTree) {
    layout_box_update(&mut commands, &layout_tree.0);
}

pub fn layout_box_update(commands: &mut Commands, layout_box: &LayoutBoxWithoutNode) {
    commands.add(layout_box.entity, layout_box.dimensions.content().clone());
    for child in &layout_box.children {
        layout_box_update(commands, child);
    }
}

pub fn render_ui(world: &World, layout_tree: &LayoutTree) {
    //TODO:macroquad
    macroquad::camera::set_default_camera();
    render_layout_tree(layout_tree, world);
}

pub fn update_hovering_state(
    node_query: Query<(&Rect, &mut Hovering), With<Node>>,
    mouse_pos: &MousePos,
) {
    for (rect, mut hovering) in node_query {
        hovering.0 = rect.inside(mouse_pos.pos);
    }
}
