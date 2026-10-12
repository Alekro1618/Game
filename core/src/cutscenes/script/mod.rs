use std::{collections::{HashMap, HashSet}, sync::Arc};

use engine::{animation::{manager::AnimationManager, sprite::{TextureRecordingPlayer, TextureState}}, app::{main_schedule::ScheduleLabel, plugin::Plugin}, asset::{AssetPath, handle::Handle}, audio::audio_source::AudioSource, ecs::{entities::id::EntityId, message::message_writer::MessageWriter, observer::system_param::On, schedule::config::IntoSchedule, systems::commands::Commands}, image::Image, utils::Either};
use engine_macros::{Asset, Message, Resource};

use crate::cutscenes::{RelativePosition, TransitionOption, script::{events::AdvanceScript, reader::ScriptReader, script_loader::ScriptLoader, script_state::ScriptState, systems::{advance_script, retrive_loaded_script, script_is_advance, wants_to_load}}};

mod systems;
mod reader;
mod params;
mod tags;
mod script_parser;
mod script_loader;
pub mod events;
pub mod script_state;
pub use reader::TextReader;


pub struct ScriptPlugin;

impl Plugin for ScriptPlugin {
    fn build(self, app:  &mut engine::app::App) {
        app
            .init_asset::<Script>()
            .init_asset_loader::<ScriptLoader>()
            .add_message::<ScriptNodeMessage>()
            .add_systems(
                ScheduleLabel::Update,
                advance_script.run_if(script_is_advance)
            )
            .add_systems(
                ScheduleLabel::PostUpdate, 
                retrive_loaded_script.run_if(wants_to_load)
            );
    }
}

#[derive(Resource)]
pub struct CurrentScript(pub Option<Handle<Script>>, pub Option<Script>);

impl Default for CurrentScript {
    fn default() -> Self {
        Self(None, Default::default())
    }
}

#[derive(Asset, Default)]
pub struct Script {
    pub nodes : Vec<ScriptNode>,
    pub labels : HashMap<Label, usize>,
    pub actors : HashMap<Box<str>, ActorDefinition>,
    pub bgs : HashMap<Box<str>, AssetPath>,
    pub music : HashMap<Box<str>, AssetPath>,
    pub sfx : HashMap<Box<str>, AssetPath>,
}

impl Script {
    pub fn push_node(mut self, script_node : ScriptNode) -> Self {
        self.nodes.push(script_node);
        self
    }
    
    pub fn prepare(&mut self) {
        let (left, drained): (Vec<_>, Vec<_>) = self
            .nodes
            .drain(..)
            .enumerate()
            .partition(|(_, node)| 
                !matches!(node, ScriptNode::Label(_))
            );

        self.labels = drained
            .into_iter()
            .map(|(index, node)| {
                match node {
                    ScriptNode::Label(label) => {
                        (label.clone(), index)
                    },
                    _ => unreachable!()
                }
            })
            .collect();
        
        self.nodes = left.into_iter().map(|(_, node)| node).collect();
        
        for index in 0..self.nodes.len() {
            self.verify_node(index);
        }
    }

    pub fn get(&self, line : usize) -> Option<&ScriptNode> {
        self.nodes.get(line)
    }

    pub fn label(&self, label : &Label) -> Option<&usize> {
        self.labels.get(label)
    }     

    pub fn verify_node(&mut self, index : usize) {
        match &self.nodes[index] {
            ScriptNode::ActorDefinition { name, definition } => {
                if self.actors.insert(name.clone(), definition.clone()).is_some() {
                    panic!("Actor with name {} was already defined", name);
                }
            },
            ScriptNode::BgDefinition { name, path } => {
                if self.bgs.insert(name.clone(), path.clone()).is_some() {
                    panic!("Actor with name {} was already defined", name);
                }
            },
            ScriptNode::MusicDefinition { name, path } => {
                if self.music.insert(name.clone(), path.clone()).is_some() {
                    panic!("Actor with name {} was already defined", name);
                }
            },
            ScriptNode::SfxDefinition { name, path } => {
                if self.sfx.insert(name.clone(), path.clone()).is_some() {
                    panic!("Actor with name {} was already defined", name);
                }
            },
            ScriptNode::Jump(label) => {
                if !self.labels.contains_key(label) {
                    panic!("Jump to unknown label {}", label.0);
                }
            },
            ScriptNode::Command(command) => {
                self.verify_command(command);
            },
            ScriptNode::Replica { text, speaker } => {
                if let Some(speaker) = speaker {
                    if !self.actors.contains_key(speaker) {
                        panic!("Unknown actor {}", speaker);
                    }
                }
            },
            ScriptNode::Choice { prompt, choices } => {
                for (label, _) in choices{
                    panic!("Jump to unknown label {}", label.0);
                }
            },
            _ => {},
        }
    }

