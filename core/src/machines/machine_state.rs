use engine_macros::Component;

#[derive(Component)]
pub enum MachineState {
    Idle,
    Active,
    Finished
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
}