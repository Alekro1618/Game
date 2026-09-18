
use crate::ecs::{
    schedule::schedule_system::ScheduleSystem, systems::{node::SystemNode, system::System},
};

#[derive(Default)]
pub(crate) struct Systems {
    nodes: Vec<SystemNode>,
    conditions : Vec<Vec<Box<dyn System<In = (), Out = bool>>>>,
}

impl Systems {
    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&mut SystemNode, &mut Vec<Box<dyn System<In = (), Out = bool>>>)> {
        self.nodes
            .iter_mut()
            .zip(self.conditions.iter_mut())
    }

    pub fn insert(
        &mut self, 
        system: ScheduleSystem, 
        conditions: Vec<Box<dyn System<In = (), Out = bool>>>
    ) -> usize {
        let index = self.len();
        self.nodes.push(SystemNode::new(system));
        self.conditions.push(conditions);

        index
    }

    #[allow(dead_code)]
    pub fn remove(&mut self, index: usize) -> bool {
        if self.len() <= index {
            return false;
        }
        self.nodes.remove(index);
        return true;
    }
}
