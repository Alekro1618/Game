use engine::{ecs::{entities::id::EntityId, observer::system_param::On, systems::{commands::Commands, query::Query}}, utils::timer::{OnTimerFinished, Timer}};
use crate::{items::{AddToInventory, RemoveFromInventory}, machines::{machine::{InteractWithMachine, Machine, OnFinishedMachine, WaitingFor}, machine_function::{ChangeOutput, MachineFunction}, machine_state::{self, MachineState}, machine_type::MachineType}, players::player::Player};


pub fn on_interact_with_machine(
    on : On<InteractWithMachine>,
    mut commands : Commands,
    query_machines : Query<(&Machine, &MachineFunction, &mut MachineState)>,
) {
    let InteractWithMachine{ 0: interacting_id, 1: machine_id} = on.event();
    let Ok(( machine, machine_function, mut machine_state)) = query_machines.get_mut(*machine_id) else {
        panic!("");
    };

    match *machine_state {
        MachineState::Idle => {
            commands.trigger(RemoveFromInventory(*interacting_id, machine_function.input));
            if let Some(take_time) = machine.take_time {
                machine_state.set_active(); 
                let mut timer = Timer::new(0.0, take_time);
                timer.set_result(*machine_id);
                commands.spawn((timer, ));
                if machine.is_holding {
                    commands.add(*interacting_id, (WaitingFor(*machine_id), ));
                }
            } else {
                if machine.is_holding {
                    commands.add(*interacting_id, (WaitingFor(*machine_id), ));
                } else {
                    machine_state.set_idle();
                    commands.trigger(AddToInventory(*interacting_id, machine_function.output)); 
                }
            }
        },
        MachineState::Active => {
            commands.trigger(OnFinishedMachine(*machine_id));
            machine_state.set_finished();
        },
        MachineState::Finished => {
            machine_state.set_idle();
            commands.trigger(AddToInventory(*interacting_id, machine_function.output));   
        },
    }

    

}

pub fn on_machine_finished(
    on : On<OnFinishedMachine>,
    mut commands : Commands, 
    query_machines : Query<(EntityId, &Machine)>,
    query_waiting : Query<(EntityId, &WaitingFor)>
) {
    let on_finished_machine = on.event();
    let Ok((machine_id, _))
        = query_machines.get_mut(on_finished_machine.0) else {
            panic!("");
        };
    
    //Check if any entity is waiting for machine to finish
    for (entity_id, _) in query_waiting.iter()
        .filter(|(_, waiting_for)| waiting_for.0 == on_finished_machine.0) {
            commands.trigger(InteractWithMachine(entity_id, machine_id));
            commands.remove::<(WaitingFor, )>(entity_id);
        }
}

pub fn on_machine_change_output(
    on : On<ChangeOutput>,
    query_machines : Query<&mut MachineFunction>,
) {
    let change_output = on.event();
    let Ok(mut machine_function) 
        = query_machines.get_mut(change_output.0) else {
            panic!("");
        };
    
    machine_function.output = change_output.1;
}

pub fn when_machine_timer_finishes(
    on : On<OnTimerFinished<EntityId>>,
    mut commands : Commands,
    query_machines : Query<(EntityId, &Machine, &mut MachineState)>,
) {
    let on_timer_finished = on.event();
    let result = on_timer_finished.result.unwrap();

    let Ok((machine_id, _, mut machine_state))
        = query_machines.get_mut(result) else {
            panic!("");
        };
    
    commands.trigger(OnFinishedMachine(machine_id));
    machine_state.set_finished();
}