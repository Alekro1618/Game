use engine::{app::{main_schedule::ScheduleLabel::Startup, plugin::Plugin}, camera::{Camera, SetMainCamera}, ecs::{entities::id::EntityId, hierarchy::{ChildOf, Children}, systems::{commands::Commands}, worlds::FromWorld}, math::{rect::Rect, vec::vec2}, render::color::Color, ui::{click::{ClickEvent, ClickEventPlugin, OnClick}, geometry::{UiRect, percent, px, vh, vw}, ui_node::{AlignItems, BackgroundColor, Display, JustifyContent, Node, OverflowClipBox::PaddingBox}, widget::Button}};
use engine_macros::Event;

pub struct MainMenu;

impl Plugin for MainMenu {
    fn build(self, app:  &mut engine::app::App) {
        app
            .add_plugin(ClickEventPlugin::<MainMenuEvent>::new())
            .add_systems(
                Startup, 
                main_menu
            );
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
        println!("new");
    }

    fn load(&self, commands : &mut Commands) {
        println!("load")
    }

    fn settings(&self, commands : &mut Commands) {

    }

    fn exit(&self, commands : &mut Commands) {

    }
}

pub fn main_menu(mut commands : Commands) {
    commands.queue( |world| {
        let entity = world.spawn((Camera::new(Rect::new(vec2(0., 0.), vec2(800., 800.))), ));
        world.trigger_ref(&mut SetMainCamera(entity));
    });

    commands
        .spawn_with_id((
            Node {
                width: vw(100),
                height: vh(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::Block,
                margin: UiRect::all(px(40)),
                padding: UiRect::left(px(20)),
                ..Default::default()
            },
            BackgroundColor(Color::GREEN)
        ))
        .add_children(|commands| {
            commands
                .spawn_with_id((
                    Button,
                    OnClick(MainMenuEvent::New),
                    Node {
                        width: percent(50),
                        height: px(20),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        display: Display::Block,
                        margin: UiRect::vertical(px(10)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::BLUE)
                )).and()
                .spawn_with_id((
                    Button,
                    OnClick(MainMenuEvent::Load),
                    Node { 
                        width: percent(50),
                        height: px(20),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        display: Display::Block,
                        margin: UiRect::vertical(px(10)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::RED)
                ))
        });
}