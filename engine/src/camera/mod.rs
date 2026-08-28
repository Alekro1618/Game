use crate::{
    app::{main_schedule::ScheduleLabel, plugin::Plugin},
    ecs::{
        components::{Component, Mutable, StorageType},
        entities::id::EntityId,
        event::Event,
        observer::system_param::On,
        query::filter::With,
        systems::{
            commands::Commands,
            query::Query,
        },
    },
    math::{
        rect::Rect,
        vec::{Vec2, vec2},
    },
};

mod view;

pub struct Camera {
    rotation: f32,
    zoom: Vec2,
    target: Vec2,
    offset: Vec2,
    render_target: Option<u8>, //TODO
}

impl Component for Camera {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Mutable;
}

pub struct MainCamera;

impl Component for MainCamera {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Mutable;
}

pub struct SetMainCamera(pub EntityId);

impl Event for SetMainCamera {}

impl Camera {
    pub fn new(rect: Rect) -> Self {
        let target = 0.5 * (rect.min + rect.max);
        let v = rect.max - rect.min;
        let zoom = vec2(1. / v.x * 2., -1. / v.y * 2.);

        Self {
            rotation: 0.0,
            target,
            zoom,
            offset: vec2(0.0, 0.0),
            render_target: None,
        }
    }

    pub fn rect(&self) -> Rect {
        let zoom_inv = vec2(1. / self.zoom.x, -1. / self.zoom.y);
        let max = self.target + zoom_inv;
        let min = self.target - zoom_inv;

        Rect::new(min, max)
    }

    pub fn target(&self) -> Vec2 {
        self.target
    }

    pub fn translate(&mut self, by: Vec2) {
        self.target += by;
    }
}

//TODO:macroquad
impl macroquad::camera::Camera for Camera {
    fn matrix(&self) -> macroquad::prelude::Mat4 {
        let mat_origin = macroquad::math::Mat4::from_translation(macroquad::math::vec3(
            -self.target.x,
            -self.target.y,
            0.0,
        ));
        let mat_rotation = macroquad::math::Mat4::from_axis_angle(
            macroquad::math::vec3(0.0, 0.0, 1.0),
            self.rotation.to_radians(),
        );
        let invert_y = if self.render_target.is_some() {
            1.0
        } else {
            -1.0
        };
        let mat_scale = macroquad::math::Mat4::from_scale(macroquad::math::vec3(
            self.zoom.x,
            self.zoom.y * invert_y,
            1.0,
        ));
        let mat_translation = macroquad::math::Mat4::from_translation(macroquad::math::vec3(
            self.offset.x,
            self.offset.y,
            0.0,
        ));

        mat_translation * ((mat_scale * mat_rotation) * mat_origin)
    }

    fn depth_enabled(&self) -> bool {
        false
    }

    fn render_pass(&self) -> Option<macroquad::prelude::RenderPass> {
        None
    }

    fn viewport(&self) -> Option<(i32, i32, i32, i32)> {
        None
    }
}

pub fn set_main_camera(
    on: On<SetMainCamera>,
    mut commands: Commands,
    query_cameras: Query<EntityId, With<Camera>>,
    query_main_cameras: Query<EntityId, With<MainCamera>>,
) {
    for entity in query_main_cameras {
        commands.remove::<(MainCamera,)>(entity);
    }

    let set = on.event();

    let Ok(find) = query_cameras.get_mut(set.0) else {
        panic!("Camera doesn't exist")
    };

    commands.add(find, (MainCamera,));
}

pub fn render_camera(query: Query<&Camera, With<MainCamera>>) {
    for camera in query {
        //TODO:macroquad
        macroquad::camera::set_camera(camera);
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(self, app: &mut crate::app::App) {
        app.add_observer(set_main_camera)
            .add_systems(
                ScheduleLabel::PreUpdate,
                render_camera,
            );
    }
}
