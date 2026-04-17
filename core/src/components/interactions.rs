use std::rc::Rc;

use Game::component;
use raylib::{ffi::{KeyboardKey, MouseButton}, math::Vector2};
use crate::{core::algos::{mouse_gesture::mouse_shaking, path_finding::path}, engine::{components::component::{self, Component, ComponentClone}, input::manager::InputManager, objects::object::{GameObject, GameObjectRef, Object, ObjectActions}, prefabs::manager::ObjectManager, systems::system::System, utils::grid::Grid}};

#[component(name = "interaction")]
#[derive(Clone)]
pub struct Interaction {
    last_mouth_pos : Vector2,
    pointer : f32,
    mouse_path : Vec<Vector2>,
    timer : f32
}

impl Interaction {
    pub fn new() -> Self {
        Self {
            last_mouth_pos : Vector2::zero(),
            pointer : -1.0,
            mouse_path : Vec::new(),
            timer : 0.0
        }
    }
}

impl Component for Interaction {
    fn update(&mut self, object : &mut Object, other : &mut ObjectManager, system : &mut System, delta : f32) {
        if self.timer < 0.02 {
            self.timer += delta;
        } else {
            self.mouse_path.push(self.last_mouth_pos);
            if self.mouse_path.len() > 5 {
                self.mouse_path.remove(0);
            }
            self.timer = 0.0;
        }
    }

    fn on_input(&mut self, object : &mut Object, input : &InputManager, other : &mut ObjectManager, system : &mut System) {
        self.last_mouth_pos = input.mouse_pos();
        let value = mouse_shaking(&self.mouse_path);
        self.pointer += 0.5 * value;
        self.pointer = self.pointer.clamp(-1.0, 1.0);

        // if self.pointer > 0.93 {
        //     object.set_state("3");
        // } else if self.pointer > 0.71 {
        //     object.set_state("2");
        // } else if self.pointer > 0.38 {
        //     object.set_state("1");
        // } else if self.pointer > 0.0 {
        //     object.set_state("0");
        // } else if self.pointer > -0.38 {
        //     object.set_state("-1");
        // } else if self.pointer > -0.71 {
        //     object.set_state("-2");
        // } else {
        //     object.set_state("-3");
        // }
    }
}
