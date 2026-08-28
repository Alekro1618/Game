use crate::{
    ecs::{
        components::{Component, Mutable},
        entities::id::EntityId,
        resource::Resource,
        worlds::World,
    },
    render::color::Color,
    ui::{
        render::layout::{Dimensions, LayoutBoxWithoutNode},
        ui_node::BackgroundColor,
        widget::{Text, TextColor, TextSize},
    },
};

#[derive(Default)]
pub struct LayoutTree(pub LayoutBoxWithoutNode);

impl Component for LayoutTree {
    type Mutability = Mutable;
}

impl Resource for LayoutTree {}
pub fn render_layout_tree(layout_tree: &LayoutTree, world: &World) {
    render_layout_box(&layout_tree.0, &world);
}

pub fn render_layout_box(layout_box: &LayoutBoxWithoutNode, world: &World) {
    render_node_edge(layout_box.entity, &layout_box.dimensions, world);
    for child in &layout_box.children {
        render_layout_box(&child, &world);
    }
}

pub fn render_node_edge<'a>(entity: EntityId, dimensions: &Dimensions, world: &World) {
    //let node = world.get_component::<Node>(entity).unwrap();

    if let Some(background_color) = world.get_component::<BackgroundColor>(entity) {
        render_background(&dimensions, &background_color);
    }

    if let Some(text) = world.get_component::<Text>(entity) {
        let text_size = world.get_component::<TextSize>(entity);
        let text_color = world.get_component::<TextColor>(entity);

        render_text(dimensions, text, text_size, text_color);
    }
}

pub fn render_background(dimensions: &Dimensions, background_color: &BackgroundColor) {
    //TODO:macroquad
    macroquad::shapes::draw_rectangle_ex(
        dimensions.content().x(),
        dimensions.content().y(),
        dimensions.content().width(),
        dimensions.content().height(),
        //TODO:macroquad
        macroquad::shapes::DrawRectangleParams {
            offset: macroquad::math::Vec2::ZERO.into(),
            rotation: 0.0,
            color: (&background_color.0).into(),
        },
    );
}

pub fn render_text(
    dimensions: &Dimensions,
    text: &Text,
    text_size: Option<&TextSize>,
    text_color: Option<&TextColor>,
) {
    let padding_box = dimensions.padding_box();
    let text_size = text_size.map(|t| t.0).unwrap_or(10.0);
    let text_color = text_color.map(|t| t.0).unwrap_or(Color::BLACK);

    //TODO:macroquad
    macroquad::text::draw_text(
        &text.0,
        padding_box.x(),
        padding_box.y(),
        text_size,
        text_color.into(),
    );
}
