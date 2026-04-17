use std::{collections::HashMap};

use Game::component;
use crate::engine::{components::component::Component, graphics::animation::manager::Track,  objects::object::{Object, ObjectActions}, prefabs::manager::ObjectManager, systems::system::System};

#[component(name = "drag_animation")]
#[derive(Clone)]
pub struct DragAnimation {
    track : Track
}

impl DragAnimation {
    pub fn new() -> Self {
        Self {
            track : Track::empty()
        }
    }
}

impl Component for DragAnimation {
    fn on_start(&mut self, object : &mut Object, other : &mut ObjectManager, system : &mut System) {
        let frame_size = object.texture_frame_size();
        let sprite_size = object.texture_size();

        self.track = Track::new(frame_size, sprite_size, 0, 5, 1.0);
    }
    
    fn update(&mut self, object : &mut Object, other : &mut ObjectManager, system : &mut System, delta : f32) {
        let target = self
            .track
            .frame(
                object
                .state()
            );

        object.set_target(target);
    }
}