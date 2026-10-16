use engine::ecs::entities::id::EntityId;
use engine_macros::Resource;

#[derive(Resource, Default)]
pub struct PickedServant(Option<EntityId>);
