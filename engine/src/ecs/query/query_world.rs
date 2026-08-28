use crate::ecs::{
    change_detection::tick::Tick, components::id::ComponentId, storage::Table, worlds::World,
};

pub trait WorldQuery {
    type Fetch<'w>: Clone;
    type State: Send + Sync + Sized;

    fn init_fetch<'w, 's>(
        world: &'w World,
        state: &'s Self::State,
        last_run: Tick,
        this_run: Tick,
    ) -> Self::Fetch<'w>;

    fn init_state(world: &mut World) -> Self::State;

    unsafe fn set_table<'w, 's>(
        fetch: &mut Self::Fetch<'w>,
        state: &'s Self::State,
        table: &'w Table,
    );

    fn matches_component_set(
        state: &Self::State,
        set_contains_id: &impl Fn(ComponentId) -> bool,
    ) -> bool;
}
