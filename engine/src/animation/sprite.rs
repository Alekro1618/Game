
use serde::{Deserialize, Serialize};

use crate::{
    animation::{Animatable, media::AnimatableMedia, player::Player}, asset::{AssetPath, asset::Asset, handle::Handle}, image::Image, math::rect::Rect, render::color::Color,
};


#[derive(Clone, Serialize, Deserialize)]
pub struct TextureFrame {
    time_stamp: f32,
    state : TextureState
}

impl TextureFrame {
    pub fn new( state: TextureState, time_stamp : f32) -> Self {
        Self { state, time_stamp  }
    }
}


#[derive(Clone, Serialize, Deserialize)]
pub struct TextureState {
    pub rect : Rect,
    #[serde(skip)]
    pub image : Option<Handle<Image>>
}

impl TextureState {
    pub fn new( rect : Rect, image : Option<Handle<Image>>) -> Self {
        Self { rect, image  }
    }
}


impl Animatable for TextureState {
    fn advance(self, _rhs: Self, _time: f32) -> Self {
        self
    }
}

pub struct TextureRecording {
    image : Handle<Image>,
    frames : Vec<TextureFrame>,
    length : f32
}

impl Asset for TextureRecording {}

impl TextureRecording {
    pub fn new(image : Handle<Image>, frames: Vec<TextureFrame>, length: f32) -> Self {
        Self { image, frames, length }
    }

    pub fn current(&self, time: f32) -> TextureFrame {
        let frame = self
            .frames
            .iter()
            .position(|f| f.time_stamp < time)
            .unwrap();

        self.frames[frame].clone()
    }
}


#[derive(Serialize, Deserialize)]
pub struct TextureRecordingDeserializer {
    image : AssetPath,
    frames : Vec<TextureFrame>,
    length : f32
}

impl TextureRecordingDeserializer {
    pub fn new(
        image : AssetPath,
        frames : Vec<TextureFrame>,
        length : f32
    ) -> Self 
    {
        Self { image, frames, length }
    }
}

impl AnimatableMedia<TextureState> for TextureRecording {
    type Deserializer = TextureRecordingDeserializer;

    fn load(deserializer : Self::Deserializer, context : &mut crate::asset::loader::LoadContext) -> Self {
        Self { 
            //Here we delegate the actual sprite loading to the user whenever he needs it
            image: context.asset_server.prepare_without_loading(deserializer.image), 
            frames: deserializer.frames, 
            length: deserializer.length
        }
    }
}


pub struct TextureRecordingPlayer {
    recording: Option<Handle<TextureRecording>>,
    time: f32,
    premature_end : bool
}

impl TextureRecordingPlayer {
    pub fn new(recording  : Option<Handle<TextureRecording>>) -> Self {
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

    pub fn recording(&self) -> &Handle<TextureRecording> {
        self.recording.as_ref().unwrap()
    }

    pub fn advance(&mut self, delta_time: f32) -> f32 {
        self.time += delta_time;
        self.time
    }
}


impl Player for TextureRecordingPlayer {
    type State = TextureState;
    type Recording = TextureRecording;

    fn init(&mut self) {
        self.reset();
    }

    fn advance(&mut self, recording: &TextureRecording, delta_time: f32) -> Self::State {
        let time = TextureRecordingPlayer::advance(self, delta_time);
        let current = recording.current(time);
        let mut state = current.state;
        state.image = Some(recording.image.clone());

        return state
    }

    fn repeat(&mut self) {
        self.reset();
    }

    fn is_end(&self, recording: &TextureRecording) -> bool {
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
