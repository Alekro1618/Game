use engine::{BasicPlugins, MinimalPlugins, animation::AnimationPlugin, app::plugin::Plugin, camera::CameraPlugin, input::InputPlugin, render::sprite_renderer::RenderPlugin, ui::{UIPlugin, click::ClickEventPlugin}, utils::{profiler::ProfilerPlugin, resources::ResourcePlugin}};

use crate::{actions::ActionPlugin, client::ClientPlugin, cutscenes::{CutscenePlugin, script::ScriptPlugin}, enemy::EnemyPlugin, items::InventoryPlugin, machines::MachinePlugins, order::OrderPlugin, players::PlayerPlugin, service::ServicePlugin, ui::GameUIPlugin, utils::{grid::GridPlugin, mouse::GameCameraPlugin}};

pub struct AllPlugins;

impl Plugin for AllPlugins {
    fn build(self, app:  &mut engine::app::App) {
        app
        .add_plugin(BasicPlugins {
            file_path: "res/"
        })
        .add_plugin(GridPlugin)
        .add_plugin(MachinePlugins)
        .add_plugin(ClientPlugin)
        .add_plugin(ServicePlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(ActionPlugin)
        .add_plugin(InventoryPlugin)
        .add_plugin(OrderPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(GameUIPlugin)
        .add_plugin(CutscenePlugin)
        .add_plugin(GameCameraPlugin);
    }
}