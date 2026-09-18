use std::collections::{HashMap, VecDeque};

use engine::asset::AssetPath;

use crate::cutscenes::{Level, RelativePosition, TransitionOption, script::{ActorDefinition, Command::{self, HideCharacter}, Label, ScriptNode, script_loader::ScriptLoaderError}};

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
                b"actor" => parse_actor_def(&mut nodes, &mut parts)?,
                b"music" => parse_music_def(&mut nodes, &mut parts)?,
                b"sfx" => parse_sound_def(&mut nodes, &mut parts)?,
                b"label" => parse_label(&mut nodes, &mut parts)?,
                b"hide" => parse_hide(&mut nodes, &mut parts)?, 
                b"play" => parse_play(&mut nodes, &mut parts)?,
                b"stop" => parse_stop(&mut nodes, &mut parts)?,
                b"menu" => parse_menu(&mut nodes, &mut parts)?,
                b"jump" => parse_jump(&mut nodes, &mut parts)?,
                b"show" => parse_show(&mut nodes, &mut parts)?,
                b"bg" => parse_bg_def(&mut nodes, &mut parts)?,
                b"set" => parse_set(&mut nodes, &mut parts)?,
                //If we don't know the tag then it is actor saying text or narrating text
                unknown => parse_dialog(&mut nodes, &mut parts, unknown)?
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

    if !current.is_empty() {
        let (is_semicolon, after) = ends_with(current, &b';')?;
        let (is_end, _) = ends_with(after, &b'"')?;

        if is_end {
            current.pop();
            return Ok(is_semicolon);
        }
    }

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
        (None, unknown[1..unknown.len()].to_owned())
    } else {
        (Some(unknown), Vec::new())
    };

    let is_pause = quote(parts, &mut text)?;

    if text.is_empty() {
        return Err(ScriptLoaderError::NoTextAfterSpeaker)
    }
    
    nodes.push_front(ScriptNode::Replica { 
        text: to_box_str_unchecked(&text),
        speaker: speaker.map(|s| to_box_str_unchecked(s))
    });

    if is_pause {
        nodes.push_front(ScriptNode::Pause);
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

    nodes.push_front(ScriptNode::Label(
        label
    ));

    Ok(())
}

pub fn parse_actor_def<'w>(
    nodes : &mut VecDeque<ScriptNode>,
    parts : &mut impl Iterator<Item=&'w [u8]>
) -> Result<(), ScriptLoaderError> {
    let Some(local_name) = parts.next() else {
        return Err(ScriptLoaderError::NoActor);
    };

    let Some(full_name) = parts.next() else {
        return Err(ScriptLoaderError::NoActor);
    };

    let mut full_name = full_name[1..full_name.len()].to_owned();
    

    let _ = quote(parts, &mut full_name);

    let Some(animation_manager) = parts.next() else {
        return Err(ScriptLoaderError::NoActor);
    };

    let Some(initial_state) = parts.next() else {
        return Err(ScriptLoaderError::NoActor);
    };

    nodes.push_front(ScriptNode::ActorDefinition {
        name: to_box_str_unchecked(local_name),
        definition: ActorDefinition::new(
            to_box_str_unchecked(&full_name), 
            AssetPath::from(to_box_str_unchecked(animation_manager)), 
            to_box_str_unchecked(initial_state)
        )
    });

    Ok(())
}

pub fn parse_bg_def<'w>(
    nodes : &mut VecDeque<ScriptNode>,
    parts : &mut impl Iterator<Item=&'w [u8]>
) -> Result<(), ScriptLoaderError> {
    let Some(local_name) = parts.next() else {
        return Err(ScriptLoaderError::NoBackground);
    };

    let Some(full_path) = parts.next() else {
        return Err(ScriptLoaderError::NoBackground);
    };

    nodes.push_front(ScriptNode::BgDefinition { 
        name: to_box_str_unchecked(local_name), 
        path: AssetPath::from(to_box_str_unchecked(full_path))
    });

    Ok(())
}


pub fn parse_music_def<'w>(
    nodes : &mut VecDeque<ScriptNode>,
    parts : &mut impl Iterator<Item=&'w [u8]>
) -> Result<(), ScriptLoaderError> {
    let Some(local_name) = parts.next() else {
        return Err(ScriptLoaderError::NoMusic);
    };

    let Some(full_path) = parts.next() else {
        return Err(ScriptLoaderError::NoMusic);
    };

    nodes.push_front(ScriptNode::MusicDefinition { 
        name: to_box_str_unchecked(local_name), 
        path: AssetPath::from(to_box_str_unchecked(full_path))
    });

    Ok(())
}

pub fn parse_sound_def<'w>(
    nodes : &mut VecDeque<ScriptNode>,
    parts : &mut impl Iterator<Item=&'w [u8]>
) -> Result<(), ScriptLoaderError> {
    let Some(local_name) = parts.next() else {
        return Err(ScriptLoaderError::NoSound);
    };

    let Some(full_path) = parts.next() else {
        return Err(ScriptLoaderError::NoSound);
    };

    nodes.push_front(ScriptNode::SfxDefinition { 
        name: to_box_str_unchecked(local_name), 
        path: AssetPath::from(to_box_str_unchecked(full_path))
    });

    Ok(())
}


