use engine::{asset::AssetServer, ecs::{change_detection::params::Mut, entities::id::EntityId, message::message_reader::MessageReader, query::filter::With, systems::{commands::Commands, query::Query}}, input::{button_input::ButtonInput, mouse::MouseButton}};

use crate::{actions::events::AddAction, cutscenes::{actor::{Actor, ActorBundle, actions::{Hide, MoveTo, SetState, Show}}, other::event::BackgroundTransition, script::{Command, CommandMessage, ScriptNodeMessage, TextReader, events::AdvanceScript}}};


pub fn update_cutscene(
    mut text_reader : Mut<TextReader>,
    mut script_messages : MessageReader<ScriptNodeMessage>,
    mut commands : Commands,
) {
    for script_node in script_messages.drain() {
        match script_node {
            ScriptNodeMessage::Replica { text, speaker } => {
                text_reader.push(text.clone());
            },
            ScriptNodeMessage::Choice { prompt, choices } => {
                text_reader.push(prompt.clone());
            },
            ScriptNodeMessage::Command(command) =>  {
                on_command(&command, &mut commands);
            },
            ScriptNodeMessage::ActorDefinition(definition) => {
                commands.spawn(ActorBundle::new(definition.name, definition.start, definition.animation_manager));
            },
        }
    }
}

pub fn on_command(
    command : &CommandMessage,
    commands : &mut Commands,
) {
    match command {
        CommandMessage::ShowBackground(
            background,
            _transition_option
        ) => {
            commands.trigger(BackgroundTransition(background.clone()));
        },
        CommandMessage::ShowCharacter(
            character,
            relative_position,
            transition_option
        ) => {
            commands.trigger(
                AddAction(*character, Box::new(
                        Show(relative_position.clone(), transition_option.clone())
                    )
                )
            );
        },
        CommandMessage::SetCharacter(
            character, 
            state, 
            transition_option
        ) => {
            commands.trigger(
                AddAction(*character, Box::new(
                        SetState(state.clone(), transition_option.clone())
                    )
                )
            );
        },
        CommandMessage::SetCharacterPosition(
            character, 
            relative_position, 
            transition_option
        ) => {
            commands.trigger(
                AddAction(*character, Box::new(
                        MoveTo(relative_position.clone(), transition_option.clone())
                    )
                )
            );
        },
        CommandMessage::HideCharacter(
            character, 
            transition_option
        ) => {
            commands.trigger(
                AddAction(*character, Box::new(
                        Hide(transition_option.clone())
                    )
                )
            );
        },
        CommandMessage::PlayMusic(music) => {
            
        },
        CommandMessage::StopMusic => {
            
        },
        CommandMessage::PlaySound(sound) => {

        },
        CommandMessage::PlayEffect(effect) => {

        },
    }
}
