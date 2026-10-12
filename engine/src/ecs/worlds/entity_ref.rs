use std::cell::Cell;

use crate::ecs::{
    components::{Component, id::ComponentId, info::Components}, entities::{EntityLocation, id::EntityId}, storage::row::{RowRef, RowRefMut},
};


/// Represents entity reference. Allows user to get reference to each component.
pub struct EntityRef<'w> {
    components: &'w Components,
    entity: EntityId,
    location: EntityLocation,
    row_ref: RowRef<'w>,
}

impl<'w> EntityRef<'w> {
    pub fn new(
        components: &'w Components,
        entity: EntityId,
        location: EntityLocation,
        row_ref: RowRef<'w>,
    ) -> Self {
        Self {
            components,
            entity,
            location,
            row_ref,
        }
    }

    pub fn get<T: Component>(&'w self) -> Option<&'w T> {
        let comp_id = self.components.component_id::<T>()?;

        //Safety: idk 
        unsafe { Some(self.row_ref.element_ref(comp_id).get()) }
    }

    pub fn contains<T: Component>(&'w self) -> bool {
        let Some(comp_id) =  self.components.component_id::<T>() else {
            return false;
        };

        self.row_ref.contains(comp_id)
    }

    pub(crate) fn id(&'w self) -> EntityId {
        self.entity
    }

    pub(crate) fn location(&'w self) -> EntityLocation {
        self.location
    }
}


/// Represents mutable entity reference. Allows user to get (mutable) reference to each component.
/// Allows for mulitple mutable reference which is unchecked by compiler, but will panic at runtime if user does it.
pub struct EntityMutRef<'w> {
    components: &'w Components,
    entity: EntityId,
    location: EntityLocation,
    row_ref_mut: RowRefMut<'w>,
    borrows: Vec<Cell<bool>>,
}

impl<'w> EntityMutRef<'w> {
    pub fn new(
        components: &'w Components,
        entity: EntityId,
        location: EntityLocation,
        row_ref_mut: RowRefMut<'w>,
    ) -> Self {
        let borrows = vec![Cell::new(false); row_ref_mut.len()];
        Self {
            components,
            entity,
            location,
            row_ref_mut,
            borrows,
        }
    }

    pub fn get<T: Component>(&'w self) -> Option<&'w T> {
        let comp_id = self.components.component_id::<T>()?;
        self.check_for_conflicts(comp_id);

        //Safety: check_for_conflicts is ran before the unsafe part
        unsafe { Some(self.row_ref_mut.element_ref(comp_id).get()) }
    }

    pub fn get_mut<T: Component>(&'w self) -> Option<&'w mut T> {
        let comp_id = self.components.component_id::<T>()?;
        self.check_for_conflicts(comp_id);

        //Safety: check_for_conflicts is ran before the unsafe part
        unsafe { Some(self.row_ref_mut.element_ref(comp_id).get_mut()) }
    }

    pub fn contains<T: Component>(&'w self) -> bool {
        let Some(comp_id) =  self.components.component_id::<T>() else {
            return false;
        };

        self.row_ref_mut.contains(comp_id)
    }

    fn check_for_conflicts(&'w self, comp_id: ComponentId) {
        let id = comp_id.id as usize;
        assert!(!self.borrows[id].get());

        self.borrows[id].set(true);
    }

    pub(crate) fn id(&'w self) -> EntityId {
        self.entity
    }

    pub(crate) fn location(&'w self) -> EntityLocation {
        self.location
    }
}
