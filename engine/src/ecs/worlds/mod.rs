use std::{any::TypeId, collections::HashSet, sync::atomic::AtomicU32};

use downcast_rs::Downcast;
use macroquad::prelude::warn;

use crate::{app::main_schedule::ScheduleLabel, ecs::{archetype::{Archetype, ArchetypeId, Archetypes}, bundle::{Bundle, info::Bundles}, change_detection::{params::Mut, tick::Tick}, components::{Component, id::{ComponentId, ComponentIdsArray}, info::{ComponentDescriptor, Components}, register::{ComponentIds, ComponentsRegistrator}}, entities::{Entities, EntityAllocator, EntityLocation, id::EntityId}, event::{Event, EventWrapperComponent}, lifecycle::RemovedComponentMessages, message::{Message, messages::Messages}, observer::{centralized_storage::Observers, distributed_storage::{IntoObserver, Observer}}, query::{fetch::QueryData, filter::QueryFilter, state::QueryState}, resource::{Resource, ResourceEntities}, schedule::schedule::{Schedule, Schedules}, storage::{Storages, TableBuilder}, systems::{commands::command::CommandQueue, observer_system::{IntoObserverSystem, ObserverSystem}, system}, worlds::id::WorldId}, error::error::{Error, ErrorKind}};
mod id;
pub mod assert;
pub mod log;

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
    pub observes: Observers,
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
            observes: Observers::default(),
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
    pub fn observers(&self) -> &Observers {
        &self.observes
    } 

    #[inline]
    pub fn last_trigger_id(&self) -> u32 {
        self.last_trigger_id
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

    pub fn add_observer<E : Event, M>(&mut self, observer: impl IntoObserverSystem<E, M>) -> &mut Self {
        let system = IntoObserverSystem::into_system(observer);
        self.observes.register::<E>(system);
        self
    }

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

        let comp_id = self.component_id::<T>()?;
        let archetype_id = self.entities.get_spawned(entity).ok()?.archetype_id;
        let archetype = self.archetypes.get(archetype_id)?;

        let &row = archetype.row(entity)?;
        let table = &self.storages.tables[archetype.table_id() as usize];

        //Safety: ComponentId is correct and since we take row from archetype we are in bounds.
        unsafe {
            table.get(comp_id, row as usize)
        }
    }

    pub fn get_archetype(&self, entity: EntityId) -> Option<&Archetype> {
        if !self.entities.is_alive(entity) {
            return None;
        }

        let archetype_id = self.entities.get_spawned(entity).ok()?.archetype_id;
        self.archetypes.get(archetype_id)
    }

    pub fn get_component_mut<T: Component>(&mut self, entity: EntityId) -> Option<&mut T> {
        if !self.entities.is_alive(entity) {
            return None;
        }

        let comp_id = self.component_id::<T>()?;
        let archetype_id = self.entities.get_spawned(entity).ok()?.archetype_id;
        let archetype = self.archetypes.get(archetype_id)?;

        let &row = archetype.row(entity)?;
        let table = &mut self.storages.tables[archetype.table_id() as usize];

        //Safety: ComponentId is correct and since we take row from archetype we are in bounds.
        unsafe {
            table.get_mut(comp_id, row as usize)
        }
    }


    pub fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityId {
        let entity = self.entity_allocator.allocate();
        self.entities.alloc(entity.id(), entity.gnr());

        self.spawn_at_unchecked(entity, bundle)
    }

    pub fn despawn(&mut self, entity: EntityId) {
        let location = self.entities.get_spawned(entity).unwrap();
        if self.entities.free(entity) {
            self.despawn_unchecked(entity, location)
        }
    }

    pub fn despawn_unchecked(&mut self, entity: EntityId, location : EntityLocation) {
        let change_tick = self.change_tick();

        let table = self.storages.get_table_mut(location.table_id as usize).unwrap();
        let _ = table.remove_entity(location.table_row as usize);

        let (last_entity, last_row) = self
            .archetypes
            .get_mut(location.archetype_id)
            .unwrap()
            .swap_remove(entity, location.table_row);
        
        //set entity replaced by moved location
        if let Some(last_entity_id) = last_entity {
            self.entities.set_location(
                last_entity_id, 
                EntityLocation { 
                    archetype_id: location.archetype_id, 
                    table_id: location.table_id, 
                    table_row: last_row
                }
            );
        }
    }

    pub fn insert<B: Bundle>(
        &mut self,
        entity_id: EntityId,
        bundle : B
    )  {
        let change_tick = self.change_tick();
        let component_ids : ComponentIdsArray   
            = B::component_ids(&mut self.components_registrator())
            .collect();
        let old_component_ids = component_ids.clone();

        let old_location = self.get_entity(entity_id).unwrap();
        let old_row: u32 = old_location.table_row;
        let old_archetype = self.archetypes.get(old_location.archetype_id).unwrap();
        if old_archetype.contains_all(component_ids.ids()) {
            let table = &mut self.storages.tables[old_location.table_id as usize];
            bundle.replace_in_table(&old_component_ids, table, old_row as usize, change_tick);
            return;
        };

        let mut new_component_ids = old_archetype.archetype_components().clone();
        new_component_ids.append(&mut old_component_ids.ids().clone());
        new_component_ids.sort();

        let new_archetype_id = self.get_or_create_archetype(new_component_ids);

        let new_archetype = self.archetypes.get_mut(new_archetype_id).unwrap();


        let [old_table, new_table] 
            = self.storages.get_disjoint_table_mut(
                [
                        old_location.table_id as usize, 
                        new_archetype.table_id() as usize
                    ]
                )
                .unwrap();
        let _ = old_table.remove_entity(old_row as usize);


        let row = new_table.add_entity(entity_id);
        let new_location = new_archetype.insert(entity_id, row as u32);

        unsafe {
            old_table.move_to(old_row as usize,  new_table , change_tick);
        }

        bundle.insert_into_table(&old_component_ids, new_table, change_tick);

        let (last_entity, last_row) = self
            .archetypes
            .get_mut(old_location.archetype_id)
            .unwrap()
            .swap_remove(entity_id, old_location.table_row);
        
        //set entity replaced by moved location
        if let Some(entity) = last_entity {
            self.entities.set_location(
                entity, 
                EntityLocation { 
                    archetype_id: old_location.archetype_id, 
                    table_id: old_location.table_id, 
                    table_row: last_row
                }
            );
        }
        
        //set moved entity location
        self.entities.set_location(entity_id, new_location);
        self.bundles.register();
    }

    pub fn remove<B: Bundle>(&mut self, entity_id: EntityId){
        let change_tick = self.change_tick();
        let to_remove_component_ids : ComponentIdsArray   
            = B::component_ids(&mut self.components_registrator())
            .collect();

        let old_location = self.get_entity(entity_id).unwrap();
        let old_archetype = self.archetypes.get(old_location.archetype_id).unwrap();
        if !old_archetype.contains_any(to_remove_component_ids.ids()) {
            return;
        };

        let mut new_component_ids = Vec::new();
        
        for &component_id in old_archetype.archetype_components() {
            if !to_remove_component_ids.contains(component_id) {
                new_component_ids.push(component_id);
            }
        }

        new_component_ids.sort(); //REDO:

        let new_archetype_id = self.get_or_create_archetype(new_component_ids);
        let new_archetype = self.archetypes.get_mut(new_archetype_id).unwrap();

        let [old_table, new_table] 
            = self.storages.get_disjoint_table_mut(
                [
                        old_location.table_id as usize, 
                        new_archetype.table_id() as usize
                    ]
                )
                .unwrap();
        let _ = old_table.remove_entity(old_location.table_row as usize);

        let row = new_table.add_entity(entity_id);
        let new_location = new_archetype.insert(entity_id, row as u32);

        unsafe {
            old_table.move_to(old_location.table_row as usize,  new_table , change_tick);
        }

        B::remove_from_table(&to_remove_component_ids, old_table, old_location.table_row as usize, change_tick);

        let (last_entity, last_row) = self
            .archetypes
            .get_mut(old_location.archetype_id)
            .unwrap()
            .swap_remove(entity_id, old_location.table_row);
        
        //set entity replaced by moved location
        if let Some(entity) = last_entity {
            self.entities.set_location(
                entity, 
                EntityLocation { 
                    archetype_id: old_location.archetype_id, 
                    table_id: old_location.table_id, 
                    table_row: last_row
                }
            );
        }
        
        self.entities.set_location(entity_id, new_location);
        self.bundles.register();
    }

    
    pub fn spawn_at_unchecked<B: Bundle>(
        &mut self,
        entity_id: EntityId,
        bundle: B,
    ) -> EntityId {
        let change_tick = self.change_tick();
        let component_ids: ComponentIdsArray 
            = B::component_ids(&mut self.components_registrator())
            .collect();

        let mut current_component_ids = component_ids.ids().clone();
        current_component_ids.sort(); //REDO

        let archetype_id = self.get_or_create_archetype(current_component_ids);

        let archetype = self.archetypes.get_mut(archetype_id).unwrap();
        let table = &mut self.storages.tables[archetype.table_id() as usize];

        let row = table.add_entity(entity_id);

        

        let location = archetype.insert(entity_id, row as u32);
        
        self.entities.set_location(entity_id, location);
        
        bundle.insert_into_table(&component_ids, table, change_tick);
        self.bundles.register();

        entity_id
    }

    pub fn get_or_create_archetype(&mut self, component_ids : Vec<ComponentId>) -> ArchetypeId {
        let table_components = component_ids.clone();
        if let Some(archetype_id) = self.archetypes.get_id(component_ids) {
            archetype_id
        } else {
            let table_id = self.storages().len();

            let mut table_builder = TableBuilder::with_capacity(0, table_components.len());
            for component_id in table_components.iter() {
                table_builder.add_column(self.components().get_info(*component_id).unwrap());
            }

            let table = table_builder.build();
            self.storages.insert_table(table);
            self.archetypes.insert_id(table_id as u32, table_components)
        }
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
        let component_id = self.components().get_id(TypeId::of::<R>())?;

        self
            .resource_entities
            .get::<R>(component_id)

    }

    #[inline]
    pub fn get_resource_mut<R: Resource>(&mut self) -> Option<Mut<'_, R>> {
        let component_id = self.components().get_id(TypeId::of::<R>())?;

        self
            .resource_entities
            .get_mut::<R>(component_id)
            .map(|r| Mut { value: r})

    }

    pub fn init_resource<R: Resource + Default>(&mut self) {
        let component_id = self.register_resource::<R>();

        if !self.resource_entities().contains::<R>(component_id) {
            self.insert_resource(R::default());
        }
    }

    pub fn insert_resource<R: Resource>(&mut self, value: R) {
        let component_id = self.register_resource::<R>();

        self.resource_entities.insert(component_id, value);
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

    pub fn add_message<M : Message>(&mut self) {
        self.insert_resource(Messages::<M>::default());
    }

    pub(crate) fn increment_trigger_id(&mut self) {
        self.last_trigger_id += 1;
    }

    pub fn trigger_ref<'a, E: Event>(
        &mut self,
        event: &mut E,
    ) {
        unsafe {
            self.trigger_raw( event );
        }
    }

    #[inline]
    pub unsafe fn trigger_raw<'a, E: Event>(
        &mut self,
        event: &mut E,
    ) {
        if let Some(observer) = self.observes.remove::<E>() {
            (observer.observer_function())(self, event as *mut E as *mut u8);
            self.observes.insert::<E>(observer);
        } 
    }

    #[inline]
    #[track_caller]
    pub fn flush(&mut self) {
        let mut command_queue = std::mem::take(&mut self.command_queue);
        command_queue.apply(self);
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

#[cfg(test)]
mod tests {
    use std::{fmt::Display, vec};

use crate::ecs::{components::{Component, Mutable, StorageType}, entities::id::EntityId, query::filter::{With, Without}, systems::commands::{Commands, command::CommandQueue}, worlds::{World, log::print_component_of}};

    #[derive(PartialEq, Eq, Debug)]
    pub struct A;
    impl Component for A {
        const STORAGE_TYPE: StorageType = StorageType::Table;
    
        type Mutability = Mutable;
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct B;
    impl Component for B {
        const STORAGE_TYPE: StorageType = StorageType::Table;
    
        type Mutability = Mutable;
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct C;
    impl Component for C {
        const STORAGE_TYPE: StorageType = StorageType::Table;
    
        type Mutability = Mutable;
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct T(u32);
    impl Component for T {
        const STORAGE_TYPE: StorageType = StorageType::Table;
    
        type Mutability = Mutable;
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct D(Vec<u32>);
    impl Component for D {
        const STORAGE_TYPE: StorageType = StorageType::Table;
    
        type Mutability = Mutable;
    }

    impl Display for D {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[")?;
            let mut iter = self.0.iter();
            if let Some(first) = iter.next() {
                write!(f, "{}", first)?;
            }

            for element in iter {
                write!(f, ", {}", element)?;
            }

            write!(f, "]")
        }
    }

    #[test]
    fn test_spawn_entity_world() {
        let mut world = World::new();
    
        let entity = world.spawn((A, B, C, T(1)));

        let a = world.get_component::<A>(entity);
        let b = world.get_component::<B>(entity);
        let c = world.get_component::<C>(entity);
        let t = world.get_component::<T>(entity);

        assert_eq!(a, Some(&A));
        assert_eq!(b, Some(&B));
        assert_eq!(c, Some(&C));
        assert_eq!(t, Some(&T(1)));
    }

    #[test]
    fn test_spawn_entity_and_insert_world() {
        let mut world = World::new();
    
        let entity = world.spawn((A, T(1)));

        //Replace with new
        world.insert(entity, (T(4),));

        //Insert new
        world.insert(entity, (C,));

        let c = world.get_component::<C>(entity);
        let t = world.get_component::<T>(entity);

        assert_eq!(c, Some(&C));
        assert_eq!(t, Some(&T(4)));
    }

     #[test]
    fn test_spawn_entity_and_remove_world() {
        let mut world = World::new();
    
        let entity = world.spawn((A, T(1)));

        //Remove non-existing
        world.remove::<(C, )>(entity);

        //Remove existing
        world.remove::<(A, )>(entity);

        let a = world.get_component::<A>(entity);
        let t = world.get_component::<T>(entity);

        assert_eq!(a, None);
        assert_eq!(t, Some(&T(1)));
    }

    #[test]
    fn test_spawn_and_edit_entity_vec_world() {
        let mut world = World::new();
    
        let entity = world.spawn((A, T(1)));
        let entity2 = world.spawn((A, D(vec![3, 4, 5, 7, 8, 9])));

        world.insert(entity, (D(vec![3, 4, 5, 7, 8, 9]),));

        let d = world.get_component::<D>(entity);

        assert_eq!(d, Some(&D(vec![3, 4, 5, 7, 8, 9])));

        if let Some(d) = world.get_component_mut::<D>(entity) {
            d.0.push(10);
        }
        
        if let Some(d) = world.get_component_mut::<D>(entity2) {
            d.0.push(10);
        }
    
        let d = world.get_component::<D>(entity);
        let d1 = world.get_component::<D>(entity2);

        assert_eq!(d, Some(&D(vec![3, 4, 5, 7, 8, 9, 10])));
        assert_eq!(d1, Some(&D(vec![3, 4, 5, 7, 8, 9, 10])))
    }   

    #[test]
    fn test_archetype_movement_world() {
        let mut world = World::new();
        let entity = world.spawn((T(1), A));

        let entity2 = world.spawn((T(1), A));

        let mut flipper1 = false;
        let mut flipper2 = false;

        for i in 1..10 {
            if let Some(c) = world.get_component::<C>(entity) {
                world.insert(entity, (A, ));
                world.remove::<(C, )>(entity);
                world.remove::<(T, )>(entity);
            }

            if let Some(c) = world.get_component::<C>(entity2) {
                world.insert(entity2, (A, ));
                world.remove::<(C, )>(entity2);
                world.remove::<(T, )>(entity2);
            }

            if let Some(b) = world.get_component::<B>(entity) {
                world.insert(entity, (C, ));
                world.remove::<(B, )>(entity);
            }

            if let Some(b) = world.get_component::<B>(entity2) {
                world.insert(entity2, (C, ));
                world.remove::<(B, )>(entity2);
            }

            if let Some(_) = world.get_component::<A>(entity) {
                if flipper1 {
                    world.insert(entity, (B, ));
                    world.insert(entity, (T(1), ));
                } else {
                    world.insert(entity, (C, ));
                }
                flipper1 = !flipper1;
                world.remove::<(A, )>(entity);
            }

            if let Some(_) = world.get_component::<A>(entity2) {
                if flipper2 {
                    world.insert(entity2, (B, ));
                    world.insert(entity2, (T(1), ));
                } else {
                    world.insert(entity2, (C, ));
                }
                flipper2 = !flipper2;
                world.remove::<(A, )>(entity2);
            }

            if let Some(t) = world.get_component_mut::<T>(entity) {
                t.0 += 1;
            }

            if let Some(t) = world.get_component_mut::<T>(entity2) {
                t.0 += 1;
            }
        }
        
        assert_eq!(world.archetypes.len(), 10);
        assert_eq!(world.get_component::<T>(entity), Some(&T(3)));
        assert_eq!(world.get_component::<T>(entity2), Some(&T(3)));
    }   

    #[test]
    fn test_query() {
        let mut world = World::new();
        let e1 = world.spawn((A, D(vec![1, 2, 3, 4, 5])));
        let e2 = world.spawn((A, D(vec![5, 4, 3, 2, 1])));
        let e3 = world.spawn((A, D(vec![5, 4, 3, 2, 1, 5, 4, 3, 2, 1])));

        for i in 1..5 {
            let mut query = world.query::<(EntityId, &A, &mut D)>();
            let mut command_queue = CommandQueue::default();
            let mut command = Commands::from(&mut command_queue);
            command.log("init");
            command.log_component::<D>(e3);
            for (entity, a, mut d) in query.iter(&mut world) {
                command.remove::<(A,)>(entity);
            }
            command.log("before");
            command.log_component::<D>(e3);
            let mut query = world.query::<(EntityId, &mut D)>();

            for (entity, mut d) in query.iter(&mut world) {
                assert!(d.0.len() > 0);
                d.0.pop();
                command.add(entity, (A, ));
            }
            
            command.log("after");
            command.log_component::<D>(e3);

            command_queue.apply(&mut world);
        }

        assert_eq!(world.get_component::<D>(e1), Some(&D(vec![1])));
        assert_eq!(world.get_component::<D>(e2), Some(&D(vec![5])));
        assert_eq!(world.get_component::<D>(e3), Some(&D(vec![5, 4, 3, 2, 1, 5])));
    }


    #[test]
    fn test_query_with_without() {
        let mut world = World::new();
        let e1 = world.spawn((A, B));
        let e2 = world.spawn((A, C));
        let e3 = world.spawn((B, C));

        let mut query_1 = world.query_filtered::<(EntityId, &A), With<B>>();
        let mut list_1 = Vec::new();
        for (entity, a) in query_1.iter(&mut world) {
            list_1.push(entity);
        }

        let mut query_2 = world.query_filtered::<(EntityId, &A), Without<C>>();
        let mut list_2 = Vec::new();
        for (entity, a) in query_2.iter(&mut world) {
            list_2.push(entity);
        }

        for (entity) in list_1 {
            assert_eq!(world.get_component::<B>(entity), Some(&B));
        }

        for (entity) in list_2 {
            assert_eq!(world.get_component::<C>(entity), None);
        }

        
    }
}
