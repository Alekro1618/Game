use engine::{asset::{AssetServer, asset::{Asset, Assets}}, ecs::{change_detection::params::Mut, entities::id::EntityId, message::message_writer::{self, MessageWriter}, name::Name, observer::system_param::On, query::filter::With, systems::{commands::Commands, query::Query}}, utils::Either::{self, Left, Right}};

use crate::cutscenes::{actor::Actor, script::{CurrentScript, Script, ScriptNodeMessage, events::AdvanceScript, reader::ScriptReader, script_state::ScriptState}};


pub fn script_is_advance(
    script_state : &ScriptState,
) -> bool {
    script_state.is_advance()
}

pub fn advance_script(
    script : &CurrentScript,
    mut script_state : Mut<ScriptState>,
    mut script_reader : Mut<ScriptReader>,
    mut message_writer: MessageWriter<ScriptNodeMessage>,
    actors : Query<(EntityId, &Name), With<Actor>>,
    asset_server: &AssetServer
) {
    let Some(ref script) = script.1 else {
        return;
    };

    *script_state = match script_reader.advance(script, &mut message_writer, actors, asset_server) {
        super::reader::ReaderWait::WaitingForAdvance => {
           ScriptState::WaitingForAdvance
        },
        super::reader::ReaderWait::WaitingForChoice => {
            ScriptState::WaitForChoice
        },
        super::reader::ReaderWait::EndOfScript => {
            ScriptState::EndOfScript
        },
        super::reader::ReaderWait::NeedsLoading => {
            ScriptState::Advance
        },
    }
}


pub fn retrive_loaded_script(
    mut assets : Mut<Assets<Script>>,
    mut current : Mut<CurrentScript>
) {
    if let Some(ref handle) = current.0 {
        if let Some(s) = assets.remove(handle.index()) {
            current.1 = Some(s);
        }
    }
}

pub fn wants_to_load(
    current : &CurrentScript
) -> bool {
    current.0.is_some()
}