use crate::ecs::{
    schedule::schedule_system::ScheduleSystem, systems::node::SystemNode, worlds::World,
};

#[derive(Default)]
pub(crate) struct Systems {
    nodes: Vec<SystemNode>,
}

impl Systems {
    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut SystemNode> {
        self.nodes.iter_mut()
    }

    pub fn apply(&mut self, world: &mut World) {
        self.iter_mut().for_each(|n| n.apply(world));
    }

    pub fn insert(&mut self, system: ScheduleSystem) -> usize {
        let index = self.nodes.len();
        self.nodes.push(SystemNode::new(system));
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
