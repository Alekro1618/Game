use crate::{actions::{action::Action, components::{ActionQueue, CurrentAction}}, items::Item};

pub fn set_up_action_queue(mut iter : impl Iterator<Item = Box<dyn Action>>) -> (CurrentAction, ActionQueue) {
    let mut current_action = CurrentAction::new();
    let mut action_queue = ActionQueue::new();

    if let Some(first) = iter.next() {
        current_action.0 = Some(first);
    }

    while let Some(item) = iter.next() {
        action_queue.push(item);
    }

    
    return (current_action, action_queue);
}

#[macro_export]
macro_rules! queue {
    () => {
        $crate::actions::utils::set_up_action_queue(
            ::std::iter::empty::<Box<dyn $crate::actions::action::Action>>()
        )
    };
    ($($action:expr),+ $(,)?) => {
        $crate::actions::utils::set_up_action_queue(
            vec![
                $(Box::new($action) as Box<dyn $crate::actions::action::Action>),+
            ].into_iter()
        )
    };
}