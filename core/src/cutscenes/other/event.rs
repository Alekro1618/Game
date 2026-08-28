use engine::{asset::handle::Handle, image::Image};
use engine_macros::Event;

#[derive(Event)]
pub struct BackgroundTransition(pub Handle<Image>);