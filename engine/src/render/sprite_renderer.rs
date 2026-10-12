
use crate::{
    app::{App, main_schedule::ScheduleLabel, plugin::Plugin}, camera::{Camera, MainCamera}, ecs::{
        query::filter::With, systems::query::{Query, Single}, worlds::World,
    }, image::TempQuadImageStorage, math::{transform::Transform, vec::Vec2}, render::sprite::{Anchor, Sprite},
};

pub struct RenderPlugin;

pub fn extract(main_world: &mut World, render_world: &mut World) {
    core::mem::swap(main_world, render_world);
}

impl Plugin for RenderPlugin {
    fn build(self, app: &mut App) {
        app.add_systems(
            ScheduleLabel::RenderSystems,
            render,
        );

        //let mut render_app = SubApp::new();
        //app.set_extract(extract);
        //render_app.insert_sub_app("Render", render_app);
        //TODO: REMAKE TO BE A SUBAPP
    }
}

fn render(
    drawable: Query<(&Transform, &Sprite, Option<&Anchor>)>,
    main_camera: Single<&Camera, With<MainCamera>>,
    temp_storage: &TempQuadImageStorage
) {
    //TODO:macroquad
    macroquad::window::clear_background(macroquad::color::WHITE);
    macroquad::camera::set_camera(*main_camera);
    for (transform, sprite, anchor) in drawable {
        let default_anchor = Anchor::default();
        let anchor = anchor.unwrap_or(&default_anchor);
        if let Some(texture) = temp_storage.get(sprite.texture.index()) {
            draw_with_image(transform, sprite, texture, anchor);
        } else {
            if sprite.custom_size.is_some() {
                draw_with_custom_size(transform, sprite, anchor);
            } else {
                draw_else(transform, sprite);
            }
        }
    }
}

fn draw_with_image(
    transform: &Transform,
    sprite: &Sprite,
    texture: &macroquad::texture::Texture2D,
    anchor : &Anchor
) {
    let texture_size = sprite.custom_size.unwrap_or(Vec2::from(texture.size()));

    macroquad::texture::draw_texture_ex(
        texture,
        transform.position.x + 2. * texture_size.x * anchor.0.x,
        transform.position.y + 2. * texture_size.y * anchor.0.y,
        (&sprite.color).into(),
        macroquad::texture::DrawTextureParams {
            dest_size: Some(texture_size.mult_by(transform.scale).into()),
            source: sprite.rect.map(|r| r.into()),
            rotation: transform.rotation,
            flip_x: sprite.flip_x,
            flip_y: sprite.flip_y,
            pivot: None,
        },
    );
}

fn draw_with_custom_size(transform: &Transform, sprite: &Sprite, anchor : &Anchor) {
    //TODO:macroquad
    macroquad::shapes::draw_rectangle_ex(
        transform.position.x,
        transform.position.y,
        transform.scale.x * sprite.custom_size.unwrap().x,
        transform.scale.y * sprite.custom_size.unwrap().y,
        macroquad::shapes::DrawRectangleParams {
            offset: Vec2::ZERO.into(),
            rotation: transform.rotation,
            color: (&sprite.color).into(),
        },
    );
}

fn draw_else(_transform: &Transform, _sprite: &Sprite) {
    //TODO:macroquad
    // macroquad::shapes::draw_rectangle_ex(
    //     transform.position.x,
    //     transform.position.y,
    //     transform.scale.x * sprite.custom_size.unwrap().x,
    //     transform.scale.y * sprite.custom_size.unwrap().y,
    //     macroquad::shapes::DrawRectangleParams {
    //         offset: Vec2::ZERO.into(),
    //         rotation: transform.rotation,
    //         color: (&sprite.color).into(),
    //     }
    // );
}

// pub fn draw_texture_ex(
//     texture: &Texture2D,
//     x: f32,
//     y: f32,
//     color: Color,
//     params: DrawTextureParams,
// ) {
//     let context = get_context();

//     let [mut width, mut height] = texture.size().to_array();

//     let Rect {
//         x: mut sx,
//         y: mut sy,
//         w: mut sw,
//         h: mut sh,
//     } = params.source.unwrap_or(Rect {
//         x: 0.,
//         y: 0.,
//         w: width,
//         h: height,
//     });

//     let texture_opt = context
//         .texture_batcher
//         .get(texture)
//         .map(|(batched_texture, uv)| {
//             let [batched_width, batched_height] = batched_texture.size().to_array();
//             sx = ((sx / width) * uv.w + uv.x) * batched_width;
//             sy = ((sy / height) * uv.h + uv.y) * batched_height;
//             sw = (sw / width) * uv.w * batched_width;
//             sh = (sh / height) * uv.h * batched_height;

//             width = batched_width;
//             height = batched_height;

//             batched_texture
//         });
//     let texture = texture_opt.as_ref().unwrap_or(texture);

//     let (mut w, mut h) = match params.dest_size {
//         Some(dst) => (dst.x, dst.y),
//         _ => (sw, sh),
//     };
//     let mut x = x;
//     let mut y = y;
//     if params.flip_x {
//         x += w;
//         w = -w;
//     }
//     if params.flip_y {
//         y += h;
//         h = -h;
//     }

//     let pivot = params.pivot.unwrap_or(vec2(x + w / 2., y + h / 2.));
//     let m = pivot;
//     let p = [
//         vec2(x, y) - pivot,
//         vec2(x + w, y) - pivot,
//         vec2(x + w, y + h) - pivot,
//         vec2(x, y + h) - pivot,
//     ];
//     let r = params.rotation;
//     let p = [
//         vec2(
//             p[0].x * r.cos() - p[0].y * r.sin(),
//             p[0].x * r.sin() + p[0].y * r.cos(),
//         ) + m,
//         vec2(
//             p[1].x * r.cos() - p[1].y * r.sin(),
//             p[1].x * r.sin() + p[1].y * r.cos(),
//         ) + m,
//         vec2(
//             p[2].x * r.cos() - p[2].y * r.sin(),
//             p[2].x * r.sin() + p[2].y * r.cos(),
//         ) + m,
//         vec2(
//             p[3].x * r.cos() - p[3].y * r.sin(),
//             p[3].x * r.sin() + p[3].y * r.cos(),
//         ) + m,
//     ];
//     #[rustfmt::skip]
//     let vertices = [
//         Vertex::new(p[0].x, p[0].y, 0.,  sx      /width,  sy      /height, color),
//         Vertex::new(p[1].x, p[1].y, 0., (sx + sw)/width,  sy      /height, color),
//         Vertex::new(p[2].x, p[2].y, 0., (sx + sw)/width, (sy + sh)/height, color),
//         Vertex::new(p[3].x, p[3].y, 0.,  sx      /width, (sy + sh)/height, color),
//     ];
//     let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];

//     context.gl.texture(Some(texture));
//     context.gl.draw_mode(DrawMode::Triangles);
//     context.gl.geometry(&vertices, &indices);
// }
