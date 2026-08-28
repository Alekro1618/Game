use engine::{ecs::{change_detection::params::Mut, observer::system_param::On, systems::commands::Commands}, utils::*};

use crate::{items::{Item, food::Food::{Espresso, Latte}}, order::order::{CompleteOrder, CompletedOrder, FailedOrder, NewOrder, NewRandomOrder, Order, Orders}};

pub fn on_new_random_order(
    on : On<NewRandomOrder>,
    mut commands : Commands
) {
    let new_order = on.event();
    let entity = new_order.0;

    let order =  if rand::random_bool(0.5) {
        Order::new(Item::Food(Espresso), 20.0)
    } else {
        Order::new(Item::Food(Latte), 20.0)
    };

    commands.trigger(NewOrder(entity, order));
}


pub fn on_new_order(
    on : On<NewOrder>,
    mut orders : Mut<Orders>,
    mut commands : Commands
) {
    let new_order = on.event();
    let entity = new_order.0;
    let order = new_order.1;

    println!("new order placed for: {:#?}", order.item);
    
    let mut timer 
        = Timer::<FailedOrder>::new(Some(entity), 0.0, order.wait_time);
    timer.set_result(FailedOrder(entity, order));

    commands.spawn((timer, ));

    orders.add(entity, order);
}

pub fn on_completed_order(
    on : On<CompleteOrder>,
    mut commands : Commands,
    mut orders : Mut<Orders>
) {
    let completed = on.event();
    let CompleteOrder(item) = completed;
    let entity_id = orders.get_first_match(*item);

    let Some(entity_id) = entity_id else {
        return;
    };

    let order = orders.remove(entity_id).unwrap();

    
    println!("order completed for: {:#?}", order.item);
    
    commands.trigger(StopTimer::<FailedOrder>::new(Some(entity_id)));
    commands.trigger(CompletedOrder(entity_id, order));
}

pub fn on_order_failed(
    on : On<FailedOrder>,
    mut orders : Mut<Orders>,
) {
    let failed_order = on.event();

    println!("order failed");

    orders.remove(failed_order.0);
}