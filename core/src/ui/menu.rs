use engine::{app::{main_schedule::ScheduleLabel::{PostUpdate, PreUpdate, Startup}, plugin::Plugin}, asset::AssetServer, camera::{Camera, SetMainCamera}, ecs::{entities::id::EntityId, hierarchy::{ChildOf, Children}, message::message_writer::MessageWriter, observer::system_param::On, schedule::config::{IntoSchedule, changed_state_to}, systems::commands::Commands, worlds::FromWorld}, exit::ExitGame, math::{rect::Rect, vec::vec2}, render::color::Color, ui::{click::{ClickEvent, ClickEventPlugin, OnClick}, geometry::{UiRect, percent, px, vh, vw}, ui_node::{AlignItems, BackgroundColor, Display, JustifyContent, Node, OverflowClipBox::PaddingBox}, widget::{Button, ImageNode, Text}}};
use engine_macros::Event;

use crate::game_state::GameState;

pub struct MainMenu;

impl Plugin for MainMenu {
    fn build(self, app:  &mut engine::app::App) {
        app
            .add_plugin(ClickEventPlugin::<MainMenuEvent>::new())
            .add_systems(
                Startup, 
                main_menu.run_if(changed_state_to(GameState::Menu))
            )
            .add_observer(on_exit);
    }
}

#[derive(Event, Clone, Copy)]
pub enum MainMenuEvent {
    New,
    Load,
    Settings,
    Exit
}

impl ClickEvent for MainMenuEvent  {
    fn on_click(&self, _node_id : EntityId, commands : &mut Commands) {
        match self {
            MainMenuEvent::New => self.new(commands),
            MainMenuEvent::Load => self.load(commands),
            MainMenuEvent::Settings => self.settings(commands),
            MainMenuEvent::Exit => self.exit(commands),
        }
    }
}

impl MainMenuEvent {
    fn new(&self, commands : &mut Commands) {
        commands.with_resource::<GameState>(|mut game_state| {
            *game_state = GameState::Simulator
        });
    }

    fn load(&self, commands : &mut Commands) {
        println!("load");
    }

    fn settings(&self, commands : &mut Commands) {
        println!("settings")
    }

    fn exit(&self, commands : &mut Commands) {
        commands.trigger(Exit);
    }
}

#[derive(Event)]
pub struct Exit;

fn on_exit(on : On<Exit>, mut message_writer : MessageWriter<ExitGame>) {
    message_writer.write(ExitGame::Force);
}

pub fn main_menu(mut commands : Commands, asset_server : &AssetServer) {
    let button_texture = asset_server.load("res/images/ui/button.png");

    commands
        .spawn_with_id((
            Node {
                width: vw(100),
                height: vh(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::Block,
                padding: UiRect::left(px(20)),
                ..Default::default()
            },
        ))
        .add_children(move |commands| {
            commands
                .spawn_with_id((
                    Button,
                    OnClick(MainMenuEvent::New),
                    Node {
                        width: percent(50),
                        height: px(20),
                        justify_content: JustifyContent::Center,
                        display: Display::Block,
                        margin: UiRect::vertical(px(10)),
                        ..Default::default()
                    },
                    ImageNode::new(button_texture.clone()),
                    Text::new("New game")
                )).and()
                .spawn_with_id((
                    Button,
                    OnClick(MainMenuEvent::Load),
                    Node { 
                        width: percent(50),
                        height: px(20),
                        justify_content: JustifyContent::Center,
                        display: Display::Block,
                        margin: UiRect::vertical(px(10)),
                        ..Default::default()
                    },
                    ImageNode::new(button_texture.clone()),
                    Text::new("Load save")
                ))
                .spawn_with_id((
                    Button,
                    OnClick(MainMenuEvent::Settings),
                    Node { 
                        width: percent(50),
                        height: px(20),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        display: Display::Block,
                        margin: UiRect::vertical(px(10)),
                        ..Default::default()
                    },
                    ImageNode::new(button_texture.clone()),
                    Text::new("Settings")
                ))
                .spawn_with_id((
                    Button,
                    OnClick(MainMenuEvent::Exit),
                    Node { 
                        width: percent(50),
                        height: px(20),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        display: Display::Block,
                        margin: UiRect::vertical(px(10)),
                        ..Default::default()
                    },
                    ImageNode::new(button_texture),
                    Text::new("Exit")
                ))
        });
}