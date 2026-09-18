use std::collections::{HashMap, VecDeque};

use engine::{asset::loader::AssetLoader, ecs::{schedule::node, worlds::FromWorld}, error::error::Error};
use macroquad::input::KeyCode::L;

use crate::cutscenes::script::{ActorDefinition, Label, Script, ScriptNode, script_parser::ScriptLine};

pub struct ScriptReader<'a> {
    source : &'a mut Vec<u8>,
    buffer : Vec<ScriptNode>
}

impl<'a> ScriptReader<'a> {
    fn new(source : &'a mut Vec<u8>) -> Self {
        Self {
            source: source,
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
            script_result.prepare();

            Ok(script_result)
        }
    }
}

#[derive(Debug)]
pub enum ScriptLoaderError {
    NoTextAfterSpeaker,
    EmptyLabel,
    EmptyText,
    NoOptionsForChoice,
    NoEndOfChoice,
    NoBackground,
    NoMusic,
    NoSound,
    NotFullCommand,
    NoTransition,
    UnknownTransition,
    NoPosition,
    UnknownPosition,
    NoState,
    NoActor,
    Custom(String)
}

impl Into<Error> for ScriptLoaderError {
    fn into(self) -> Error {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::cutscenes::script::{Script, script_loader::{ ScriptReader}};

    #[test]
    fn test() {
        let mut reader = fs::read("res/scripts/test.txt").unwrap();
        let mut script_reader = ScriptReader::new(&mut reader);
        let mut script_result = Script::default();
        let mut number = 0;
        while let Some(node) = script_reader.next_node() {
            println!("{}", number);
            let node = node.unwrap();
            number += 1;
            script_result.nodes.push(node);
        }
        script_result.prepare();

        for node in script_result.nodes {
            println!("{:#?}", node);
        }

    }
}