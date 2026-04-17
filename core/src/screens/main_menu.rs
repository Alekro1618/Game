use Game::screen;

use crate::engine::{graphics::ui::{manager::{UIManager, Widgets}, screen::Screen, widget::ButtonState}, id::{ WidgetId}, systems::system::System};


#[screen(name = "main_menu")]
#[derive(Clone)]
pub struct MainMenu {
    start_button : Option<WidgetId>
}

impl MainMenu {
    pub fn new() -> Self {
        Self { start_button: None }
    }
}

impl Screen for MainMenu {
    fn on_start(&mut self, ui : &mut Widgets, system : &mut System) {
        self.start_button = ui.find_by_name("Start");
    }

    fn update(&mut self, ui : &mut Widgets, system : &mut System, delta : f32) {
        
    }
}