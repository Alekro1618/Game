use engine::{ecs::entities::id::EntityId, utils::*};
use engine_macros::{Component, Event};

use crate::{ items::Item, machines::{state::MachineState, machine_type::MachineType}};


/// `Machine`
#[derive(Component)]
pub struct Machine {
    pub is_holding : bool, // If true the charachter won't be able to move until the machine finishes
    pub take_time : Option<f32>, //Time in seconds
    pub fix_time : Option<f32>,
}

#[derive(Component)]
pub struct IsHolding;

#[derive(Component)]
pub struct TakesTime(f32);

#[derive(Component)]
pub struct FixTime(f32);


impl Machine {
    #[inline]
    pub fn new(is_holding : bool, fix_time : Option<f32>, take_time : Option<f32>) -> Self {
        Self {
            is_holding,
            fix_time,
            take_time: take_time,
        }
    }
}


impl From<MachineType> for Machine {
    fn from(value: MachineType) -> Self {
         match value {
            MachineType::CoffeeMachine => {
                Self::new(false, Some(2.0), Some(4.0))
            },
            MachineType::RubbishCan => {
                Self::new(false, None, None)
            },
            MachineType::CoffeeBag => {
                Self::new(false,  None, None)
            },
            MachineType::CashRegister => {
                Self::new(true, None, None)
            },
            MachineType::MilkStation => {
                Self::new(true, None, Some(4.0))
            },
            MachineType::Cutter => {
                Self::new(true, None, Some(4.0))
            },
            MachineType::Unknown => {
                Self::new(false, None, None)
            },
            MachineType::Table => {
                Self::new(false, None, None)
            },
        }
    }
}


#[derive(Component)]
pub struct WaitingFor(pub EntityId);