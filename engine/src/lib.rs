use crate::{animation::AnimationPlugin, app::{App, plugin::Plugin}, asset::AssetPlugin, camera::CameraPlugin, image::ImagePlugin, input::InputPlugin, relationship::RelationshipPlugin, render::sprite_renderer::RenderPlugin, ui::UIPlugin, utils::resources::ResourcePlugin, window::WindowPlugin};

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
pub mod texture;
pub mod ui;
pub mod utils;
pub mod video;
pub mod window;

#[cfg(test)]
mod test;

pub struct BasicPlugins {
    pub file_path : &'static str
}

impl Plugin for BasicPlugins {
    fn build(self, app: &mut App) {
        app
            .add_plugin(AssetPlugin { file_path: self.file_path })
            .add_plugin(AnimationPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(ImagePlugin)
            .add_plugin(InputPlugin)
            .add_plugin(RelationshipPlugin)
            .add_plugin(RenderPlugin)
            .add_plugin(UIPlugin)
            .add_plugin(ResourcePlugin)
            .add_plugin(WindowPlugin);
    }
}