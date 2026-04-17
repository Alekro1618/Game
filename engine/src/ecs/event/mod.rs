use crate::ecs::components::id::ComponentId;

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct EventKey(pub(crate) ComponentId);