use engine::{app::{main_schedule::ScheduleLabel::Startup, plugin::Plugin}, camera::{Camera, SetMainCamera}, ecs::{entities::id::EntityId, systems::{commands::Commands}}, math::{rect::Rect, vec::vec2}, render::color::Color, ui::{click::{ClickEvent, ClickEventPlugin}, geometry::{UiRect, px, vh, vw}, ui_node::{ BackgroundColor, Display, JustifyContent, Node, OverflowClipBox::PaddingBox}, widget::Button}};
use engine_macros::{Event};

pub struct Dialog;

impl Plugin for Dialog {
    fn build(self, app:  &mut engine::app::App) {
        app
            .add_plugin(ClickEventPlugin::<DialogEvent>::new())
            .add_systems(
                Startup, 
                dialog
            );
    }
}

#[derive(Event, Clone, Copy)]
pub enum DialogEvent {
    Advance,
    Skip,
    Menu
}

impl ClickEvent for DialogEvent  {
    fn on_click(&self, _node_id : EntityId, commands : &mut Commands) {

    }
}

pub fn dialog(commands : Commands) {
    commands
        .spawn_with_id(Camera::new(Rect::new(vec2(0., 0.), vec2(800., 800.))))
        .then(|mut commands, entity| {
            commands.trigger(SetMainCamera(entity));
        })
        .spawn_with_id((
            Node {
                width: vw(100),
                height: vh(100),
                padding: UiRect::all(px(20)),
                ..Default::default()
            },
            BackgroundColor(Color::GREEN)
        ))
        .add_children(|commands| {
            commands
                .spawn_with_id(
                    Node {
                        ..Default::default()
                    }
                ).and()
                .add_children( |commands| {
                    commands.spawn_with_id((
                        Node {
                            ..Default::default()
                        },
                        
                    )).and()
                })
        });
}