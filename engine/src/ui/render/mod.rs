use crate::{
    app::{
        main_schedule::ScheduleLabel::RenderSystems, plugin::Plugin,
    }, ui::render::render_layout::render_layout_tree,
};

pub mod interact;
mod render_layout;

pub struct UIRenderPlugin;

impl Plugin for UIRenderPlugin {
    fn build(self, app: &mut crate::app::App) {
        app
            .add_systems(
                RenderSystems, 
                render_layout_tree
            );
    }
}
