use crate::{app::main_schedule::ScheduleLabel, ecs::{schedule::schedule_system::ScheduleSystem, systems::node::SystemNode, worlds::World}};


#[derive(Default)]
pub(crate) struct Systems {
    nodes: Vec<SystemNode>,
}

impl Systems {
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
    pub fn get(&self, index : usize) -> Option<&SystemNode> {
        self.nodes.get(index)
    }

    pub fn get_mut(&mut self, index : usize) -> Option<&mut SystemNode> {
        self.nodes.get_mut(index)
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &SystemNode> {
        self.nodes.iter()
    }

    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut SystemNode> {
        self.nodes.iter_mut()
    }

    pub fn apply(&mut self, world : &mut World) {
        self.iter_mut().for_each(|n| n.apply(world));
    }

    pub fn insert(
        &mut self,
        system: ScheduleSystem
    ) -> usize {
        let index = self.nodes.len();
        self.nodes.push(SystemNode::new(system));
        index
    }

    pub fn remove(&mut self, index: usize) -> bool {
        if self.len() <= index {
            return false;
        }
        self.nodes.remove(index);
        return true;
    }
}