    pub fn verify_command(& self, command : &Command) {
        match command {
            Command::ShowBackground(name, _) => {
                if !self.bgs.contains_key(name) {
                    panic!("Unknown bg {}", name);
                }
            },
            Command::ShowCharacter(character, _, _) => {
                if !self.actors.contains_key(character) {
                    panic!("Unknown actor {}", character);
                }
            },
            Command::SetCharacter(character, _, _) => {
                if !self.actors.contains_key(character) {
                    panic!("Unknown actor {}", character);
                }
            },
            Command::SetCharacterPosition(character, _, _) => {
                if !self.actors.contains_key(character) {
                    panic!("Unknown actor {}", character);
                }
            },
            Command::HideCharacter(character, _) => {
                if !self.actors.contains_key(character) {
                    panic!("Unknown actor {}", character);
                }
            },
            Command::PlayMusic(music) => {
                if !self.music.contains_key(music) {
                    panic!("Unknown music {}", music);
                }
            },
            Command::PlaySound(sound) => {
                if !self.sfx.contains_key(sound) {
                    panic!("Unknown sfx {}", sound);
                }
            },
            Command::PlayEffect(effect) => {
                todo!()
            },
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
pub struct  ActorDefinition {
    name : Box<str>,
    animation_manager : AssetPath,
    start : Box<str>
}

impl ActorDefinition {
    pub fn new<S1 : Into<Box<str>>, S2 : Into<Box<str>>, P : Into<AssetPath>>(name : S1, animation_manager : P, start: S2) -> Self {
        Self {
            name: name.into(),
            animation_manager: animation_manager.into(),
            start: start.into(),
        }
    }
}

#[derive(Debug)]
pub enum ScriptNode {
    /// `speaker` "`text`"
    Replica {
        text : Box<str>,
        speaker : Option<Box<str>>,
    },
    /// menu "`prompt`" `label` "`choice1`" `label` "`choice2`" ...
    Choice {
        prompt: Box<str>,
        choices : HashMap<Label, Box<str>>
    },
    /// actor `actor short cut` "`real actor name`"
    ActorDefinition {
        name : Box<str>, 
        definition : ActorDefinition
    },
    /// bg `short cut` "`path`"
    BgDefinition {
        name : Box<str>, 
        path : AssetPath 
    },
    /// music `short cut` "`path`"
    MusicDefinition {
        name : Box<str>, 
        path : AssetPath
    },
    /// sfx `short cut` "`bg path`"
    SfxDefinition {
        name : Box<str>, 
        path : AssetPath
    },
    /// jump `label`
    Jump(Label),
    Command(Command),
    /// label `label`
    Label(Label),
    ///;
    Pause
}

pub struct ActorDefinitionMessage {
    pub name : Box<str>,
    pub animation_manager : Handle<AnimationManager<TextureState, TextureRecordingPlayer>>,
    pub start : Box<str>
}

#[derive(Message)]
pub enum ScriptNodeMessage {
    Replica {
        text : Box<str>,
        speaker : Option<EntityId>,
    },
    Choice {
        prompt: Box<str>,
        choices : HashMap<Label, Box<str>>
    },
    ActorDefinition {
        name : Box<str>,
        definition: ActorDefinitionMessage
    },
    Command(CommandMessage),
}

#[derive(Clone)]
pub enum CommandMessage {
    ///show bg `background` `transition`
    ShowBackground(Handle<Image>, TransitionOption),
    ///show `player` `position` `transition`
    ShowCharacter(EntityId, RelativePosition, TransitionOption),
    ///set `player` `state` `transition`
    SetCharacter(EntityId, Box<str>, TransitionOption),
    ///set `player` `position` `transition`
    SetCharacterPosition(EntityId, RelativePosition, TransitionOption),
    ///hide `player` `transition`
    HideCharacter(EntityId, TransitionOption),
    ///play m `name`
    PlayMusic(Handle<AudioSource>),
    ///stop m
    StopMusic,
    ///play s `name`
    PlaySound(Handle<AudioSource>),
    ///play e `name`
    PlayEffect(Handle<AudioSource>),
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Label(Box<str>);


#[derive(Debug, Clone)]
pub enum Command {
    ///show bg `background` `transition`
    ShowBackground(Box<str>, TransitionOption),
    ///show `player` `position` `transition`
    ShowCharacter(Box<str>, RelativePosition, TransitionOption),
    ///set `player` `state` `transition`
    SetCharacter(Box<str>, Box<str>, TransitionOption),
    ///set `player` `position` `transition`
    SetCharacterPosition(Box<str>, RelativePosition, TransitionOption),
    ///hide `player` `transition`
    HideCharacter(Box<str>, TransitionOption),
    ///play m `name`
    PlayMusic(Box<str>),
    ///stop m
    StopMusic,
    ///play s `name`
    PlaySound(Box<str>),
    ///play e `name`
    PlayEffect(Box<str>),
}