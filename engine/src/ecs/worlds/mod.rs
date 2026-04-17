use std::{any::{Any, TypeId, type_name}, sync::atomic::AtomicU32};

use macroquad::prelude::warn;

use crate::{app::main_schedule::ScheduleLabel, ecs::{archetype::{self, Archetypes}, bundle::{Bundle, info::Bundles}, change_detection::{params::Mut, tick::Tick}, components::{Component, id::{ComponentId, ComponentIdsArray}, info::{ComponentDescriptor, Components}, register::{ComponentIds, ComponentsRegistrator}}, entities::{Entities, Entity, EntityAllocator, EntityLocation, id::EntityId}, lifecycle::{Add, ComponentHooks, Despawn, Insert, Remove, RemovedComponentMessages, Replace}, observers::Observers, query::{fetch::QueryData, filter::QueryFilter, state::QueryState}, resource::{Resource, ResourceEntities}, schedule::schedule::{Schedule, Schedules}, storage::{Storages, TableBuilder}, systems::commands::{Commands, command::CommandQueue}, worlds::id::WorldId}, error::error::{Error, ErrorKind}};
mod id;
pub mod assert;

pub struct World {
    id: WorldId,
    pub entities: Entities,
    pub entity_allocator: EntityAllocator,
    pub components: Components,
    pub resource_entities: ResourceEntities,
    pub component_ids: ComponentIds,
    pub archetypes: Archetypes,
    pub storages: Storages,
    pub bundles: Bundles,
    pub observers: Observers,
    pub removed_components: RemovedComponentMessages,
    pub change_tick: AtomicU32,
    pub last_change_tick: Tick,
    pub last_check_tick: Tick,
    pub command_queue: CommandQueue,
    pub last_trigger_id: u32
}
impl Default for World {
    fn default() -> Self {
        let mut world = Self {
            id: WorldId::new().expect("More `World`s have been created than is supported"),
            entities: Entities::new(),
            entity_allocator: EntityAllocator::default(),
            components: Default::default(),
            resource_entities: ResourceEntities::default(),
            archetypes: Archetypes::new(),
            storages: Default::default(),
            bundles: Default::default(),
            observers: Observers::default(),
            removed_components: Default::default(),
            change_tick: AtomicU32::new(1),
            last_change_tick: Tick::new(0),
            last_check_tick: Tick::new(0),
            last_trigger_id: 0,
            command_queue: CommandQueue::default(),
            component_ids: ComponentIds::default(),
        };
        world.bootstrap();
        world
    }
}

impl Drop for World {
    fn drop(&mut self) {
        //todo!("implement drop");
    }
}

impl World {

    #[inline]
    fn bootstrap(&mut self) {
        // self.register_event_key::<Add>(); TODO
        // self.register_event_key::<Insert>();
        // self.register_event_key::<Replace>();
        // self.register_event_key::<Remove>();
        // self.register_event_key::<Despawn>();
        // let is_resource = self.register_component::<IsResource>();
        // self.init_resource::<DefaultQueryFilters>(); TODO
    }

    #[inline]
    pub fn new() -> World {
        World::default()
    }

    #[inline]
    pub fn id(&self) -> WorldId {
        self.id
    }

    #[inline]
    pub fn entities(&self) -> &Entities {
        &self.entities
    }

    #[inline]
    pub fn entity_allocator(&self) -> &EntityAllocator {
        &self.entity_allocator
    }

    #[inline]
    pub fn entity_allocator_mut(&mut self) -> &mut EntityAllocator {
        &mut self.entity_allocator
    }

    #[inline]
    pub unsafe fn entities_mut(&mut self) -> &mut Entities {
        &mut self.entities
    }

    #[inline]
    pub fn entity_count(&self) -> u32 {
        self.entities.count_spawned()
    }

    #[inline]
    pub fn archetypes(&self) -> &Archetypes {
        &self.archetypes
    }

    #[inline]
    pub fn components(&self) -> &Components {
        &self.components
    }

