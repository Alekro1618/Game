use engine::{ecs::{change_detection::params::Mut, entities::id::EntityId, observer::system_param::On, systems::commands::{self, Commands}}, utils::*};

use crate::{order::order::{FailedOrder, NewOrder, Order, Orders}, players::player::Command};


pub fn on_new_order(
    on : On<NewOrder>,
    mut orders : Mut<Orders>,
    mut commands : Commands
) {
    let new_order = on.event();
    let entity = new_order.0;
    let order = new_order.1;
    
    let mut timer 
        = Timer::<Order, FailedOrder>::new(0.0, order.wait_time);
    timer.set_result(FailedOrder(entity, order));

    commands.spawn((timer, ));

    orders.add(entity, order);
}

pub fn on_order_time(
    on : On<OnTimerFinished<Order, FailedOrder>>,
    mut commands : Commands,
    mut orders : Mut<Orders>,
) {
    let timer_finished = on.event();
    let Some(failed_order) = timer_finished.result else {
        panic!("");
    };
    
    orders.remove(failed_order.0);
    commands.trigger(failed_order);

}