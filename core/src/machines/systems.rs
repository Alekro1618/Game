use engine::{ecs::{change_detection::params::Mut, entities::id::EntityId, observer::system_param::On, systems::{commands::Commands, query::Query}}, utils::*};
use crate::{items::{AddToInventory, Item, RemoveFromInventory}, machines::{machine::{DisruptMachine, InteractWithMachine, Machine, OnFinishedMachine, WaitingFor}, machine_function::{MachineFunction}, machine_register::{self, MachineRegister}, machine_state::{self, MachineState}, machine_type::MachineType}, players::player::Player};


pub fn on_disrupt_machine(
    on : On<DisruptMachine>,
    mut commands : Commands,
    query_machines : Query<(&Machine, &mut MachineFunction, &mut MachineState)>,
) {
    let disrupt_who= on.event();
    let machine_id = disrupt_who.1;
    let Ok((machine, mut machine_function, mut machine_state)) = query_machines.get_mut(machine_id) else {
        panic!("");
    };
    machine_function.input(None);
    machine_state.set_disrupted();
}

pub fn on_interact_with_machine(
    on : On<InteractWithMachine>,
    mut commands : Commands,
    query_machines : Query<(&Machine, &mut MachineFunction, &mut MachineState)>,
) {
    let interact_with= on.event();
    let machine_id = interact_with.1;
    let Ok(( machine, machine_function, machine_state)) = query_machines.get_mut(machine_id) else {
        panic!("");
    };

    match *machine_state {
        MachineState::Idle => {
            on_interact_with_idle_machine(&mut commands, interact_with, machine, machine_function, machine_state);
        },
        MachineState::Active => {
            on_interact_with_active_machine(&mut commands, interact_with, machine, machine_function, machine_state);
        },
        MachineState::Finished => {
            on_interact_with_finished_machine(&mut commands, interact_with, machine, machine_function, machine_state);
        },
        MachineState::Busy => {
            on_interact_with_busy_machine(&mut commands, interact_with, machine, machine_function, machine_state);
        },
        MachineState::Disrupted => {
            on_interact_with_disrupted_machine(&mut commands, interact_with, machine, machine_function, machine_state);
        },
        MachineState::Repearing => {}
    }
}

fn on_interact_with_disrupted_machine(
    commands : &mut Commands,
    interact_with : &InteractWithMachine,
    machine : &Machine,
    mut machine_function: Mut<MachineFunction>, 
    mut machine_state: Mut<MachineState>
) {
    let InteractWithMachine{ 0: interacting_id, 1: machine_id, 2: item} = interact_with;

    //If fixing takes time we will wait for it
    if let Some(take_time) = machine.fix_time {
        machine_state.set_repearing(); 
        let mut timer = Timer::<Machine, EntityId>::new(0.0, take_time);
        timer.set_result(*machine_id);
        commands.spawn((timer, ));

        //3.1 If machine is holding then we hold entity waiting for the machine to end. Removes it on OnFinishedMachine.
        if machine.is_holding {
            commands.add(*interacting_id, (WaitingFor(*machine_id), ));
        }
        return;
    } 

    //Instead we just interact as if machine was idle
    on_interact_with_idle_machine(commands, interact_with, machine, machine_function, machine_state);
}

fn on_interact_with_busy_machine(
    commands : &mut Commands,
    interact_with : &InteractWithMachine,
    machine : &Machine,
    machine_function: Mut<MachineFunction>, 
    machine_state: Mut<MachineState>
) {
    on_interact_with_finished_machine(commands, interact_with, machine, machine_function, machine_state);
}

fn on_interact_with_finished_machine(
    commands : &mut Commands,
    interact_with : &InteractWithMachine,
    machine : &Machine,
    mut machine_function: Mut<MachineFunction>, 
    mut machine_state: Mut<MachineState>
) {
    let InteractWithMachine{ 0: interacting_id, 1: machine_id, 2: item} = interact_with;

    machine_state.set_idle();
    match machine_function.get_ready_output() {
        Ok(machine_output) => {
            if let Some(output) = machine_output {
                commands.trigger(AddToInventory(
                    *interacting_id,
                    output
                ));
            }
            machine_function.input(None);
        },
        Err(error) => {
            if let Some(item) = item {
                panic!("Error with machine {:#?} and item {}. {}", machine.machine_type, item, error);
            } else {
                panic!("Error with machine {:#?}. {}", machine.machine_type, error);
            }
            
        },
    }
}

fn on_interact_with_active_machine(
    commands : &mut Commands,
    interact_with : &InteractWithMachine,
    machine : &Machine,
    mut _machine_function: Mut<MachineFunction>, 
    mut machine_state: Mut<MachineState>
) {
    
}

fn on_interact_with_idle_machine(
    commands : &mut Commands,
    interact_with : &InteractWithMachine,
    machine : &Machine,
    mut machine_function: Mut<MachineFunction>, 
    mut machine_state: Mut<MachineState>
) {
    let InteractWithMachine{ 0: interacting_id, 1: machine_id, 2: item} = interact_with;

    //1. Trigger action to remove item from inventory of interacting entity.
    if let Some(item) = item {
        commands.trigger(RemoveFromInventory(
            *interacting_id,
            *item
        ));
    }

    //2. Input item into the machine to prepeare output.
    machine_function.input(*item);
    
    //3. If machine takes time we set timer and make machine active. On timer and it will trigger OnFinishedMachine.
    if let Some(take_time) = machine.take_time {
        machine_state.set_active(); 
        let mut timer = Timer::<Machine, EntityId>::new(0.0, take_time);
        timer.set_result(*machine_id);
        commands.spawn((timer, ));

        //3.1 If machine is holding then we hold entity waiting for the machine to end. Removes it on OnFinishedMachine.
        if machine.is_holding {
            commands.add(*interacting_id, (WaitingFor(*machine_id), ));
        }
        return;
    } 

    //4. If machine doesn't take time, but still holds we set machine busy and we hold entity waiting for the machine to end.
    if machine.is_holding {
        machine_state.set_busy();
        commands.add(*interacting_id, (WaitingFor(*machine_id), ));
        return;
    
    } 

    //5. If machine doesn't hold and doesn't take time we interact as if it were busy
    on_interact_with_busy_machine(commands, interact_with, machine, machine_function, machine_state);
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
            commands.trigger(InteractWithMachine(entity_id, machine_id, None));
            commands.remove::<(WaitingFor, )>(entity_id);
        }
}

pub fn when_machine_timer_finishes(
    on : On<OnTimerFinished<Machine, EntityId>>,
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

pub fn register_machines(
    mut commands : Commands,
    query : Query<(EntityId, &Machine)>,
) {
    let mut machine_register = MachineRegister::new();
    for (entity, machine) in query {
        machine_register.insert(machine.machine_type, entity);
    }
    commands.insert(machine_register);
}