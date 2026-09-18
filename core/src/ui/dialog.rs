use engine::{app::{main_schedule::ScheduleLabel::{self, Startup}, plugin::Plugin}, camera::{Camera, SetMainCamera}, ecs::{change_detection::params::Mut, entities::id::EntityId, query::filter::With, schedule::config::IntoSchedule, systems::{commands::Commands, query::{Query, Single}}}, input::{button_input::ButtonInput, keyboard::KeyCode, mouse::MouseButton}, math::{rect::Rect, vec::vec2}, render::color::Color, ui::{click::{ClickEvent, ClickEventPlugin}, geometry::{UiRect, percent, px, vh, vw}, ui_node::{ AlignItems, BackgroundColor, Display, JustifyContent, Node, OverflowClipBox::PaddingBox}, widget::{Button, Text}}};
use engine_macros::{Component, Event};

use crate::cutscenes::script::{TextReader, events::AdvanceScript, script_state::ScriptState};

pub struct Dialog;

impl Plugin for Dialog {
    fn build(self, app:  &mut engine::app::App) {
        app
            .add_plugin(ClickEventPlugin::<DialogEvent>::new())
            .add_systems(
                ScheduleLabel::Startup, 
                dialog
            )
            .add_systems(
                ScheduleLabel::Update,
                update_text_dialog
            )
            .add_systems(
                ScheduleLabel::Update,
                on_advance.run_if(check_user_input_for_advance)
            );
    }
}

#[derive(Event, Clone, Copy)]
pub enum DialogEvent {
    Advance,
    Skip,
    Menu
}

#[derive(Component)]
pub struct TextDialog;

impl ClickEvent for DialogEvent  {
    fn on_click(&self, _node_id : EntityId, commands : &mut Commands) {
        match self {
            DialogEvent::Advance => commands.trigger(AdvanceScript),
            DialogEvent::Skip => todo!(),
            DialogEvent::Menu => todo!(),
        }
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
                align_items: AlignItems::Center,
                ..Default::default()
            }
        ))
        .add_children(|commands| {
            //Dialog box
            commands
                .spawn_with_id((
                    Node {
                        width: percent(80),
                        height: px(300),
                        display: Display::Block,
                        margin: UiRect::vertical(px(10)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::RED)
                )).and()
                .add_children( |commands| {
                    //The text itself
                    commands.spawn_with_id((
                        TextDialog,
                        Node {
                            width: percent(80),
                            height: px(300),
                            display: Display::Block,
                            margin: UiRect::vertical(px(10)),
                            ..Default::default()
                        },
                        Text(String::new()),
                    )).and()
                })
        });
}

pub fn update_text_dialog(
    mut text_reader : Mut<TextReader>,
    mut dialog_text : Single<&mut Text, With<TextDialog>>
) {
    if let Some((symbol, tag))= text_reader.advance() {
        dialog_text.0.push(symbol);
    }
}

fn check_user_input_for_advance(keyboard : &ButtonInput<KeyCode>, mouse : &ButtonInput<MouseButton>) -> bool {
    keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Enter) || mouse.just_pressed(MouseButton::Left)
}

pub fn on_advance(
    mut script_state : Mut<ScriptState>,
    mut dialog_text : Single<&mut Text, With<TextDialog>>
) {
    script_state.advance();
    dialog_text.0.clear();
}
