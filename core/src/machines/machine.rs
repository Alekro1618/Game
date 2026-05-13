use engine::{ecs::entities::id::EntityId, utils::timer::Timer};
use engine_macros::{Component, Event};

use crate::{ machines::machine_type::MachineType};

/// 1) [`EntityId`] of [`Player`]
/// 2) [`EntityId`] of [`Machine`]
#[derive(Event)]
pub struct InteractWithMachine(pub EntityId, pub EntityId);

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
    pub machine_type : MachineType, 
}

#[derive(Component)]
pub struct MachineLink {
    pub actual_machine : EntityId,
    pub machine_type : MachineType, 
}


impl Machine {
    #[inline]
    pub fn new(is_holding : bool, take_time : Option<f32>, machine_type : MachineType) -> Self {
        Self {
            is_holding,
            take_time: take_time,
            machine_type
        }
    }

    #[inline]
    pub fn timer(&self) -> Option<Timer<()>> {
        Some(Timer::new(0.0, self.take_time?))
    }
}


impl From<MachineType> for Machine {
    fn from(value: MachineType) -> Self {
         match value {
            MachineType::CoffeeMachine => {
                Self::new(false, Some(4.0), MachineType::CoffeeMachine)
            },
            MachineType::RubbishCan => {
                Self::new(false, None, MachineType::RubbishCan)
            },
            MachineType::CoffeeBag => {
                Self::new(false, None, MachineType::CoffeeBag)
            },
            MachineType::CashRegister => {
                Self::new(true, None, MachineType::CashRegister)
            },
        }
    }
}