use engine_macros::Resource;

#[derive(Resource, Default)]
pub enum ScriptState {
    #[default]
    Advance,
    WaitingForAdvance,
    WaitForChoice,
    EndOfScript,
}

impl ScriptState {
    pub fn advance(&mut self) {
        *self = ScriptState::Advance
    }

    pub fn is_advance(&self) -> bool {
        matches!(self, ScriptState::Advance)
    }
}