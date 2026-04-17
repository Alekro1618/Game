use Game::component;
use raylib::{ffi::{MouseButton}, math::{Vector2}};
use crate::{core::{algos::path_finding::path, objects::player::PlayerState}, engine::{components::component::Component, input::manager::InputManager, objects::object::{Object, ObjectActions}, prefabs::manager::ObjectManager, systems::{params::{self, ParamsActions}, system::{self, System}}}};

#[component(name = "controller")]
#[derive(Clone)]
pub struct PlayerController {
    start : (usize, usize),
    path : Vec<(usize, usize)>,
    time : f32,
    chosen : bool,
}

impl PlayerController {
    pub fn new() -> Self {
        Self {
            start : (0, 0),
            path : Vec::new(),
            time : 0.0,
            chosen : false,
        }
    }
}

impl Component for PlayerController {
    fn on_start(&mut self, object : &mut Object, other : &mut ObjectManager, system : &mut System) {
        system.params.set_bool(object.name().to_string(), true);
        self.start = system.params.grid.what_cell(object.pos());
        system.params.grid.set_special(object.name(), self.start);

        let walls = other
            .find_all_by_tag("block");
        
        for wall in walls {
            let point = system.params.grid.what_cell(wall.obj().pos());
            system.params.grid.add_wall(point);
        }
    }
    
    fn on_input(&mut self, object : &mut Object, input : &InputManager, other : &mut ObjectManager, system : &mut System) {
        if input.mouse_pressed(MouseButton::MOUSE_BUTTON_LEFT)  {
            if self.chosen {
                let grid_input = system.params.grid.what_cell(input.mouse_pos());
                self.path = path(&system.params.grid, self.start, grid_input);
                if !self.path.is_empty() {
                    self.chosen = false;
                }
            }
                
            if object.rect().check_collision_point_rec(input.mouse_pos()) {
                self.chosen = !self.chosen;
            }
        }
    }

    fn update(&mut self, object : &mut Object, other : &mut ObjectManager, system : &mut System, delta : f32) {        
        let next_node = if self.time < 100.0 {
            if let Some(node) = self.path.first() {
                if !system.params.grid.is_special(node) {
                    self.time += delta * 500.0;
                } else {
                    object.set_state_t(PlayerState::Wait);
                }
            }
            self.path.first()
        } else {
            self.time = 0.0;         
            if self.path.len() > 0 {
                let point = self.path.get(0).unwrap().clone();
                system.params.grid.set_special(object.name(), point);

                self.start = point;
                self.path.remove(0);
            }
            self.path.first()
        };

        if let Some(node) = next_node {
            if node.0 > self.start.0 {
                object.set_state_t(PlayerState::Left);
            } else if node.0 < self.start.0  {
                object.set_state_t(PlayerState::Right);
            } else if node.1 < self.start.1 {
                object.set_state_t(PlayerState::Up);
            } else if node.1 > self.start.1 {
                object.set_state_t(PlayerState::Down);
            }
            
            object.set_pos_v(system.params.grid.translate(&self.start, node, self.time));
        } else {
            if object.state() == PlayerState::Stop {
                object.set_state_t(PlayerState::Idle);
            } else if object.state() != PlayerState::Idle {
                object.set_state_t(PlayerState::Stop);
            }
        }
    }
}