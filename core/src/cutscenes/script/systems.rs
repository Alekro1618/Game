use engine::{asset::AssetServer, ecs::{change_detection::params::Mut, entities::id::EntityId, message::message_writer::{self, MessageWriter}, name::Name, observer::system_param::On, query::filter::With, systems::{commands::Commands, query::Query}}};

use crate::cutscenes::{actor::Actor, script::{CurrentScript, ScriptNodeMessage, events::AdvanceScript, reader::ScriptReader, script_state::ScriptState}};



pub fn advance_script(
    script : &CurrentScript,
    mut script_state : Mut<ScriptState>,
    mut script_reader : Mut<ScriptReader>,
    mut message_writer: MessageWriter<ScriptNodeMessage>,
    actors : Query<(EntityId, &Name), With<Actor>>,
    asset_server: &AssetServer
) {
    if !script_state.is_advance() {
        return;
    }
    
    *script_state = match script_reader.advance(&script.0, &mut message_writer, actors, asset_server) {
        super::reader::ReaderWait::WaitingForAdvance => {
           ScriptState::WaitingForAdvance
        },
        super::reader::ReaderWait::WaitingForChoice => {
            ScriptState::WaitForChoice
        },
        super::reader::ReaderWait::EndOfScript => {
            ScriptState::EndOfScript
        },
    }
}

pub fn on_advance(
    _ : On<AdvanceScript>,
    mut script_state : Mut<ScriptState>,
) {
    script_state.advance();
}