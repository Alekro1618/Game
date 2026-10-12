
use crate::ecs::{
    schedule::{config::ScheduleConfig, schedule_system::ScheduleSystem}, systems::{node::SystemNode, system::System}, worlds::World,
};

pub enum SystemGraph {
    Node {
        node : SystemNode,
        conditions : Vec<Box<dyn System<In = (), Out = bool>>>,
    },
    Children {
        graph : Vec<SystemGraph>,
        conditions : Vec<Box<dyn System<In = (), Out = bool>>>,
    }
}

impl SystemGraph {
    pub fn run(&mut self, world : &mut World) {
        match self {
            SystemGraph::Node { node, conditions } => {
                if conditions.iter_mut().all(|condition| condition.run((), world)) {
                    node.system.run((), world);
                    node.system.apply(world);
                }
            },
            SystemGraph::Children { graph, conditions } => {
                if conditions.iter_mut().all(|condition| condition.run((), world)) {
                    for node in graph {
                        node.run(world);
                    }
                }
            },
        }
    }

    pub fn init(&mut self, world : &mut World) {
        match self {
            SystemGraph::Node { node, conditions } => {
                conditions
                    .iter_mut()
                    .for_each(|condition| condition.init(world));
                node.system.init(world);
                
            },
            SystemGraph::Children { graph, conditions } => {
                conditions
                    .iter_mut()
                    .for_each(|condition| condition.init(world));
                for node in graph {
                    node.init(world);
                }
            }
        }
    }

    pub fn from_schedule(schedule_config : ScheduleConfig<ScheduleSystem>) -> Self {
        match schedule_config {
            ScheduleConfig::One { node, conditions } => {
                SystemGraph::Node { node: SystemNode::new(node), conditions }
            },
            ScheduleConfig::Many { children, conditions } => {
                SystemGraph::Children {
                    graph: children.into_iter().map(|sc| SystemGraph::from_schedule(sc)).collect(), 
                    conditions 
                }
            },
        }
    }
}



#[derive(Default)]
pub(crate) struct Systems {
    nodes: Vec<SystemGraph>,
}

impl Systems {

    fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut SystemGraph> {
        self.nodes
            .iter_mut()
    }

    pub fn insert(
        &mut self, 
        system: SystemGraph, 
    ) -> usize {
        let index = self.len();
        self.nodes.push(system);

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
