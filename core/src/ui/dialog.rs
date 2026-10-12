use engine::{app::{main_schedule::ScheduleLabel::{self, Startup}, plugin::Plugin}, camera::{Camera, SetMainCamera}, ecs::{change_detection::params::Mut, entities::id::EntityId, query::filter::With, schedule::config::{IntoSchedule, changed_state_to, in_state}, systems::{commands::Commands, query::{Query, Single}}}, input::{button_input::ButtonInput, keyboard::KeyCode, mouse::MouseButton}, math::{rect::Rect, vec::vec2}, render::color::Color, ui::{click::{ClickEvent, ClickEventPlugin}, geometry::{UiRect, percent, px, vh, vw}, ui_node::{ AlignItems, BackgroundColor, Display, FlexDirection, JustifyContent, Node, OverflowClipBox::PaddingBox, PositionType}, widget::{Button, Text, TextColor, TextSize}}};
use engine_macros::{Component, Event};

use crate::{cutscenes::script::{TextReader, events::AdvanceScript, script_state::ScriptState}, game_state::GameState};

pub struct Dialog;

impl Plugin for Dialog {
    fn build(self, app:  &mut engine::app::App) {
        app
            .add_plugin(ClickEventPlugin::<DialogEvent>::new())
            .add_systems(
                ScheduleLabel::PreUpdate, 
                dialog.run_if(changed_state_to(GameState::Cutscene))
            )
            .add_systems(
                ScheduleLabel::Update,
                (
                    update_text_dialog,
                    on_advance.run_if(check_user_input_for_advance)
                ).run_if(in_state(GameState::Cutscene))
            );
    }
}

#[derive(Event, Clone, Copy)]
pub enum DialogEvent {
    Advance,
    Skip,
    Save,
    QuickSave,
    QuickLoad,
    Load,
    Settings
}

#[derive(Component)]
pub struct TextDialog;

#[derive(Component)]
pub struct NameDialog;

impl ClickEvent for DialogEvent  {
    fn on_click(&self, _node_id : EntityId, commands : &mut Commands) {
        match self {
            DialogEvent::Advance => commands.trigger(AdvanceScript),
            DialogEvent::Skip => todo!(),
            DialogEvent::Save => todo!(),
            DialogEvent::QuickSave => todo!(),
            DialogEvent::QuickLoad => todo!(),
            DialogEvent::Load => todo!(),
            DialogEvent::Settings => todo!(),
        }
    }
}


pub fn dialog(commands : Commands) {
    commands
        .spawn_with_id((
            Node {
                width: vw(100),
                height: vh(100),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(px(20)),
                position_type: PositionType::Relative,
                ..Default::default()
            },
        ))
        .add_children(|commands| {
            //The top
            commands
                .spawn_with_id((
                    Node {
                        width: percent(100),
                        flex_direction: FlexDirection::Row,
                        position_type: PositionType::Absolute,
                        top: px(0),
                        ..Default::default()
                    },
                )).and()
                .add_children(|commands| {
                    commands.spawn_with_id((
                        Button,
                        Node {
                            width: percent(100),
                            padding: UiRect::all(px(20)),
                            flex_direction: FlexDirection::Column,
                            bottom: px(0),
                            ..Default::default()
                        },
                    )).and()
                    .spawn_with_id((
                        Text(String::new()),
                        Node {
                            width: percent(100),
                            padding: UiRect::all(px(20)),
                            flex_direction: FlexDirection::Column,
                            bottom: px(0),
                            ..Default::default()
                        },
                    ))
                })
                //The bottom
                .spawn_with_id((
                    Node {
                        width: percent(100),
                        height: px(100),
                        flex_direction: FlexDirection::Column,
                        position_type: PositionType::Absolute,
                        bottom: px(0),
                        ..Default::default()
                    },
                ))
                .add_children( |commands| {
                    commands
                        //The name
                        .spawn_with_id((
                            Node {
                                width: percent(20),
                                ..Default::default()
                            },
                            BackgroundColor(Color::BLUE)
                        )).and()
                        .add_children(|commands| 
                            commands.spawn_with_id((
                                NameDialog,
                                Node { 
                                    ..Default::default()
                                },
                                Text(String::from("Test")),
                                TextSize(35),
                            )).and()
                        )
                        //The text itself
                        .spawn_with_id((
                            Node {
                                width: percent(70),
                                padding: UiRect::all(px(20)),
                                ..Default::default()
                            },
                            BackgroundColor(Color::RED),
                            
                        )).add_children(|commands| 
                            commands.spawn_with_id((
                                TextDialog,
                                Node {
                                    ..Default::default()
                                },
                                Text(String::new()), 
                                TextSize(25)
                            )
                            ).and()
                        )
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
