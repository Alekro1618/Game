use crate::{app::{main_schedule::ScheduleLabel::{PostStartup, PostUpdate, PreUpdate}, plugin::Plugin}, ecs::schedule::config::{IntoSchedule, resource_changed}, input::mouse::MousePos, ui::{render::UIRenderPlugin, systems::{on_layout_tree_update, place_nodes_on_camera_view, setup_nodes, update_hovering_state}, ui_camera::{resize_to_window, setup_ui_camera}}, window::Window};

pub mod click;
pub mod geometry;
mod render;
mod tree;
pub mod ui_node;
pub mod widget;
mod modifier;
mod taffy;
mod layout;
mod systems;
mod ui_camera;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(self, app: &mut crate::app::App) {
        app
            .add_systems(PostStartup, (
                setup_ui_camera,
                setup_nodes,
                place_nodes_on_camera_view
            ))
            .add_systems(
                PreUpdate, 
                update_hovering_state.run_if(resource_changed::<MousePos>())
            )
            .add_systems(PostUpdate, (
                resize_to_window,
                on_layout_tree_update
                ).run_if(resource_changed::<Window>())
            )
            .add_plugin(UIRenderPlugin);
    }
}
