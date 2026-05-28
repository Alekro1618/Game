use engine::{ecs::entities::id::EntityId, utils::*};
use engine_macros::{Component, Event};

use crate::{ items::Item, machines::machine_type::MachineType};

/// 1) [`EntityId`] of [`Player`]
/// 2) [`EntityId`] of [`Machine`]
#[derive(Event)]
pub struct InteractWithMachine(pub EntityId, pub EntityId, pub Option<Item>);

/// 1) [`EntityId`] of [`Player`]
/// 2) [`EntityId`] of [`Machine`]
#[derive(Event)]
pub struct DisruptMachine(pub EntityId, pub EntityId);

/// 1) [`EntityId`] of [`Machine`]
#[derive(Event)]
pub struct OnFinishedMachine(pub EntityId);

/// Component to set entity waiting for [`Machine`]
#[derive(Component)]
pub struct WaitingFor(pub EntityId);

/// `Machine`
#[derive(Component)]
pub struct Machine {
    pub is_holding : bool, // If true the charachter won't be able to move until the machine finishes
    pub take_time : Option<f32>, //Time in seconds
    pub fix_time : Option<f32>,
    pub machine_type : MachineType,
}


impl Machine {
    #[inline]
    pub fn new(is_holding : bool, fix_time : Option<f32>, take_time : Option<f32>, machine_type : MachineType) -> Self {
        Self {
            is_holding,
            fix_time,
            take_time: take_time,
            machine_type
        }
    }

    #[inline]
    pub fn timer(&self) -> Option<Timer<Machine, EntityId>> {
        Some(Timer::new(0.0, self.take_time?))
    }
}


impl From<MachineType> for Machine {
    fn from(value: MachineType) -> Self {
         match value {
            MachineType::CoffeeMachine => {
                Self::new(false, Some(2.0), Some(4.0), MachineType::CoffeeMachine)
            },
            MachineType::RubbishCan => {
                Self::new(false, None, None, MachineType::RubbishCan)
            },
            MachineType::CoffeeBag => {
                Self::new(false,  None, None, MachineType::CoffeeBag)
            },
            MachineType::CashRegister => {
                Self::new(true, None, None, MachineType::CashRegister)
            },
            MachineType::Fridge => {
                Self::new(false, None, None, MachineType::Fridge)
            },
            MachineType::Cutter => {
                Self::new(true, None, Some(4.0), MachineType::Cutter)
            },
        }
    }
}