use crate::{
    app::{
        main_schedule::ScheduleLabel::{PostStartup, Update},
        plugin::Plugin,
    },
    camera::{Camera, MainCamera},
    ecs::{
        entities::id::EntityId,
        hierarchy::ChildOf,
        query::filter::{With, Without},
        systems::{
            commands::Commands,
            query::Query,
        },
        worlds::World,
    },
    input::mouse::MousePos,
    math::rect::Rect,
    ui::{
        render::{
            interact::Hovering,
            render_layout::LayoutTree,
            systems::{place_nodes_on_camera_view, render_ui, setup_nodes, update_hovering_state},
        },
        ui_node::Node,
    },
};

pub mod interact;
mod layout;
mod render_layout;
mod systems;

pub struct UIRenderPlugin;

impl Plugin for UIRenderPlugin {
    fn build(self, app: &mut crate::app::App) {
        app.add_systems(PostStartup, place_nodes_on_camera_view)
            .add_systems(PostStartup, setup_nodes)
            .add_systems(Update, render_ui)
            .add_systems(
                Update,
                update_hovering_state,
            );
    }
}
