use engine_macros::{Component, Reflect, reflect};
use serde::{Deserialize, Serialize};


#[derive(Component, Clone, Copy, Default, PartialEq, Eq, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub enum MachineState {
    #[default]
    /// Machine is idle and any entity can interact with it using item or without. If machine doesn't take time and doesn't hold we don't change state and leave it to be. Will translate to either [`MachineState::Active`], [`MachineState::Idle`].
    Idle,
    /// Machine is currenyly active. If machine takes time we set this state. In this state in can either hold entity or not. The machine cannot be interrupted until it finishes with time.  Will translate to [`MachineState::Finished`].
    Active,
    /// Machine has finished it's task. Makes sense only if machine takes time. On interact will give ouput. Will translate to [`MachineState::Idle`].
    Finished,
    /// Machine has been disrupted. Other entitiy should come and fix it (may take time depending on machine type). Will translate to either [`MachineState::Repearing`] or [`MachineState::Idle`].
    Disrupted,
    /// Machine is being repeaired. Makes sense only if machine takes time to repeair. Will translate to [`MachineState::Idle`].
    Repearing
}

impl MachineState {
    pub fn set_idle(&mut self) {
        *self = Self::Idle;
    }

    pub fn set_active(&mut self) {
        *self = Self::Active;
    }

    pub fn set_finished(&mut self) {
        *self = Self::Finished;
    }

    pub fn set_disrupted(&mut self) {
        *self = Self::Disrupted;
    }

    pub fn set_repearing(&mut self) {
        *self = Self::Repearing;
    }

    pub fn is(&self, state: Self) -> bool {
        self == &state
    }
}