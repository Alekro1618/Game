use std::{any::{TypeId}, iter};

use crate::ecs::{bundle::Bundle, change_detection::tick::Tick, components::{Component, id::{ComponentId, ComponentIdsArray}, register::{ComponentsRegistrator}}};

#[derive(Debug, Default)]
pub struct Bundles {
    count: usize,
}

impl Bundles {
    pub fn register(&mut self) -> usize {
        let id = self.count;
        self.count += 1;
        id
    }
}

impl<A: Component + 'static> Bundle for (A,)  {
    fn component_ids(
        components: &mut crate::ecs::components::register::ComponentsRegistrator,
    ) -> impl Iterator<Item = (TypeId, ComponentId)> + use<A> {
        iter::once((TypeId::of::<A>(), components.register_component::<A>()))
    }

    fn insert_into_table(
        self, 
        components : &ComponentIdsArray,
        table: &mut crate::ecs::storage::Table,
        tick : Tick
    ) {
        let component_id = components.get(TypeId::of::<A>()).unwrap();
        table.insert(component_id, self.0, tick);
    }
    
    fn replace_in_table(self, components : &ComponentIdsArray, table: &mut crate::ecs::storage::Table, row : usize, tick : Tick) {
        let component_id = components.get(TypeId::of::<A>()).unwrap();
        table.replace(component_id, self.0, row, tick);
    }
    
    fn remove_from_table(components : &ComponentIdsArray, table: &mut crate::ecs::storage::Table, row : usize, tick : Tick) {
        let component_id = components.get(TypeId::of::<A>()).unwrap();
        table.remove::<A>(component_id, row, tick);
    }
}

impl<A: Component + 'static, B: Component + 'static> Bundle for (A, B)  {
    fn component_ids(
        components: &mut ComponentsRegistrator,
    ) -> impl Iterator<Item = (TypeId, ComponentId)> + use<A, B> {
        let a = components.register_component::<A>();
        let b = components.register_component::<B>();
        iter::once((TypeId::of::<A>(),a)).chain(iter::once((TypeId::of::<B>(),b)))
    }

    fn insert_into_table(
        self, 
        components : &ComponentIdsArray,
        table: &mut crate::ecs::storage::Table,
        tick : Tick
    ) {
        let component_id_1 = components.get(TypeId::of::<A>()).unwrap();
        let component_id_2 = components.get(TypeId::of::<B>()).unwrap();
        table.insert(component_id_1, self.0, tick);
        table.insert(component_id_2, self.1, tick);
    }
    
    fn replace_in_table(self, components : &ComponentIdsArray, table: &mut crate::ecs::storage::Table, row : usize, tick : Tick) {
        let component_id_1 = components.get(TypeId::of::<A>()).unwrap();
        let component_id_2 = components.get(TypeId::of::<B>()).unwrap();
        table.replace(component_id_1, self.0,row, tick);
        table.replace(component_id_2, self.1, row, tick);
    }
    
    fn remove_from_table(components : &ComponentIdsArray, table: &mut crate::ecs::storage::Table, row : usize, tick : Tick) {
        let component_id_1 = components.get(TypeId::of::<A>()).unwrap();
        let component_id_2 = components.get(TypeId::of::<B>()).unwrap();
        table.remove::<A>(component_id_1, row, tick);
        table.remove::<B>(component_id_2, row, tick);
    }
    
}

