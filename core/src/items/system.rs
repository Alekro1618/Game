use engine::ecs::{observer::system_param::On, query, systems::query::Query};

use crate::items::{self, AddToInventory, Inventory, RemoveFromInventory};

pub fn on_add_to_inventory(
    on : On<AddToInventory>,
    query : Query<&mut Inventory>
) {
    let add_to_inventory = on.event();
    let AddToInventory(entity_id, item) = add_to_inventory;
    let Ok(mut inventory) = query.get_mut(*entity_id) else {
        panic!("No inventory")
    };
    inventory.add(item);
    println!("added {:#?} to {}", item, entity_id);
}

pub fn on_remove_from_inventory(
    on : On<RemoveFromInventory>,
    query : Query<&mut Inventory>
) {
    let remove_from_inventory = on.event();
    let RemoveFromInventory(entity_id, item) = remove_from_inventory;
    let Ok(mut inventory) = query.get_mut(*entity_id) else {
        panic!("No inventory")
    };
    inventory.remove(item);
    println!("removed {:#?} from {}", item, entity_id);
}