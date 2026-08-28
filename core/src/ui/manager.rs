use engine::{app::{main_schedule::ScheduleLabel::Startup, plugin::Plugin}, camera::{Camera, SetMainCamera}, ecs::{entities::id::EntityId, hierarchy::{ChildOf, Children}, systems::{commands::Commands}, worlds::{FromWorld, World}}, math::{rect::Rect, vec::vec2}, render::color::Color, ui::{click::{ClickEvent, ClickEventPlugin, OnClick}, geometry::{UiRect, percent, px, vh, vw}, ui_node::{AlignItems, BackgroundColor, Display, JustifyContent, Node, OverflowClipBox::PaddingBox}, widget::Button}};
use engine_macros::Event;

use crate::{actions::events::AddAction, algos::path_finding::find_path, client::queue::SpawnRandomClient, instructions::{components::InstructionLabel, events::ExecuteInstruction}, items::{Item, food::Food}, machines::{machine_type::MachineType, register::MachineRegister}, players::{events::GoToPoint, player_actions::{Interact, SetPath}, player_register::PlayerRegister, player_type::{PlayerType, Service}}, utils::grid::{Grid, Point}};

pub struct Manager;

impl Plugin for Manager {
    fn build(self, app:  &mut engine::app::App) {
        app
            .add_plugin(ClickEventPlugin::<ManagerEvent>::new())
            .add_systems(
                Startup, 
                manager
            );
    }
}

#[derive(Event, Clone, Copy)]
pub enum ManagerEvent {
    Menu,
    Player(PlayerEvent)
}

#[derive(Clone, Copy)]
pub struct PlayerEvent {
    player : PlayerType,
    label : InstructionLabel,
}

impl ClickEvent for ManagerEvent  {
    fn on_click(&self, _node_id : EntityId, commands : &mut Commands) {
        match self {
            ManagerEvent::Menu => self.menu(commands),
            ManagerEvent::Player(_) => self.player(commands),
        }
    }
}

impl ManagerEvent {
    fn get_player(&self) -> &PlayerEvent {
        let ManagerEvent::Player(player_event) = self else {
            panic!("")
        };
        player_event
    }

    fn menu(&self, commands : &mut Commands) {
        println!("menu");
        commands.trigger(SpawnRandomClient);
    }

    fn player(&self, commands : &mut Commands) {
        let player_event = self.get_player().clone();
        commands.trigger(ExecuteInstruction(player_event.player, player_event.label));
    }
}


pub fn manager(mut commands : Commands) {
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
                ..Default::default()
            },
        ))
        .add_children(|commands| {
            commands
                .spawn_with_id((
                    Button,
                    OnClick(ManagerEvent::Player(PlayerEvent {
                            player: PlayerType::Service(Service::First),
                            label: InstructionLabel::Create(Item::Food(Food::Espresso))
                        }
                    )),
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
                    OnClick(ManagerEvent::Menu),
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