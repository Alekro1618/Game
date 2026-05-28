use std::marker::PhantomData;

use engine::{app::{main_schedule::ScheduleLabel, plugin::Plugin}, ecs::{entities::id::EntityId, observer::system_param::On, systems::{commands::Commands, function_system::{ F2, F3, F4}, query::Query}}, utils::*};

use crate::machines::{machine::{DisruptMachine, InteractWithMachine, Machine, OnFinishedMachine, WaitingFor}, machine_function::{ MachineFunction}, machine_state::MachineState, systems::{on_disrupt_machine, on_interact_with_machine, on_machine_finished, register_machines, when_machine_timer_finishes}};

pub mod machine;
pub mod machine_type;
pub mod systems;
pub mod machine_function;
pub mod machine_state;
pub mod machine_register;

pub struct MachinePlugins;

impl Plugin for MachinePlugins {
    fn build(self, app:  &mut engine::app::App) {
        app
            .add_plugin(TimerPlugin::<Machine, EntityId>(PhantomData))
            .add_systems::<F2<Commands, Query<(EntityId, &Machine)>>>(
                ScheduleLabel::Startup, 
                register_machines
            )
            .add_observer::<InteractWithMachine, F3<On<InteractWithMachine>, Commands, Query<(&Machine, &mut MachineFunction, &mut MachineState)>>>(
                on_interact_with_machine
            )
            .add_observer::<OnFinishedMachine, F4<On<OnFinishedMachine>, Commands, Query<(EntityId, &Machine)>,  Query<(EntityId, &WaitingFor)>>>(
                on_machine_finished
            )
            .add_observer::<OnTimerFinished<Machine, EntityId>, F3<On<OnTimerFinished<Machine, EntityId>>, Commands, Query<(EntityId, &Machine, &mut MachineState)>>>(
                when_machine_timer_finishes
            )
            .add_observer::<DisruptMachine, F3<On<DisruptMachine>, Commands, Query<(&Machine, &mut MachineFunction, &mut MachineState)>>>(
                on_disrupt_machine
            );
    }
}