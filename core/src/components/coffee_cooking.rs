use Game::component;
use crate::{core::objects::blocks::CoffeeMachineState, engine::{components::component::Component, objects::object::{GameObject, GameObjectRef, Object, ObjectActions}, prefabs::manager::ObjectManager, systems::system::System, utils::grid::Grid}};

#[component(name = "coffee_cooking")]
#[derive(Clone)]
pub struct CoffeeCooking {
    timer : f32,
    started : bool
}

impl CoffeeCooking {
    pub fn new() -> Self {
        Self {
            timer : 0.0,
            started : false
        }
    }
}

impl Component for CoffeeCooking {
    fn update(&mut self, object : &mut Object, other : &mut ObjectManager, system : &mut System, delta : f32) {
        if object.state() == CoffeeMachineState::Making && self.timer == 0.0 {
            self.timer = 2.0;
            self.started = true;
        }

        if self.started {
            self.timer -= delta;
            if self.timer <= 0.0 {
                self.timer = 0.0;
                self.started = false;
                object.set_state_t(CoffeeMachineState::Ready);
            }
        }
    }
}
