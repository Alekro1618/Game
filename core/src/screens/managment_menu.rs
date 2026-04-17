use Game::screen;

use crate::{core::structures::{food::Food, player::Player}, engine::{graphics::ui::{manager::{UIManager, Widgets}, screen::Screen, widget::ButtonState}, id::WidgetId, input::manager::InputManager, systems::{params::ParamsActions, system::{self, System}}}};


#[screen(name = "managment_menu")]
#[derive(Clone)]
pub struct ManagmentMenu {
    cell_1 : Option<WidgetId>,
    cell_2 : Option<WidgetId>,
    cell_3 : Option<WidgetId>,
    cell_4 : Option<WidgetId>,
}

impl ManagmentMenu {
    pub fn new() -> Self {
        Self { 
            cell_1 : None,
            cell_2 : None,
            cell_3 : None,
            cell_4 : None
        }
    }
}

impl Screen for ManagmentMenu {
    fn on_start(&mut self, ui : &mut Widgets, system : &mut System) {
        self.setup_ids(ui, system);
    }

    fn update(&mut self, ui : &mut Widgets, system : &mut System, delta : f32) {
        self.update(ui, system);
    }

    fn on_input(&mut self, input : &InputManager, ui : &mut Widgets, system : &mut System) {
        
    }
}


impl ManagmentMenu {
    fn setup_ids(&mut self, ui : &mut Widgets, system : &mut System) {
        self.cell_1 = ui.find_by_name("cell_1");
        self.cell_2 = ui.find_by_name("cell_2");
        self.cell_3 = ui.find_by_name("cell_3");
        self.cell_4 = ui.find_by_name("cell_4");

    }

    fn update(&mut self, ui : &mut Widgets, system : &mut System) {
        let mut index = 0;
        for player in Player::all() {
            let name = player.to();

            if system.get_bool(name) != Some(&true) {
                continue;
            }

            index += 1;

            let cell_id = match index {
                1 => self.cell_1,
                2 => self.cell_2,
                3 => self.cell_3,
                4 => self.cell_4,
                _ => None,
            };

            let cell = ui
                .get_multi_mut(cell_id);
            
            let cell_icon = ui
                .get_image_mut(cell_id);

            cell_icon.state = name.to_string();

            if let Some(food) = system.get_str(&(name.to_owned() + "_inv")) {
                let cell_food = ui
                    .get_image_mut(cell_id);


                cell_food.state = food.clone();
            }
        }
    }
}