pub fn parse_play<'w>(
    nodes : &mut VecDeque<ScriptNode>,
    parts : &mut impl Iterator<Item=&'w [u8]>
) -> Result<(), ScriptLoaderError> {
    let Some(media_type) = parts.next() else {
        return Err(ScriptLoaderError::NotFullCommand);
    };

    let Some(name) = parts.next() else {
        return Err(ScriptLoaderError::NoMusic);
    };

    let command = match media_type {
        b"m" => Ok(Command::PlayMusic(to_box_str_unchecked(name))),
        b"s" => Ok(Command::PlaySound(to_box_str_unchecked(name))),
        b"e" => Ok(Command::PlayEffect(to_box_str_unchecked(name))),
        _ => Err(ScriptLoaderError::Custom(String::from("Unknown symbol after `play`. Must be either `m`, `s` or `e`")))
    };

    nodes.push_front(ScriptNode::Command(
        command?
    ));

    Ok(())
}

pub fn parse_set<'w>(
    nodes : &mut VecDeque<ScriptNode>,
    parts : &mut impl Iterator<Item=&'w [u8]>
) -> Result<(), ScriptLoaderError> {
    let Some(character) = parts.next() else {
        return Err(ScriptLoaderError::NotFullCommand);
    };

    let Some(state) = parts.next() else {
        return Err(ScriptLoaderError::NoState);
    };

    let Some(transition) = parts.next() else {
        return Err(ScriptLoaderError::NoTransition);
    };
    //Check if it is position. Otherwise must be some unique character state
    if let Ok(position) = parse_position(state) {
        nodes.push_front(ScriptNode::Command(
            Command::SetCharacterPosition(to_box_str_unchecked(character), position, parse_transition(transition)?)
        ));
    } else {
        nodes.push_front(ScriptNode::Command(
            Command::SetCharacter(to_box_str_unchecked(character), to_box_str_unchecked(state), parse_transition(transition)?)
        ));
    }

    

    Ok(())
}

pub fn parse_show<'w>(
    nodes : &mut VecDeque<ScriptNode>,
    parts : &mut impl Iterator<Item=&'w [u8]>
) -> Result<(), ScriptLoaderError> {
    let Some(peek) = parts.next() else {
        return Err(ScriptLoaderError::NotFullCommand);
    };

    if peek == b"bg" {
        return parse_show_bg(nodes, parts);
    }

    let character = peek;

    let Some(pos) = parts.next() else {
        return Err(ScriptLoaderError::NoPosition);
    };

    let Some(transition) = parts.next() else {
        return Err(ScriptLoaderError::NoTransition);
    };

    nodes.push_front(ScriptNode::Command(
        Command::ShowCharacter(to_box_str_unchecked(character), parse_position(pos)?, parse_transition(transition)?)
    ));

    Ok(())
}

pub fn parse_show_bg<'w>(
    nodes : &mut VecDeque<ScriptNode>,
    parts : &mut impl Iterator<Item=&'w [u8]>
) -> Result<(), ScriptLoaderError> {
    let Some(name) = parts.next() else {
        return Err(ScriptLoaderError::NoPosition);
    };

    let Some(transition) = parts.next() else {
        return Err(ScriptLoaderError::NoTransition);
    };

    nodes.push_front(ScriptNode::Command(Command::ShowBackground(to_box_str_unchecked(name), parse_transition(transition)?)));

    Ok(())
}

pub fn parse_hide<'w>(
    nodes : &mut VecDeque<ScriptNode>,
    parts : &mut impl Iterator<Item=&'w [u8]>
) -> Result<(), ScriptLoaderError> {
    let Some(character) = parts.next() else {
        return Err(ScriptLoaderError::NotFullCommand);
    };

    let Some(transition) = parts.next() else {
        return Err(ScriptLoaderError::NoMusic);
    };

    nodes.push_front(ScriptNode::Command(
        HideCharacter(to_box_str_unchecked(character), parse_transition(transition)?)
    ));

    Ok(())
}

pub fn parse_transition<'w>(
    name : &'w [u8]
) -> Result<TransitionOption, ScriptLoaderError> {
    match name {
        b"s" => Ok(TransitionOption::Smooth),
        b"i" => Ok(TransitionOption::Immidiate),
        _ => Err(ScriptLoaderError::UnknownTransition)
    }
}

pub fn parse_position<'w>(
    name : &'w [u8]
) -> Result<RelativePosition, ScriptLoaderError> {
    match name {
        b"c" => Ok(RelativePosition::Center),
        b"ob" =>  Ok(RelativePosition::OutOfBounds),
        b"lb" =>  Ok(RelativePosition::Left(Level::Bottom)),
        b"lc" =>  Ok(RelativePosition::Left(Level::Center)),
        b"ls" => Ok(RelativePosition::Left(Level::Side)),
        b"lt" =>  Ok(RelativePosition::Left(Level::Top)),
        b"rb" =>  Ok(RelativePosition::Left(Level::Bottom)),
        b"rc" =>  Ok(RelativePosition::Left(Level::Center)),
        b"rs" =>  Ok(RelativePosition::Left(Level::Side)),
        b"rt" =>  Ok(RelativePosition::Left(Level::Top)),
        _ => Err(ScriptLoaderError::UnknownPosition)
    }
}

pub fn parse_jump<'w>(
    nodes : &mut VecDeque<ScriptNode>,
    parts : &mut impl Iterator<Item=&'w [u8]>
) -> Result<(), ScriptLoaderError> {
    let Some(label_name) = parts.next() else {
        return Err(ScriptLoaderError::EmptyLabel);
    };

    let label = Label(to_box_str_unchecked(label_name));

    nodes.push_front(ScriptNode::Jump(
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

    nodes.push_front(ScriptNode::Choice{
        prompt,
        choices
    });

    nodes.push_front(ScriptNode::Pause);

    Ok(())
}

pub fn parse_stop<'w>(
    nodes : &mut VecDeque<ScriptNode>,
    _parts : &mut impl Iterator<Item=&'w [u8]>
) -> Result<(), ScriptLoaderError> {
    nodes.push_front(ScriptNode::Command(super::Command::StopMusic));

    Ok(())
}
