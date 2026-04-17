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
        let component_id = *table.columns.keys().next().unwrap();
        table.insert(component_id, self.0, tick);
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
    
}