use engine::{app::{main_schedule::ScheduleLabel, plugin::Plugin}, ecs::{entities::id::EntityId, observer::system_param::On, systems::{commands::Commands, query::Query}, worlds::World}};

use crate::actions::{components::CurrentAction, events::{ActionEvent, AddAction, Finish, QueueActions, RemoveAction, Start, Stop}, systems::{on_action_finish, on_action_start, on_action_stop, on_add_action, on_queue_actions, on_remove_action, update_actions}};

pub mod action;
mod systems;
pub mod components;
pub mod events;
pub mod utils;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(self, app:  &mut engine::app::App) {
        app
            .add_observer(
                on_add_action
            )
            .add_observer(
                on_remove_action
            )
            .add_observer(
                on_queue_actions
            )
            .add_observer(
                on_action_start
            )
            .add_observer(
                on_action_finish
            )
            .add_observer(
                on_action_stop
            )
            .add_systems(
                ScheduleLabel::FixedUpdate, 
                update_actions
            );
    }
}

