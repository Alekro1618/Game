use crate::{animation::{Animatable, media::AnimatableMedia, recording::{RecordPlayer, Recording}, sprite::{TextureRecording, TextureRecordingPlayer}}, asset::handle::Handle, ecs::components::{Component, Mutable}};


pub trait Player : Send + Sync + 'static {
    type State : Animatable;
    type Recording : AnimatableMedia<Self::State>;

    fn init(&mut self);
    fn advance(&mut self, recording : &Self::Recording, delta_time : f32) -> Self::State;
    fn repeat(&mut self);
    fn is_end(& self, recording : &Self::Recording) -> bool;
    fn on_end(&mut self);
    fn handle(&self) -> Option<&Handle<Self::Recording>>;
    fn time(&self) -> f32;
    fn length(&self, recording : &Self::Recording) -> f32;
    fn replace_handle(&mut self, recording : Handle<Self::Recording>);
}

pub struct PlayerInstance<P : Player> {
    pub player : P,
    pub is_end : bool,
}


impl<P : Player> Component for PlayerInstance<P> {
    type Mutability = Mutable;
}

impl<P : Player> PlayerInstance<P> {
    pub fn is_end(&self) -> bool {
        self.is_end
    }
}

pub fn recording_player<A : Animatable>() -> PlayerInstance<RecordPlayer<A>> {
    PlayerInstance {
        player: RecordPlayer::<A>::new(None), 
        is_end: false
    }
}

pub fn texture_recording_player() -> PlayerInstance<TextureRecordingPlayer> {
    PlayerInstance {
        player: TextureRecordingPlayer::new(None), 
        is_end: false
    }
}