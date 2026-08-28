use std::marker::PhantomData;

use engine::{app::plugin::Plugin, ecs::{entities::id::EntityId, observer::system_param::On, systems::{commands::Commands}}, utils::*};

use crate::order::{order::{CompleteOrder, FailedOrder, NewOrder, NewRandomOrder, Order, Orders}, system::{on_completed_order, on_new_order, on_new_random_order, on_order_failed}};

pub mod order;
mod system;

pub struct OrderPlugin;

impl Plugin for OrderPlugin {
    fn build(self, app:  &mut engine::app::App) {
        app
            .add_plugin(TimerPlugin::<FailedOrder>::new())
            .add_observer(
                on_new_order
            )
            .add_observer(
                on_new_random_order
            )
            .add_observer(
                on_completed_order
            )
            .add_observer(
                on_order_failed
            );
    }
}