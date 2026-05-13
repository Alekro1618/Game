use engine::ecs::entities::id::EntityId;
use engine_macros::{Component, Event};
use crate::items::{Item};




///Component for `Machine` that describes what machine takes and what it may output. Output is wrapped in option to configure 2 results
/// - If items match with input then it returns `MachineItem` that may be `MachineItem::None`
/// - Otherwise may return none if 
#[derive(Component)]
pub struct MachineFunction
{
    pub input : Item,
    pub output : Item
}


impl MachineFunction {
    pub fn new(input : Item, output : Item) -> Self {
        Self {
            input,
            output
        }
    }
}

#[derive(Event)]
pub struct ChangeOutput(pub EntityId, pub Item);