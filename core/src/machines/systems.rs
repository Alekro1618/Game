
use engine::{ecs::{change_detection::params::Mut, entities::id::EntityId, observer::system_param::On, query::filter::With, systems::{commands::{self, Commands}, query::Query}}, utils::*};
use crate::{items::Item, machines::{ events::{self, DisruptMachine, FinishedMachine, InteractWithMachine, Outputed}, functions::MachineFunction, machine::{Machine, WaitingFor}, machine_type::MachineType, register::MachineRegister, state::MachineState}};


pub fn register_machines(
    query_machines : Query<(EntityId, &MachineType)>,
    mut register : Mut<MachineRegister>
) {
    for (entity, machine_type) in query_machines {
        register.insert(machine_type.clone(), entity);
    }
}

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

    commands.trigger(StopTimer::<FinishedMachine>::new(Some(machine_id)));
    machine_state.set_disrupted();
    machine_function.clear();
}

pub fn on_interact_with_machine(
    on : On<InteractWithMachine>,
    mut commands : Commands,
    query_machines : Query<(&Machine, &mut MachineFunction, &mut MachineState)>,
) {
    let interact_with= on.event();
    let interacting_id = interact_with.0;
    let machine_id = interact_with.1;
    let item = interact_with.2;

    let Ok((machine, mut machine_function, mut machine_state)) = query_machines.get_mut(machine_id) else {
        panic!("");
    };

    if machine_state.is(MachineState::Finished) {
        machine_state.set_idle();
        output_machine(&mut commands, &mut machine_function, interacting_id, machine_id);
        return;
    }

    choose_new_state(machine, &mut machine_state, &mut machine_function);

    input_machine(&mut commands, &mut machine_function, interacting_id, machine_id, item);

    hold_interacting(&mut commands, machine, interacting_id, machine_id);

    set_timer(&mut commands, machine, &mut machine_function, interacting_id, machine_id);  
}

pub fn on_machine_finished(
    on : On<FinishedMachine>,
    query_machines : Query<(&Machine, &mut MachineFunction, &mut MachineState)>,
    query_waiting : Query<EntityId, With<WaitingFor>>,
    mut commands: Commands
) {
    let finished_machine = on.event();
    let FinishedMachine(machine_id) = finished_machine;
    
    let Ok((machine, mut machine_function, mut machine_state)) = query_machines.get_mut(*machine_id) else {
        panic!("");
    };

    machine_state.set_finished();

    //Trigger [`Outputed`] event for entities with [`WaitingFor`]
    for entity_id in query_waiting {
        output_machine(&mut commands, &mut machine_function, entity_id, *machine_id);
    }
}

pub fn hold_interacting(commands: &mut Commands, machine: &Machine, interacting_id : EntityId, machine_id : EntityId) {
    if machine.is_holding {
        commands.add(interacting_id, WaitingFor(machine_id));
    }
}

pub fn set_timer(commands: &mut Commands, machine: &Machine, machine_function : &mut MachineFunction, interacting_id : EntityId, machine_id : EntityId) {
    if let Some(time) = machine.take_time {
        let mut timer = Timer::<FinishedMachine>::new(Some(machine_id), 0.0, time);
        timer.set_result(FinishedMachine(machine_id));
        commands.spawn(timer);
    } else {
        output_machine(commands, machine_function, interacting_id, machine_id);
    }
}

pub fn input_machine(commands: &mut Commands, machine_function : &mut MachineFunction, interacting_id : EntityId, machine_id : EntityId, item : Option<Item>) {
    commands.trigger(events::Inputed(interacting_id, machine_id, item));
    if let Err(error) = machine_function.prepeare_for(item) {
        println!("Item: {:#?} Warning: {}", item, error);
    }
}

pub fn output_machine(commands: &mut Commands, machine_function : &mut MachineFunction, interacting_id : EntityId, machine_id : EntityId) {
    match machine_function.get_prepeared()  {
        Ok(output) 
            => commands.trigger(Outputed(interacting_id, machine_id, output)),
        Err(err) 
            => panic!("{}", err),
    }
}

pub fn choose_new_state(machine: &Machine, machine_state : &mut MachineState,  machine_function : &mut MachineFunction) {
    match *machine_state {
        MachineState::Idle => {
            if machine.take_time.is_some() {
                machine_state.set_active();
            } else {
                machine_state.set_idle();
            }
        },
        MachineState::Active => {},
        MachineState::Finished => {
            machine_state.set_idle();
        },
        MachineState::Disrupted => {
            if machine.take_time.is_some() {
                machine_state.set_repearing();
            } else {
                machine_state.set_idle();
            }
        },
        MachineState::Repearing => {},
    }
} 
