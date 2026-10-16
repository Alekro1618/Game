use std::marker::PhantomData;

use engine::{app::{main_schedule::ScheduleLabel, plugin::Plugin}, ecs::{entities::id::EntityId, observer::system_param::On, query::filter::With, systems::{commands::Commands, query::Query}}, utils::*};

use crate::machines::{events::{DisruptMachine, FinishedMachine, InteractWithMachine}, functions::MachineFunction, machine::{Machine, WaitingFor}, machine_type::MachineType, register::MachineRegister, state::MachineState, systems::{on_disrupt_machine, on_interact_with_machine, on_machine_finished, register_machines}};

pub mod machine;
pub mod machine_type;
pub mod systems;
pub mod functions;
pub mod state;
pub mod register;
pub mod events;

pub struct MachinePlugins;

impl Plugin for MachinePlugins {
    fn build(self, app:  &mut engine::app::App) {
        app
            .register_type::<Machine>()
            .register_type::<MachineFunction>()
            .register_type::<MachineState>()
            .register_type::<MachineType>()
            .add_plugin(TimerPlugin::<FinishedMachine>::new())
            .add_systems(
                ScheduleLabel::Startup, 
                register_machines
            )
            .add_observer(
                on_interact_with_machine
            )
            .add_observer(
                on_machine_finished
            )
            .add_observer(
                on_disrupt_machine
            );
    }
}