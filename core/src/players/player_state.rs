use engine_macros::Component;


#[derive(Component)]
pub enum PlayerState {
    /// Player is idle and does nothing. May hold item.
    Idle,
    /// Player is walking. May hold item. 
    Walking,
    /// Player is currently active with machine. Player that has this status should contain component [`WaitingFor`].
    Active,
    /// Player is currently repearing this machine. Player that has this status should contain component [`WaitingFor`].
    Repearing
}

impl PlayerState {
    pub fn set_idle(&mut self) {
        *self = Self::Idle;
    }

    pub fn set_walking(&mut self) {
        *self = Self::Walking;
    }

    pub fn set_active(&mut self) {
        *self = Self::Active;
    }

    pub fn set_repearing(&mut self) {
        *self = Self::Repearing;
    }

    pub fn is_idle(&self) -> bool {
        matches!(self, PlayerState::Idle)
    }

    pub fn is_active(&self) -> bool {
        matches!(self, PlayerState::Active)
    }

    pub fn is_walking(&self) -> bool {
        matches!(self, PlayerState::Walking)
    }

    pub fn is_repearing(&self) -> bool {
        matches!(self, PlayerState::Repearing)
    }
}