impl<A: Component + 'static, B: Component + 'static, C: Component + 'static> Bundle for (A, B, C)  {
    fn component_ids(
        components: &mut ComponentsRegistrator,
    ) -> impl Iterator<Item = (TypeId, ComponentId)> + use<A, B, C> {
        let a = components.register_component::<A>();
        let b = components.register_component::<B>();
        let c = components.register_component::<C>();
        //iter::once((TypeId::of::<A>(),a)).chain(iter::once((TypeId::of::<B>(),b)).chain(iter::once((TypeId::of::<C>(),c))))
        vec![(TypeId::of::<A>(),a), (TypeId::of::<B>(),b), (TypeId::of::<C>(),c)].into_iter()
    }

    fn insert_into_table(
        self, 
        components : &ComponentIdsArray,
        table: &mut crate::ecs::storage::Table,
        tick : Tick
    ) {
        let component_id_1 = components.get(TypeId::of::<A>()).unwrap();
        let component_id_2 = components.get(TypeId::of::<B>()).unwrap();
        let component_id_3 = components.get(TypeId::of::<C>()).unwrap();
        table.insert(component_id_1, self.0, tick);
        table.insert(component_id_2, self.1, tick);
        table.insert(component_id_3, self.2, tick);
    }
    
    fn replace_in_table(self, components : &ComponentIdsArray, table: &mut crate::ecs::storage::Table, row : usize, tick : Tick) {
        let component_id_1 = components.get(TypeId::of::<A>()).unwrap();
        let component_id_2 = components.get(TypeId::of::<B>()).unwrap();
        let component_id_3 = components.get(TypeId::of::<C>()).unwrap();
        table.replace(component_id_1, self.0,row, tick);
        table.replace(component_id_2, self.1, row, tick);
        table.replace(component_id_3, self.2, row, tick);
    }
    
    fn remove_from_table(components : &ComponentIdsArray, table: &mut crate::ecs::storage::Table, row : usize, tick : Tick) {
        let component_id_1 = components.get(TypeId::of::<A>()).unwrap();
        let component_id_2 = components.get(TypeId::of::<B>()).unwrap();
        let component_id_3 = components.get(TypeId::of::<C>()).unwrap();
        table.remove::<A>(component_id_1, row, tick);
        table.remove::<B>(component_id_2, row, tick);
        table.remove::<C>(component_id_3, row, tick);
    }
    
}

impl<A: Component + 'static, B: Component + 'static, C: Component + 'static, D: Component + 'static> Bundle for (A, B, C, D)  {
    fn component_ids(
        components: &mut ComponentsRegistrator,
    ) -> impl Iterator<Item = (TypeId, ComponentId)> + use<A, B, C, D> {
        let a = components.register_component::<A>();
        let b = components.register_component::<B>();
        let c = components.register_component::<C>();
        let d = components.register_component::<D>();
        vec![(TypeId::of::<A>(),a), (TypeId::of::<B>(),b), (TypeId::of::<C>(),c), (TypeId::of::<D>(), d)].into_iter()
    }

    fn insert_into_table(
        self, 
        components : &ComponentIdsArray,
        table: &mut crate::ecs::storage::Table,
        tick : Tick
    ) {
        let component_id_1 = components.get(TypeId::of::<A>()).unwrap();
        let component_id_2 = components.get(TypeId::of::<B>()).unwrap();
        let component_id_3 = components.get(TypeId::of::<C>()).unwrap();
        let component_id_4 = components.get(TypeId::of::<D>()).unwrap();
        table.insert(component_id_1, self.0, tick);
        table.insert(component_id_2, self.1, tick);
        table.insert(component_id_3, self.2, tick);
        table.insert(component_id_4, self.3, tick);
    }
    
    fn replace_in_table(self, components : &ComponentIdsArray, table: &mut crate::ecs::storage::Table, row : usize, tick : Tick) {
        let component_id_1 = components.get(TypeId::of::<A>()).unwrap();
        let component_id_2 = components.get(TypeId::of::<B>()).unwrap();
        let component_id_3 = components.get(TypeId::of::<C>()).unwrap();
        let component_id_4 = components.get(TypeId::of::<D>()).unwrap();
        table.replace(component_id_1, self.0, row, tick);
        table.replace(component_id_2, self.1, row, tick);
        table.replace(component_id_3, self.2, row, tick);
        table.replace(component_id_4, self.3, row, tick);
    }
    
