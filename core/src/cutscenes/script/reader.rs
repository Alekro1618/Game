use std::collections::VecDeque;

use engine::{asset::AssetServer, ecs::{entities::id::EntityId, message::message_writer::MessageWriter, name::Name, query::filter::With, systems::query::Query}};
use engine_macros::{Event, Resource};
use macroquad::input::KeyCode::L;

use crate::{cutscenes::{actor::Actor, script::{Command, CommandMessage, Script, ScriptNode, ScriptNodeMessage, tags::Tag}}, items::Item};


pub enum ReaderState {
    Advance,
    Wait(ReaderWait)
}

#[derive(Clone, Copy)]
pub enum ReaderWait {
    WaitingForAdvance,
    WaitingForChoice,
    EndOfScript
}

pub struct ScriptCursor {
    line : usize,
}

#[derive(Resource)]
pub struct ScriptReader {
    cursor : ScriptCursor,
    state : ReaderState
}

impl Default for ScriptReader {
    fn default() -> Self {
        Self::new()
    }
}

impl ScriptReader {
    pub fn new() -> Self {
        Self {
            cursor : ScriptCursor { line: 0 },
            state: ReaderState::Advance
        }
    }

    pub fn advance(
        &mut self, 
        script : &Script,
        message_writer : &mut MessageWriter<ScriptNodeMessage>,
        actors : Query<(EntityId, &Name), With<Actor>>,
        asset_server : &AssetServer
    ) -> ReaderWait {
        self.state = ReaderState::Advance;
        
        loop {
            match &self.state {
                ReaderState::Advance => {
                    self.write_node(script, message_writer, &actors, asset_server, true);
                    self.cursor.line += 1;
                },
                ReaderState::Wait(reader_wait) => return *reader_wait,
            }
        }
    }

    fn write_node(
        &mut self, 
        script : &Script,
        message_writer : &mut MessageWriter<ScriptNodeMessage>,
        actors : &Query<(EntityId, &Name), With<Actor>>,
        asset_server : &AssetServer,
        forward : bool
    ) {
        let Some(node) = script.get(self.cursor.line) else {
            self.state = ReaderState::Wait(ReaderWait::EndOfScript);
            return;
        };
        
        match node {
            ScriptNode::Replica { text, speaker } => {
                let speaker = speaker.as_ref().map(|s| self.find_actor_id(script, s, actors));
                message_writer.write(ScriptNodeMessage::Replica { text: text.clone(), speaker });
                self.state = ReaderState::Advance;
            },
            ScriptNode::Choice { prompt, choices } => {
                message_writer.write(ScriptNodeMessage::Choice { prompt: prompt.clone(),choices : choices.clone() } );
                self.state = ReaderState::Wait(ReaderWait::WaitingForChoice);
            },
            ScriptNode::Jump(label) => {
                let Some(new_pos) = script.label(label) else {
                    panic!("Unknown label {}", label.0);
                };
                self.cursor.line = *new_pos;
                self.state = ReaderState::Advance;
            },
            ScriptNode::Command(command) => {
                let command = self.process_command(script, command, actors, asset_server);
                message_writer.write(ScriptNodeMessage::Command(command));
                self.state = ReaderState::Advance;
            },  
            ScriptNode::Pause => {
                self.state = ReaderState::Wait(ReaderWait::WaitingForAdvance);
            },
            _ => {
                self.state = ReaderState::Advance;
            }
        }
    }

    

    fn find_actor_id(
        &self, 
        script : &Script,
        speaker : &Box<str>,
        actors : &Query<(EntityId, &Name), With<Actor>>
    ) -> EntityId {
        actors
            .iter()
            .find(|(_, name)| name.equals(&script.actors[speaker].name))
            .map(|(id, _)| id)
            .unwrap()
    }

    fn process_command(
        &self,
        script : &Script,
        command : &Command,
        actors : &Query<(EntityId, &Name), With<Actor>>,
        asset_server : &AssetServer
    ) -> CommandMessage {
        match command {
            Command::ShowBackground(bg, transition_option) => {
                let bg = asset_server.load(&script.bgs[bg]);
                CommandMessage::ShowBackground(bg, transition_option.clone())
            },
            Command::ShowCharacter(character, relative_position, transition_option) => {
                let speaker = self.find_actor_id(script, character, actors);
                CommandMessage::ShowCharacter(speaker, relative_position.clone(), transition_option.clone())
            },
            Command::SetCharacter(character, state, transition_option) => {
                let speaker = self.find_actor_id(script, character, actors);
                CommandMessage::SetCharacter(speaker, state.clone(), transition_option.clone())
            },
            Command::SetCharacterPosition(character, relative_position, transition_option) => {
                let speaker = self.find_actor_id(script, character, actors);
                CommandMessage::SetCharacterPosition(speaker, relative_position.clone(), transition_option.clone())
            },
            Command::HideCharacter(character, transition_option) => {
                let speaker = self.find_actor_id(script, character, actors);
                CommandMessage::HideCharacter(speaker, transition_option.clone())
            },
            Command::PlayMusic(music) => {
                let music = asset_server.load(&script.music[music]);
                CommandMessage::PlayMusic(music)
            },
            Command::StopMusic => {
                CommandMessage::StopMusic
            },
            Command::PlaySound(sound) => {
                let sfx = asset_server.load(&script.sfx[sound]);
                CommandMessage::PlaySound(sfx)
            },
            Command::PlayEffect(effet) => {
                let effect = asset_server.load(&script.sfx[effet]);
                CommandMessage::PlaySound(effect)
            },
        }
    }

    pub fn back(&mut self, script : &Script, message_writer : &mut MessageWriter<ScriptNodeMessage>) {

    }
}




pub struct TextCursor {
    character : isize,
    current_tag : Tag
}

#[derive(Resource)]
pub struct TextReader {
    cursor : TextCursor,
    original : Vec<char>
}

impl Default for TextReader {
    fn default() -> Self {
        Self {
            cursor: TextCursor { character: 0, current_tag: Tag::Unknown },
            original: Vec::new()
        }
        
    }
}

impl TextReader {
     pub fn new() -> Self {
        Self {
            cursor : TextCursor { character: 0, current_tag : Tag::Unknown },
            original: Vec::with_capacity(200)
        }
    }

    pub fn advance(&mut self) -> Option<(char, Tag)> {
        if self.cursor.character < 0 {
            return None;
        }

        if self.original[0] == '[' {
            let is_closing = self.original[1] == '/';
            let end = self.cursor.character as usize - self.original
                .iter()
                .rev()
                .position(|c| c == &']')
                .unwrap() ;

            self.cursor.current_tag = if is_closing {
                Tag::Unknown
            } else {
                let drain = self.original.drain(end..);
                let tag = drain.as_slice();
                Tag::from(&tag[1..tag.len()-1])
            };
        }
        
        self.cursor.character -= 1;
        self.original.pop().map(|c| (c, self.cursor.current_tag))
    }

    pub fn drain(&mut self) -> impl Iterator<Item = (char, Tag)> {
        std::iter::from_fn(|| self.advance())
    }

    pub fn clear(&mut self) {
        self.original.clear();
    }

    pub fn set(&mut self, text : Box<str>) {
        self.original = text.chars().rev().collect();
        self.cursor.character = (self.original.len() - 1) as isize;
    }

    pub fn push(&mut self, text : Box<str>) {
        let old_text = std::mem::replace(
            &mut self.original,
             text.chars().rev().collect()
            );
        self.original.extend(old_text);
        self.cursor.character = (self.original.len() - 1) as isize;
    }
}

