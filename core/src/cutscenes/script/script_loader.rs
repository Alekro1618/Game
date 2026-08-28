use std::collections::{HashMap, VecDeque};

use engine::{asset::loader::AssetLoader, ecs::{schedule::node, worlds::FromWorld}, error::error::Error};
use macroquad::input::KeyCode::L;

use crate::cutscenes::script::{ActorDefinition, Label, Script, ScriptNode, script_parser::ScriptLine};

pub struct ScriptReader<'a> {
    source : &'a mut Vec<u8>,
    labels : HashMap<Label, usize>,
    actors : HashMap<Box<str>, ActorDefinition>,
    buffer : Vec<ScriptNode>
}

impl<'a> ScriptReader<'a> {
    fn new(source : &'a mut Vec<u8>) -> Self {
        Self {
            source: source,
            labels: HashMap::new(),
            actors: HashMap::new(),
            buffer: Vec::new()
        }
    }

    fn next_line(&mut self) -> Option<ScriptLine> {
        let newline_pos = self.source.iter()
            .position(|&b| b == b'\n')?;
        let mut line: Vec<u8> = self.source.drain(..=newline_pos).collect();
        line.pop();
        Some(ScriptLine::new(line))
    }

    fn next_node(&mut self) -> Option<Result<ScriptNode, ScriptLoaderError>> {
        if let Some(node) = self.buffer.pop() {
            return Some(Ok(node));
        }

        let line = self.next_line()?;

        let mut nodes = match line.parse() {
            Ok(nodes) => nodes,
            Err(err) => return Some(Err(err)),
        };

        let first = match nodes.pop() {
            Some(first) => first,
            None => return None,
        };

        self.buffer.extend(nodes);
    
        Some(Ok(first))
    }

    fn retrive_labels_and_actors(self) -> (HashMap<Label, usize>, HashMap<Box<str>, ActorDefinition>)  {
        (self.labels, self.actors)
    }
}


pub struct ScriptLoader;

impl FromWorld for ScriptLoader {
    fn from_world(world: &mut engine::ecs::worlds::World) -> Self {
        ScriptLoader
    }
}

impl AssetLoader for ScriptLoader {
    type Asset = Script;

    type Error = ScriptLoaderError;

    fn load(
        &self,
        reader: &mut Vec<u8>,
        _load_context: &mut engine::asset::loader::LoadContext,
    ) -> impl Future<Output = Result<Self::Asset, Self::Error>> + Send {
        async move {
            let mut script_reader = ScriptReader::new(reader);
            let mut script_result = Script::default();
            while let Some(node) = script_reader.next_node() {
                let node = node?;
                script_result.nodes.push(node);
            }
            (script_result.labels, script_result.actors) = script_reader.retrive_labels_and_actors();

            Ok(script_result)
        }
    }
}

pub enum ScriptLoaderError {
    NoTextAfterSpeaker,
    EmptyLabel,
    EmptyText,
    NoOptionsForChoice,
    NoEndOfChoice,
    NoMusic,
    NoSound,
}

impl Into<Error> for ScriptLoaderError {
    fn into(self) -> Error {
        todo!()
    }
}