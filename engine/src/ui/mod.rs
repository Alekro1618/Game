use crate::{app::plugin::Plugin, ui::render::UIRenderPlugin};

pub mod click;
pub mod geometry;
mod render;
mod tree;
pub mod ui_node;
pub mod widget;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(self, app: &mut crate::app::App) {
        app.add_plugin(UIRenderPlugin);
    }
}
