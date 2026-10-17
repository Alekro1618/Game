use crate::{animation::AnimationPlugin, app::{App, plugin::Plugin}, asset::AssetPlugin, camera::CameraPlugin, image::ImagePlugin, input::InputPlugin, math::transform::TransformPlugin, relationship::RelationshipPlugin, render::{sprite::SpritePlugin, sprite_renderer::RenderPlugin}, scene::ScenePlugin, ui::UIPlugin, utils::resources::ResourcePlugin, window::WindowPlugin};

pub mod animation;
pub mod app;
pub mod asset;
pub mod audio;
pub mod camera;
pub mod core;
pub mod ecs;
pub mod error;
pub mod helper;
pub mod image;
pub mod input;
pub mod math;
pub mod relationship;
pub mod render;
pub mod scene;
pub mod ui;
pub mod utils;
pub mod video;
pub mod window;
pub mod exit;

pub struct BasicPlugins {
    pub file_path : &'static str
}

impl Plugin for BasicPlugins {
    fn build(self, app: &mut App) {
        app
            .add_plugin(AssetPlugin { file_path: self.file_path })
            .add_plugin(WindowPlugin)
            .add_plugin(AnimationPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(ResourcePlugin)
            .add_plugin(ImagePlugin)
            .add_plugin(InputPlugin)
            .add_plugin(RelationshipPlugin)
            .add_plugin(SpritePlugin)
            .add_plugin(RenderPlugin)
            .add_plugin(UIPlugin)
            .add_plugin(TransformPlugin)
            .add_plugin(ScenePlugin);
    }
}

pub struct MinimalPlugins {
    pub file_path : &'static str
}

impl Plugin for MinimalPlugins {
    fn build(self, app: &mut App) {
        app
            .add_plugin(AssetPlugin { file_path: self.file_path })
            .add_plugin(AnimationPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(ImagePlugin)
            .add_plugin(InputPlugin)
            .add_plugin(SpritePlugin)
            .add_plugin(TransformPlugin)
            .add_plugin(RelationshipPlugin)
            .add_plugin(ScenePlugin);
    }
}