    #[inline]
    pub fn components_registrator(&mut self) -> ComponentsRegistrator<'_> {
        unsafe { ComponentsRegistrator::new(&mut self.components, &mut self.component_ids) }
    }

    #[inline]
    pub fn command_queue(&self) -> CommandQueue {
        CommandQueue::default()
    }

    #[inline]
    #[track_caller]
    pub fn resource_mut<R: Resource>(&mut self) -> Mut<'_, R> {
        match self.get_resource_mut() {
            Some(x) => x,
            None => panic!(
                "Requested resource does not exist in the `World`"
            ),
        }
    }

    #[inline]
    pub fn resource_entities(&self) -> &ResourceEntities {
        &self.resource_entities
    }

    #[inline]
    pub fn storages(&self) -> &Storages {
        &self.storages
    }

    #[inline]
    pub fn bundles(&self) -> &Bundles {
        &self.bundles
    }

    #[inline]
    pub fn removed_components(&self) -> &RemovedComponentMessages {
        &self.removed_components
    }

    #[inline]
    pub fn observers(&self) -> &Observers {
        &self.observers
    }

    #[inline]
    pub fn get_entity(&self, entity: EntityId) -> Result<EntityLocation, Error> {
        self.entities().get_spawned(entity)
    }

    pub fn register_component<T: Component>(&mut self) -> ComponentId {
        self.components_registrator().register_component::<T>()
    }

    // #[must_use]
    // pub fn register_component_hooks<T: Component>(&mut self) -> &mut ComponentHooks {
    //     let index = self.register_component::<T>();
    //     assert!(!self.archetypes.archetypes.iter().any(|a| a.contains(index)), "Components hooks cannot be modified if the component already exists in an archetype, use register_component if {} may already be in use", core::any::type_name::<T>());
    //     unsafe { self.components.get_hooks_mut(index).debug_checked_unwrap() }
    // }

    // pub fn register_component_hooks_by_id(
    //     &mut self,
    //     id: ComponentId,
    // ) -> Option<&mut ComponentHooks> {
    //     assert!(!self.archetypes.archetypes.iter().any(|a| a.contains(id)), "Components hooks cannot be modified if the component already exists in an archetype, use register_component if the component with id {id:?} may already be in use");
    //     self.components.get_hooks_mut(id)
    // }

    pub fn register_component_with_descriptor(
        &mut self,
        descriptor: ComponentDescriptor,
    ) -> ComponentId {
        self.components_registrator()
            .register_component_with_descriptor(descriptor)
    }

    #[inline]
    pub fn component_id<T: Component>(&self) -> Option<ComponentId> {
        self.components.component_id::<T>()
    }

    pub fn get_component<T: Component>(&self, entity: EntityId) -> Option<&T> {
        if !self.entities.is_alive(entity) {
            return None;
        }

        let comp_id = self.components.get_id(TypeId::of::<T>())?;
        let archetype_id = self.entities.get_spawned(entity).ok()?.archetype_id;
        let archetype = self.archetypes.get(archetype_id)?;

        let &row = archetype.row(entity)?;
        let table = &self.storages.tables[archetype.table_id() as usize];
        table.get::<T>(comp_id, row as usize)
    }

    pub fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityId {
        let entity = self.entity_allocator.allocate();
        self.entities.alloc(entity.id(), entity.gnr());

        self.spawn_at_unchecked(entity, bundle)
    }

    
    pub fn spawn_at_unchecked<B: Bundle>(
        &mut self,
        entity_id: EntityId,
        bundle: B,
    ) -> EntityId {
        let change_tick = self.change_tick();
        let component_ids 
            = B::component_ids(&mut self.components_registrator())
            .collect();
        

        let archetype_id = self.archetypes.get_id(&component_ids).unwrap_or({
            let table_id = self.storages().len();

            let mut table_builder = TableBuilder::with_capacity(0, component_ids.len());
            for component_id in component_ids.iter() {
                table_builder.add_column(self.components().get_info(*component_id.1).unwrap());
            }

            let table = table_builder.build();
            self.storages.insert_table(table);
            self.archetypes.insert_id(table_id as u32, &component_ids)
        });

        let archetype = self.archetypes.get_mut(archetype_id).unwrap();
        let table = &mut self.storages.tables[archetype.table_id() as usize];

        let row = table.add_entity(entity_id);

        

        let location = archetype.insert(entity_id, row as u32);
        
        self.entities.set_location(entity_id, location);
        
        bundle.insert_into_table(&component_ids, table, change_tick);
        self.bundles.register();

        entity_id
    }

    #[inline]
    pub fn last_change_tick(&mut self) -> Tick {
        self.last_change_tick
    }

     #[inline]
    pub fn last_check_tick(&mut self) -> Tick {
        self.last_check_tick
    }

    #[inline]
    pub fn change_tick(&mut self) -> Tick {
        let tick = *self.change_tick.get_mut();
        Tick::new(tick)
    }

    #[inline]
    pub fn query<D: QueryData>(&mut self) -> QueryState<D, ()> {
        self.query_filtered::<D, ()>()
    }

    #[inline]
    pub fn query_filtered<D: QueryData, F: QueryFilter>(&mut self) -> QueryState<D, F> {
        QueryState::new(self)
    }

    #[inline]
    pub fn get_resource<R: Resource>(&self) -> Option<&R> {
        self
            .resource_entities
            .get::<R>()

    }

    #[inline]
    pub fn get_resource_mut<R: Resource>(&mut self) -> Option<Mut<'_, R>> {
        self
            .resource_entities
            .get_mut::<R>()
            .map(|r| Mut { value: r})

    }

    pub fn init_resource<R: Resource + Default>(&mut self) {
        if !self.resource_entities().contains::<R>() {
            self.insert_resource(R::default());
        }
    }

    pub fn insert_resource<R: Resource>(&mut self, value: R) {
        self.resource_entities.insert(value);
    }

    pub fn register_resource<R: Resource>(&mut self) -> ComponentId {
        self.components_registrator().register_component::<R>()
    }

    pub fn run_schedule(&mut self, label: ScheduleLabel) {
        self.schedult_scope::<()>(label, |world, sched| sched.run(world))
            .unwrap_or_else(|e| panic!("{e}"));
    }

    pub fn schedult_scope<R>(
        &mut self, 
        label: ScheduleLabel, 
        f: impl FnOnce(&mut World, &mut Schedule) -> R
    ) -> Result<R, Error> {
        let Some(mut schedules) = self
            .get_resource_mut::<Schedules>()
        else {
            return Err(Error::new(ErrorKind::ECS, "Error while getting schedules"));
        };

        let Some(mut schedule) = schedules.remove(label)
        else {
            return Err(Error::new(ErrorKind::ECS, "Error while trying to run schedule"));
        };

        let value = f(self, &mut schedule);

        let old = self.get_resource_mut::<Schedules>().unwrap().insert(schedule);
        if old.is_some() {
            warn!("Schedule `{label:?}` was inserted during a call to `World::schedule_scope`: its value has been overwritten");
        }

        Ok(value)
    }
}


unsafe impl Send for World {}
unsafe impl Sync for World {}

pub trait FromWorld {
    fn from_world(world: &mut World) -> Self;
}

impl<T: Default> FromWorld for T {
    #[track_caller]
    fn from_world(_world: &mut World) -> Self {
        T::default()
    }
}