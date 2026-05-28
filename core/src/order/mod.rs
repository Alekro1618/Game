use std::marker::PhantomData;

use engine::{app::plugin::Plugin, ecs::{entities::id::EntityId, observer::system_param::On, systems::{commands::Commands, function_system::F3}}, utils::*};

use crate::order::{order::{FailedOrder, NewOrder, Order, Orders}, system::on_new_order};

pub mod order;
mod system;

pub struct OrderPlugin;

impl Plugin for OrderPlugin {
    fn build(self, app:  &mut engine::app::App) {
        app
            .add_plugin(TimerPlugin::<Order, FailedOrder>(PhantomData))
            .add_observer::<NewOrder, F3<On<NewOrder>, &mut Orders, Commands>>(
                on_new_order
            );
    }
}