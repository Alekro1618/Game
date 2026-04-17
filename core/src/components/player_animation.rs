use std::{collections::HashMap};

use Game::component;
use crate::engine::{components::component::Component, graphics::animation::manager::Track,  objects::object::{Object, ObjectActions}, prefabs::manager::ObjectManager, systems::system::System};

#[component(name = "animation")]
#[derive(Clone)]
pub struct Animation {
    tracks : HashMap<String, Track>,
    current_track : String
}

impl Animation {
    pub fn new() -> Self {
        Self {
            tracks : HashMap::new(),
            current_track : String::new()
        }
    }
}

impl Component for Animation {
    fn on_start(&mut self, object : &mut Object, other : &mut ObjectManager, system : &mut System) {
        let frame_size = object.texture_frame_size();
        let sprite_size = object.texture_size();

        let up_track = Track::new(frame_size, sprite_size, 0, 0, 1.0);
        let left_track = Track::new(frame_size, sprite_size, 1, 1, 1.0);
        let down_track = Track::new(frame_size, sprite_size, 2, 2, 1.0);
        let right_track = Track::new(frame_size, sprite_size, 3, 3, 1.0);
        let idle_track = Track::new(frame_size, sprite_size, 4, 5, 1.0);

        self.tracks.insert("walk_up".into(), up_track);
        self.tracks.insert("walk_left".into(), left_track);
        self.tracks.insert("walk_down".into(), down_track);
        self.tracks.insert("walk_right".into(), right_track);
        self.tracks.insert("idle".into(), idle_track);
        self.current_track = "idle".into();
    }
    
    fn update(&mut self, object : &mut Object, other : &mut ObjectManager, system : &mut System, delta : f32) {
        if let Some(target) = self.tracks.get_mut(&self.current_track) {
            object.set_target(target.next());
            if target.is_end() {
                self.current_track = object.state().to_string();
            }
        }    
    }
}