    fn remove_from_table(components : &ComponentIdsArray, table: &mut crate::ecs::storage::Table, row : usize, tick : Tick) {
        let component_id_1 = components.get(TypeId::of::<A>()).unwrap();
        let component_id_2 = components.get(TypeId::of::<B>()).unwrap();
        let component_id_3 = components.get(TypeId::of::<C>()).unwrap();
        let component_id_4 = components.get(TypeId::of::<D>()).unwrap();
        table.remove::<A>(component_id_1, row, tick);
        table.remove::<B>(component_id_2, row, tick);
        table.remove::<C>(component_id_3, row, tick);
        table.remove::<D>(component_id_4, row, tick);
    }
    
}


impl<A: Component + 'static, B: Component + 'static, C: Component + 'static, D: Component + 'static, E: Component + 'static> Bundle for (A, B, C, D, E)  {
    fn component_ids(
        components: &mut ComponentsRegistrator,
    ) -> impl Iterator<Item = (TypeId, ComponentId)> + use<A, B, C, D, E> {
        let a = components.register_component::<A>();
        let b = components.register_component::<B>();
        let c = components.register_component::<C>();
        let d = components.register_component::<D>();
        let e = components.register_component::<E>();
        vec![(TypeId::of::<A>(),a), (TypeId::of::<B>(),b), (TypeId::of::<C>(),c), (TypeId::of::<D>(), d), (TypeId::of::<E>(), e)].into_iter()
    }

    fn insert_into_table(
        self, 
        components : &ComponentIdsArray,
        table: &mut crate::ecs::storage::Table,
        tick : Tick
    ) {
        let component_id_1 = components.get(TypeId::of::<A>()).unwrap();
        let component_id_2 = components.get(TypeId::of::<B>()).unwrap();
        let component_id_3 = components.get(TypeId::of::<C>()).unwrap();
        let component_id_4 = components.get(TypeId::of::<D>()).unwrap();
        let component_id_5 = components.get(TypeId::of::<E>()).unwrap();
        table.insert(component_id_1, self.0, tick);
        table.insert(component_id_2, self.1, tick);
        table.insert(component_id_3, self.2, tick);
        table.insert(component_id_4, self.3, tick);
        table.insert(component_id_5, self.4, tick);
    }
    
    fn replace_in_table(self, components : &ComponentIdsArray, table: &mut crate::ecs::storage::Table, row : usize, tick : Tick) {
        let component_id_1 = components.get(TypeId::of::<A>()).unwrap();
        let component_id_2 = components.get(TypeId::of::<B>()).unwrap();
        let component_id_3 = components.get(TypeId::of::<C>()).unwrap();
        let component_id_4 = components.get(TypeId::of::<D>()).unwrap();
        let component_id_5 = components.get(TypeId::of::<E>()).unwrap();
        table.replace(component_id_1, self.0, row, tick);
        table.replace(component_id_2, self.1, row, tick);
        table.replace(component_id_3, self.2, row, tick);
        table.replace(component_id_4, self.3, row, tick);
        table.replace(component_id_5, self.4, row, tick);
    }
    
    fn remove_from_table(components : &ComponentIdsArray, table: &mut crate::ecs::storage::Table, row : usize, tick : Tick) {
        let component_id_1 = components.get(TypeId::of::<A>()).unwrap();
        let component_id_2 = components.get(TypeId::of::<B>()).unwrap();
        let component_id_3 = components.get(TypeId::of::<C>()).unwrap();
        let component_id_4 = components.get(TypeId::of::<D>()).unwrap();
        let component_id_5 = components.get(TypeId::of::<E>()).unwrap();
        table.remove::<A>(component_id_1, row, tick);
        table.remove::<B>(component_id_2, row, tick);
        table.remove::<C>(component_id_3, row, tick);
        table.remove::<D>(component_id_4, row, tick);
        table.remove::<E>(component_id_5, row, tick);
    }
    
}