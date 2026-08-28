use std::marker::PhantomData;

use engine::ecs::{change_detection::params::Mut, entities::id::EntityId, observer::system_param::On, query, systems::{commands::{self, Commands}, query::Query}, worlds::World};

use crate::actions::{action::Action, components::{ActionQueue, CurrentAction}, events::{ActionEvent, AddAction, Finish, QueueActions, RemoveAction, Start, Stop}};

pub fn on_add_action(
    on : On<AddAction>,
    mut world : Mut<World>,
) {
    let add_action = on.event();
    let agent = add_action.0;

    let action = add_action.1.into_boxed();
    add_single_action(&mut world, action, agent);
}

pub fn add_single_action(
    world : &mut World,
    mut action :  Box<dyn Action>,
    agent : EntityId,
) {
    action.on_add(agent, world);

    let Some(current_action) = world.get_component_mut::<CurrentAction>(agent) else {
        panic!("Trying to add new action to entity that doesn't have `CurrentAction`");
    };

    if current_action.0.is_none() {
        current_action.0 = Some(action);
        world.trigger_ref(&mut ActionEvent::<Start>(agent, PhantomData));
        return;
    }

    let Some(action_queue) = world.get_component_mut::<ActionQueue>(agent) else {
        panic!("Trying to add new action to entity that doesn't have `ActionQueue`");
    };
   
    action_queue.push(action);
}

pub fn on_queue_actions(
    on : On<QueueActions>,
    mut world : Mut<World>,
) {
    let queue_actions = on.event();
    let agent = queue_actions.0;

    let mut actions : Vec<Box<dyn Action>> = queue_actions.1.iter().map(|a| a.into_boxed()).collect();

    for action in &mut actions {
        action.on_add(agent, &mut world);
    }

    let mut actions = actions.into_iter();

    let Some(current_action) = world.get_component_mut::<CurrentAction>(agent) else {
        panic!("Trying to add new action to entity that doesn't have `CurrentAction`");
    };

    if current_action.0.is_none() {
        let Some(first_action) = actions.next() else {
            return;
        };

        current_action.0 = Some(first_action);
        world.trigger_ref(&mut ActionEvent::<Start>(agent, PhantomData));
    }

    let Some(action_queue) = world.get_component_mut::<ActionQueue>(agent) else {
        panic!("Trying to add new action to entity that doesn't have `ActionQueue`");
    };

    while let Some(action) = actions.next() {
        action_queue.push(action);
    }
}

pub fn on_remove_action(
    on : On<RemoveAction>,
    mut world : Mut<World>
) {
    let remove_action = on.event();
    let agent = remove_action.0;

    let Some(action_queue) = world.get_component_mut::<ActionQueue>(agent) else {
        panic!("Trying to remove action from entity that doesn't have `ActionQueue`");
    };

    let Some(next_action) = action_queue.pop() else {
        return;
    };

    let Some(current_action) = world.get_component_mut::<CurrentAction>(agent) else {
        panic!("Trying to add remove action from entity that doesn't have `CurrentAction`");
    };

    if let Some(previous_action) = current_action.0.replace(next_action) {
        previous_action.on_remove(agent, &mut world);
    }
    
    world.trigger_ref(&mut ActionEvent::<Start>(agent, PhantomData));
}

pub fn update_actions(
    query : Query<(EntityId, &CurrentAction)>,
    mut commands : Commands,
    world : &World
) {
    query
        .iter_inner()
        .filter_map(|(agent, current_action)| {
            current_action
                .0
                .as_ref()
                .and_then(|action| action.is_finished(agent, world).then_some(agent) )
        }
        )
        .for_each(|agent| {
           commands.trigger(ActionEvent::<Finish>(agent, PhantomData));
        });
}

pub fn on_action_start(
    on : On<ActionEvent<Start>>,
    mut world : Mut<World>,
) {
    let start_action = on.event();
    let agent = start_action.0;

    let Some(current_action) 
        = world.get_component_mut::<CurrentAction>(agent) else {
        panic!("")
    };
    
    if let Some(mut action) = current_action
        .0
        .take() {
            if action.on_start(agent, &mut world) {
                world.trigger_ref(&mut ActionEvent::<Finish>(agent, PhantomData));
            }

            let Some(current_action) 
                = world.get_component_mut::<CurrentAction>(agent) else {
                panic!("")
            };
            current_action.0 = Some(action);
        } else {
            println!("No current action")
        }
}

pub fn on_action_finish(
    on : On<ActionEvent<Finish>>,
    mut world : Mut<World>,
) {
    let finish_action = on.event();
    let agent = finish_action.0;

    let Some(current_action) 
        = world.get_component_mut::<CurrentAction>(agent) else {
        panic!("")
    };
    
    if let Some(mut action) = current_action
        .0
        .take() {
            action.on_end(agent, &mut world);

            let Some(current_action) 
                = world.get_component_mut::<CurrentAction>(agent) else {
                panic!("")
            };

            current_action.0 = Some(action);

            world.trigger_ref(&mut RemoveAction(agent));
        }
}


pub fn on_action_stop(
    on : On<ActionEvent<Stop>>,
    mut world : Mut<World>,
) {
    let end_action = on.event();
    let agent = end_action.0;

    let Some(current_action) 
        = world.get_component_mut::<CurrentAction>(agent) else {
        panic!("")
    };
    
    if let Some(mut action) = current_action
        .0
        .take() {
            action.on_stop(agent, &mut world);

            let Some(current_action) 
                = world.get_component_mut::<CurrentAction>(agent) else {
                panic!("")
            };

            current_action.0 = Some(action);

            world.trigger_ref(&mut RemoveAction(agent));
        }
}