use std::collections::{HashMap, VecDeque};


use engine::ecs::schedule::node;

use crate::cutscenes::script::{Label, ScriptNode, script_loader::ScriptLoaderError};

pub struct ScriptLine {
    source : Vec<u8>
}

impl ScriptLine {
    pub fn new(source: Vec<u8>) -> Self {
        Self {
            source
        }
    }

    pub fn parse(self) -> Result<Vec<ScriptNode>, ScriptLoaderError> {
        let mut nodes: VecDeque<ScriptNode> = VecDeque::new();
        let mut parts = self.source
            .split(|b| b.is_ascii_whitespace())
            .filter(|p| !p.is_empty());

        while let Some(part) = parts.next() {
            match part {
                b"show" => {

                },
                b"set" => {

                },
                b"hide" => {
                    
                },
                b"play" => {
                    //parse_play(&mut nodes, &mut parts)?;
                },
                b"stop" => {
                    parse_stop(&mut nodes, &mut parts)?;
                },
                b"menu" => {
                    parse_menu(&mut nodes, &mut parts)?;
                },
                b"jump" => {
                    parse_jump(&mut nodes, &mut parts)?;
                },
                b"label" => {
                    parse_label(&mut nodes, &mut parts)?;
                },
                
                //If we don't know the tag then it is actor saying text or narrating text
                unknown => {
                    parse_dialog(&mut nodes, &mut parts, unknown)?;
                }
            }
        }

        Ok(Vec::from(nodes))
    }
}

pub fn quote<'w>(
    parts : &mut impl Iterator<Item=&'w [u8]>,
    current : &mut Vec<u8>
) -> Result<bool, ScriptLoaderError> {
    let mut is_pause = false;
    while let Some(after) = parts.next() {
        let (is_semicolon, after) = ends_with(after, &b';')?;

        let (is_end, after) = ends_with(after, &b'"')?;

        current.extend_from_slice(after);

        if is_end {
            is_pause = is_semicolon;
            break;
        }
    }
    Ok(is_pause)
}

pub fn ends_with<'w>(
    part : &'w [u8],
    symbol : &'w u8
) -> Result<(bool, &'w [u8]), ScriptLoaderError> {
    if part.last().ok_or(ScriptLoaderError::EmptyText)? == symbol {
        Ok((true, &part[..part.len()-1]))
    } else {
        Ok((false, part))
    }
}

fn to_box_str_unchecked(data: &[u8]) -> Box<str> {
    let s: &str = unsafe { std::str::from_utf8_unchecked(data) };
    Box::from(s)
}

pub fn parse_dialog<'w>(
    nodes : &mut VecDeque<ScriptNode>,
    parts : &mut impl Iterator<Item=&'w [u8]>,
    unknown : &[u8]
) -> Result<(), ScriptLoaderError> {
    //If starts from " then it's narrating text
                
    let (speaker, mut text) = if unknown[0] == b'"' {
        (None, unknown[1..].to_owned())
    } else {
        (Some(unknown), Vec::new())
    };

    let is_pause = quote(parts, &mut text)?;

    if text.is_empty() {
        return Err(ScriptLoaderError::NoTextAfterSpeaker)
    }
    
    nodes.push_back(ScriptNode::Replica { 
        text: to_box_str_unchecked(&text),
        speaker: speaker.map(|s| to_box_str_unchecked(s))
    });

    if is_pause {
        nodes.push_back(ScriptNode::Pause);
    }

    Ok(())
}

pub fn parse_label<'w>(
    nodes : &mut VecDeque<ScriptNode>,
    parts : &mut impl Iterator<Item=&'w [u8]>
) -> Result<(), ScriptLoaderError> {
    let Some(label_name) = parts.next() else {
        return Err(ScriptLoaderError::EmptyLabel);
    };

    let label = Label(to_box_str_unchecked(label_name));

    nodes.push_back(ScriptNode::Label(
        label
    ));

    Ok(())
}

pub fn parse_jump<'w>(
    nodes : &mut VecDeque<ScriptNode>,
    parts : &mut impl Iterator<Item=&'w [u8]>
) -> Result<(), ScriptLoaderError> {
    let Some(label_name) = parts.next() else {
        return Err(ScriptLoaderError::EmptyLabel);
    };

    let label = Label(to_box_str_unchecked(label_name));

    nodes.push_back(ScriptNode::Jump(
        label
    ));

    Ok(())
}

pub fn parse_menu<'w>(
    nodes : &mut VecDeque<ScriptNode>,
    parts : &mut impl Iterator<Item=&'w [u8]>
) -> Result<(), ScriptLoaderError> {
    let mut prompt = Vec::new();

    if quote(parts, &mut prompt)? {
        return Err(ScriptLoaderError::NoOptionsForChoice);
    }

    let prompt = to_box_str_unchecked(&prompt);

    let mut choices = HashMap::new();
    let mut counter = 0;

    loop {
        let Some(label_name) = parts.next() else {
            return Err(ScriptLoaderError::EmptyLabel);
        };

        let label = Label(to_box_str_unchecked(label_name));

        let mut option = Vec::new();
        let is_pause = quote(parts, &mut option)?;

        let option = to_box_str_unchecked(&option);

        choices.insert(label, option);

        if is_pause {
            break;
        }

        counter += 1;

        if counter > 10 {
            return Err(ScriptLoaderError::NoEndOfChoice);
        }
    }

    nodes.push_back(ScriptNode::Choice{
        prompt,
        choices
    });

    nodes.push_back(ScriptNode::Pause);

    Ok(())
}

pub fn parse_stop<'w>(
    nodes : &mut VecDeque<ScriptNode>,
    _parts : &mut impl Iterator<Item=&'w [u8]>
) -> Result<(), ScriptLoaderError> {
    nodes.push_back(ScriptNode::Command(super::Command::StopMusic));

    Ok(())
}
