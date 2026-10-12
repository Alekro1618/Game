use taffy::{Layout, NodeId};

use crate::{
    camera::Camera, ecs::{
        entities::id::EntityId, query::filter::With, systems::query::Single, worlds::World,
    }, image::TempQuadImageStorage, math::vec::{Vec2, vec2}, render::color::Color, ui::{ layout::UiLayout, ui_camera::UiCamera, ui_node::BackgroundColor, widget::{ImageNode, Text, TextColor, TextFont, TextSize},
    }
};

pub fn render_layout_tree(
    ui_layout: &UiLayout,
    world: &World,
    camera : Single<&Camera, With<UiCamera>>,
    image_storage : &TempQuadImageStorage,
) {
    macroquad::camera::set_camera(*camera);

    let Some(ref root) = ui_layout.root else  {
        return;
    };
    
    render_layout_box(ui_layout, world, *root, camera.rect().min, image_storage);

    macroquad::camera::set_default_camera();
}

pub fn render_layout_box(
    ui_layout: &UiLayout, 
    world: &World,
    node_id : NodeId,
    parent_pos : Vec2,
    image_storage : &TempQuadImageStorage
) {

    let layout = ui_layout.tree.layout(node_id).unwrap();
    let entity_id = ui_layout.node_to_entity.get(&node_id).unwrap();

    render_node_edge(*entity_id, layout, world, parent_pos, image_storage);
    for child in ui_layout.tree.children(node_id).unwrap() {
        render_layout_box(ui_layout, world, child, parent_pos + vec2(layout.location.x, layout.location.y), image_storage);
    }
}

pub fn render_node_edge<'a>(
    entity: EntityId, 
    layout: &Layout, 
    world: &World,
    parent_pos : Vec2,
    image_storage : &TempQuadImageStorage
) {
    if let Some(background_color) = world.get_component::<BackgroundColor>(entity) {
        render_background(layout, background_color, parent_pos);
    }

    if let Some(image_node) = world.get_component::<ImageNode>(entity) {
        draw_with_image(layout, image_node, parent_pos, image_storage);
    }

    if let Some(text) = world.get_component::<Text>(entity) {
        let text_size = world.get_component::<TextSize>(entity);
        let text_color = world.get_component::<TextColor>(entity);
        let text_font = world.get_component::<TextFont>(entity);

        render_text(layout, text, text_size, text_color, text_font, parent_pos);
    }
}

pub fn render_background(
    layout: &Layout, 
    background_color: &BackgroundColor, 
    parent_pos : Vec2
) {
    //TODO:macroquad
    macroquad::shapes::draw_rectangle_ex(
        parent_pos.x  + layout.location.x,
        parent_pos.y + layout.location.y,
        layout.size.width,
        layout.size.height,
        //TODO:macroquad
        macroquad::shapes::DrawRectangleParams {
            offset: macroquad::math::Vec2::ZERO.into(),
            rotation: 0.0,
            color: (&background_color.0).into(),
        }
    );
}

fn draw_with_image(
    layout: &Layout, 
    image_node: &ImageNode, 
    parent_pos : Vec2,
    image_storage : &TempQuadImageStorage
) {
    let Some(texture) = image_storage.get(image_node.image.index()) else {
        return;
    };

    macroquad::texture::draw_texture_ex(
        texture,
        parent_pos.x  + layout.location.x,
        parent_pos.y + layout.location.y,
        (&image_node.color).into(),
        macroquad::texture::DrawTextureParams {
            dest_size: Some(vec2(layout.size.width, layout.size.height).into()),
            source: image_node.rect.map(|r| r.into()),
            rotation: 0.0,
            flip_x: image_node.flip_x,
            flip_y: image_node.flip_y,
            pivot: None,
        },
    );
}

pub fn render_text(
    layout: &Layout,
    text: &Text,
    text_size: Option<&TextSize>,
    text_color: Option<&TextColor>,
    text_font: Option<&TextFont>,
    parent_pos : Vec2
) {
    let text_size = text_size.map(|t| t.0).unwrap_or(10);
    let text_color = text_color.map(|t| t.0).unwrap_or(Color::BLACK);
    let font = text_font.map(|t| &t.0);

    //TODO:macroquad
    macroquad::text::draw_text_ex(
        &text.0,
        parent_pos.x + layout.location.x,
        parent_pos.y + layout.location.y,
        macroquad::text::TextParams {
            font: font,
            font_size: text_size,
            rotation: 0.0,
            color: text_color.into(),
            ..Default::default()
        }
    );
}
