use engine::{math::{transform::Transform, vec::Vec2}, render::sprite::Sprite};
use engine_macros::Bundle;

use crate::{actions::components::{ActionQueue, CurrentAction}, items::Inventory, machines::{machine::Machine, functions::MachineFunction, state::MachineState, machine_type::MachineType}, players::{player::{Player, PlayerSpeed}, player_state::PlayerState}, utils::grid::{GridName, SmoothToGrid}};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player : Player,
    pub smooth_to_grid : SmoothToGrid,
    pub player_state : PlayerState,
    pub player_speed : PlayerSpeed,
    pub transform : Transform,
    pub sprite : Sprite,
    pub inventory : Inventory,
    pub current_action : CurrentAction,
    pub action_queue : ActionQueue
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self { 
            player: Default::default(),
            smooth_to_grid: SmoothToGrid::new(GridName::Players, (0,0), (0,0)), 
            player_state: Default::default(), 
            player_speed: Default::default(), 
            transform: Transform::IDENTITY, 
            sprite: Sprite::none(), 
            inventory: Default::default(), 
            current_action: Default::default(), 
            action_queue: Default::default()
        }
    }
}

#[derive(Bundle)]
pub struct MachineBundle {
    pub machine : Machine,
    pub machine_type : MachineType,
    pub state :  MachineState,
    pub function : MachineFunction,
    pub current_action : CurrentAction,
    pub action_queue : ActionQueue
}

impl Default for MachineBundle {
    fn default() -> Self {
        Self {
            machine: Machine::new(false, None, None), 
            machine_type: Default::default(), 
            state: Default::default(),
            function: Default::default(),
            current_action: Default::default(),
            action_queue: Default::default() 
        }
    }
}