use crate::{core::components::{client_controller::ClientController, client_manager::ClientManager, coffee_cooking::CoffeeCooking, display_inventory::DisplayInventory, draging_animation::DragAnimation, interactions::Interaction, player_animation::Animation, player_controller::PlayerController, player_interact::PlayerInteract}, engine::game::manager::Game};

pub trait Components {
    fn add_components(&mut self) -> &mut Self;
}

impl Components for Game {
    fn add_components(&mut self) -> &mut Self {
        self
            .add_component(ClientController::new())
            .add_component(ClientManager::new())
            .add_component(CoffeeCooking::new())
            .add_component(DisplayInventory::new())
            .add_component(DragAnimation::new())
            .add_component(Interaction::new())
            .add_component(Animation::new())
            .add_component(PlayerController::new())
            .add_component(PlayerInteract::new())
    }
}
