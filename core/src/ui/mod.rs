use engine::{app::plugin::Plugin, ui::UIPlugin};

use crate::ui::{dialog::Dialog, manager::{Manager, ManagerEvent}, menu::MainMenu};

pub mod manager;
pub mod dialog;
pub mod menu;


pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(self, app:  &mut engine::app::App) {
        app
            .add_plugin(Dialog);
    }
}