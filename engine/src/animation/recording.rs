

use serde::{Deserialize, Serialize};

use crate::{
    animation::{Animatable, media::AnimatableMedia, player::{Player}}, asset::{asset::Asset, handle::Handle},
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Frame<A> {
    time_stamp: f32,
    state: A,
}

impl<A : Clone> Frame<A> {
    pub fn new(time_stamp: f32, state: A) -> Self {
        Self { time_stamp, state }
    }

    pub fn time_stamp(&self) -> f32 {
        self.time_stamp
    }

    pub fn state(&self) -> A {
        self.state.clone()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Recording<A> {
    frames: Vec<Frame<A>>,
    pub length: f32,
}

impl<A> Default for Recording<A> {
    fn default() -> Self {
        Self {
            frames: Vec::new(),
            length: 0.0,
        }
    }
}

impl<A: Animatable> AnimatableMedia<A> for Recording<A> {
    type Deserializer = Recording<A>;

    fn load(deserializer : Self::Deserializer, _context : &mut crate::asset::loader::LoadContext) -> Self {
        deserializer
    }
}

impl<A: Send + Sync + 'static> Asset for Recording<A> {}

impl<A: Clone> Recording<A> {
    pub fn by_stamp(states: Vec<A>, rate: f32) -> Self {
        let diff = 1. / rate;
        let length = states.len() as f32 * diff;
        let frames = states
            .into_iter()
            .enumerate()
            .map(|(i, state)| Frame::new(i as f32 * diff, state))
            .collect();

        Self::new(frames, length)
    }

    pub fn new(frames: Vec<Frame<A>>, length: f32) -> Self {
        Self { frames, length }
    }

    pub fn current(&self, time: f32) -> Frame<A> {
        let frame = self
            .frames
            .iter()
            .position(|f| f.time_stamp < time)
            .unwrap();

        self.frames[frame].clone()
    }

    pub fn between(&self, time: f32) -> (Frame<A>, Option<Frame<A>>) {
        let frame = self
            .frames
            .iter()
            .position(|f| f.time_stamp < time)
            .unwrap();

        (self.frames[frame].clone(), self.frames.get(frame).cloned())
    }
}



pub struct RecordPlayer<A: Animatable> {
    recording: Option<Handle<Recording<A>>>,
    time: f32,
    premature_end : bool
}

impl<A: Animatable> RecordPlayer<A> {
    pub fn new(recording  : Option<Handle<Recording<A>>>) -> Self {
        Self {
            recording,
            time: 0.0,
            premature_end: false
        }
    }

    pub fn reset(&mut self) {
        self.time = 0.0;
        self.premature_end = false;
    }

    pub fn end(&mut self) {
        self.premature_end = true;
    }

    pub fn recording(&self) -> &Handle<Recording<A>> {
        self.recording.as_ref().unwrap()
    }

    pub fn advance(&mut self, delta_time: f32) -> f32 {
        self.time += delta_time;
        self.time
    }
}

impl<A: Animatable> Player for RecordPlayer<A> {
    type State = A;
    type Recording = Recording<A>;

    fn init(&mut self) {
        self.reset();
    }

    fn advance(&mut self, recording: &Recording<A>, delta_time: f32) -> Self::State {
        let time = RecordPlayer::<A>::advance(self, delta_time);
        let (current, next) = recording.between(time);

        let Some(next) = next else {
            return current.state();
        };

        Animatable::advance(current.state(), next.state(), time)
    }

    fn repeat(&mut self) {
        self.reset();
    }

    fn is_end(&self, recording: &Recording<A>) -> bool {
        self.time >= recording.length || self.premature_end
    }

    fn on_end(&mut self) {
        self.reset();
    }
    
    fn handle(&self) -> Option<&Handle<Self::Recording>> {
        self.recording.as_ref()
    }
    
    fn time(&self) -> f32 {
        self.time
    }
    
    fn length(&self, recording : &Self::Recording) -> f32 {
        recording.length
    }
    
    fn replace_handle(&mut self, recording : Handle<Self::Recording>) {
        self.reset();
        self.recording = Some(recording);
    }
}
