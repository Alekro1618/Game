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


#[macro_export]
macro_rules! impl_bundle_for_tuple {
    ($($comp:ident),+ $(,)?) => {
        impl<$($comp: Component + 'static),+> Bundle for ($($comp,)+) {
            fn component_ids(
                components: &mut ComponentsRegistrator,
            ) -> impl Iterator<Item = (TypeId, ComponentId)> + use<$($comp),+> {
                vec![
                    $(
                        (TypeId::of::<$comp>(), components.register_component::<$comp>()),
                    )+
                ].into_iter()
            }

            fn insert_into_table(
                self,
                components: &ComponentIdsArray,
                table: &mut crate::ecs::storage::Table,
                tick: Tick
            ) {
                let ($($comp,)+) = self;
                $(
                    let component_id = components.get(TypeId::of::<$comp>()).unwrap();
                    table.insert(component_id, $comp, tick);
                )+
            }

            fn replace_in_table(
                self,
                components: &ComponentIdsArray,
                table: &mut crate::ecs::storage::Table,
                row: usize,
                tick: Tick
            ) {
                let ($($comp,)+) = self;
                $(
                    let component_id = components.get(TypeId::of::<$comp>()).unwrap();
                    table.replace(component_id, $comp, row, tick);
                )+
            }

            fn remove_from_table(
                components: &ComponentIdsArray,
                table: &mut crate::ecs::storage::Table,
                row: usize,
                tick: Tick
            ) {
                $(
                    let component_id = components.get(TypeId::of::<$comp>()).unwrap();
                    table.remove::<$comp>(component_id, row, tick);
                )+
            }
        }
    };
}

impl_bundle_for_tuple!(A);
impl_bundle_for_tuple!(A, B);
impl_bundle_for_tuple!(A, B, C);
impl_bundle_for_tuple!(A, B, C, D);
impl_bundle_for_tuple!(A, B, C, D, E);
impl_bundle_for_tuple!(A, B, C, D, E, F);
impl_bundle_for_tuple!(A, B, C, D, E, F, G);
impl_bundle_for_tuple!(A, B, C, D, E, F, G, H);