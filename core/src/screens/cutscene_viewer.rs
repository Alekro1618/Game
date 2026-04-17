use std::{f32::DIGITS, mem};

use Game::screen;
use raylib::{ffi::{KeyboardKey, MouseButton}};

use crate::engine::{graphics::ui::{manager::Widgets, screen::Screen, widget::WidgetActions}, id::WidgetId, input::manager::InputManager, systems::system::System, utils::cutscene::{Action, ScenePos}};

static TEXT_SPEED : f32 = 100.0;

#[screen(name = "dialog_menu")]
#[derive(Clone)]
pub struct DialogMenu {
    actors : Vec<WidgetId>,
    dialog_text : Option<WidgetId>,
    name_label : Option<WidgetId>,
    bg_image : Option<WidgetId>,
    text_buffer : String,
    current_name : String,
    current_cg : String,
    is_pause : bool,
    is_animation : bool,
    actors_hidden : bool,
    timer : f32
}

impl DialogMenu {
    pub fn new() -> Self {
        Self {
            actors: Vec::new(),
            text_buffer: String::new(),
            dialog_text: None,
            bg_image: None,
            current_name : String::new(),
            current_cg : String::new(),
            name_label: None,
            is_animation : false,
            is_pause : false,
            actors_hidden : false,
            timer : 0.0,
        }
    }
}

impl Screen for DialogMenu {
    fn on_start(&mut self, ui : &mut Widgets, system : &mut System) {
        self.actors = ui.find_by_pred(|w| w.wid().name().contains("actor"));
        self.name_label = ui.find_by_name("name_label");
        self.dialog_text = ui.find_by_name("dialog_text");
        self.bg_image = ui.find_by_name("background_image");
    }

    fn on_input(&mut self, input : &InputManager, ui : &mut Widgets, system : &mut System) {
        if input.mouse_released(MouseButton::MOUSE_BUTTON_LEFT) 
            || input.key_down(KeyboardKey::KEY_ENTER) {
            let dialog_text = ui
                .get_label_mut(self.dialog_text);

            dialog_text.text.value = String::new();
            self.is_pause = false;
        }
    }

    fn update(&mut self, ui : &mut Widgets, system : &mut System, delta : f32) {
        if !self.actors_hidden {
            self.update_actors(ui, system, delta);
        }

        if !self.text_buffer.is_empty() {
            self.timer += TEXT_SPEED / delta;
            if self.timer > 1000.0 {
                self.update_dialog_box(ui, system, delta);
                self.timer = 0.0;
            }
        }

        if !self.current_name.is_empty() {
            self.update_name_label(ui, system, delta);
            self.current_name = String::new();
        }

        if !self.current_cg.is_empty() {
            self.update_background(ui, system, delta);
            self.current_cg = String::new();
        }

        if  self.is_pause || 
            self.is_animation || 
            self.text_buffer.is_empty() {
            return;
        }

        self.continue_with_next_action(ui, system, delta);
    }
}

impl DialogMenu {
    fn update_dialog_box(&mut self, ui : &mut Widgets, system : &mut System, delta : f32) {
        let dialog_text = ui
            .get_label_mut(self.dialog_text);

        if let Some(first_char) = self.text_buffer.chars().next() {
            dialog_text
                .text
                .value
                .push(first_char);
        }      
    }

    fn update_background(&mut self, ui : &mut Widgets, system : &mut System, delta : f32) {
        let bg_image = ui
            .get_image_mut(self.bg_image);

        if let Some(texture) = system.get_ref_texture() {
            bg_image.texture = texture;
        }
    }

    fn update_name_label(&mut self, ui : &mut Widgets, system : &mut System, delta : f32) {
        let name_label = ui
            .get_label_mut(self.name_label);

        name_label.text.value = mem::take(&mut self.current_name);
    }

    fn update_actors(&mut self, ui : &mut Widgets, system : &mut System, delta : f32) {
        for actor in system.params.cutscene.actors.iter() {
            let actor_id = self.actors[actor.index];

            let actor_holder = ui
                .get_mut(actor_id)
                .unwrap()
                .wid_mut();
            
            let x = match actor.pos {
                ScenePos::LeftBottom => 0.1,
                ScenePos::LeftMiddle => 0.2,
                ScenePos::LeftTop => 0.3,
                ScenePos::Middle => 0.5,
                ScenePos::RightBottom => 0.7,
                ScenePos::RightMiddle => 0.8,
                ScenePos::RightTop => 0.9,
            };

            actor_holder.set_pos(x, 0.7);

            let actor_holder_image = actor_holder
                .as_one_mut()
                .w_type_mut()
                .as_image_mut();

            actor_holder_image.state = actor.state.clone();
        }
    }

    fn continue_with_next_action(&mut self, ui : &mut Widgets, system : &mut System, delta : f32) {
        if let Some(action) = system.params.cutscene.next() {
            match action {
                Action::State(_, _) => {
                    self.is_animation = true;
                },
                Action::Text(name, text) => {
                    self.current_name = name;
                    self.text_buffer = text;
                },
                Action::CG(name) => {
                    system.request_new_texture(&name);
                    self.actors_hidden = true;
                    self.current_cg = name;
                },
                Action::BG(name) => {
                    system.request_new_texture(&name);
                    self.actors_hidden = false;
                    self.current_cg = name;
                },
                Action::PlaySFX(name) => {
                    system.request_sound(&name);
                },
                Action::SetMusic(name) => {
                    system.request_music(&name);
                },
                Action::Pause => self.is_pause = true,
                Action::Multiple(_) => panic!("Can't process multiple actions"),
            }
        }
    }
}

