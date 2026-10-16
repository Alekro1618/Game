use std::fmt::Display;

use engine_macros::{Component, Reflect, reflect};
use serde::{Deserialize, Serialize};



#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Component, Default, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize, Component)]
pub enum MachineType {
    #[default]
    Unknown,
    CoffeeMachine,
    RubbishCan,
    CoffeeBag,
    CashRegister,
    MilkStation,
    Cutter,
    Table
}

impl Display for MachineType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MachineType::Unknown => write!(f, "Unknown"),
            MachineType::CoffeeMachine => write!(f, "Coffee machine"),
            MachineType::RubbishCan => write!(f, "Rubbish can"),
            MachineType::CoffeeBag => write!(f, "Coffee bag"),
            MachineType::CashRegister => write!(f, "Cash register"),
            MachineType::MilkStation => write!(f, "MilkStation"),
            MachineType::Cutter => write!(f, "Cutter"),
            MachineType::Table => write!(f, "Table"),
        }
    }
}