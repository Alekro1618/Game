// src/ecs/testing/assert.rs

use crate::ecs::{
    components::Component,
    entities::id::EntityId, worlds::World,
};

pub struct WorldAssert<'w> {
    world: &'w World,
}

pub struct EntityAssert<'w> {
    world: &'w World,
    entity: EntityId,
}

impl World {
    pub fn assert(&self) -> WorldAssert<'_> {
        WorldAssert { world: self }
    }
}


impl<'w> WorldAssert<'w> {
    pub fn entity_count(self, expected: u32) -> Self {
        let actual = self.world.entity_count();
        assert_eq!(
            actual, expected,
            "expected {expected} entities, found {actual}"
        );
        self
    }

    pub fn entity_exists(self, entity: EntityId) -> Self {
        assert!(
            self.world.entities().is_alive(entity),
            "entity {entity:?} does not exist"
        );
        self
    }

    pub fn entity_dead(self, entity: EntityId) -> Self {
        assert!(
            !self.world.entities().is_alive(entity),
            "entity {entity:?} should be dead but is alive"
        );
        self
    }

    pub fn archetype_count(self, expected: usize) -> Self {
        let actual = self.world.archetypes().len();
        assert_eq!(
            actual, expected,
            "expected {expected} archetypes, found {actual}"
        );
        self
    }

    pub fn entity(self, entity: EntityId) -> EntityAssert<'w> {
        assert!(
            self.world.entities().is_alive(entity),
            "entity {entity:?} does not exist"
        );
        EntityAssert { world: self.world, entity }
    }
}

impl<'w> EntityAssert<'w> {
    pub fn has_component<T: Component + PartialEq + std::fmt::Debug>(
        self,
        expected: T,
    ) -> Self {
        let actual = self.world.get_component::<T>(self.entity);
        assert!(
            actual.is_some(),
            "entity {:?} missing component {}",
            self.entity,
            std::any::type_name::<T>()
        );
        assert_eq!(
            actual.unwrap(),
            &expected,
            "entity {:?} component {} value mismatch",
            self.entity,
            std::any::type_name::<T>()
        );
        self
    }

    pub fn has<T: Component>(self) -> Self {
        assert!(
            self.world.get_component::<T>(self.entity).is_some(),
            "entity {:?} missing component {}",
            self.entity,
            std::any::type_name::<T>()
        );
        self
    }

    pub fn has_not<T: Component>(self) -> Self {
        assert!(
            self.world.get_component::<T>(self.entity).is_none(),
            "entity {:?} should not have component {} but does",
            self.entity,
            std::any::type_name::<T>()
        );
        self
    }

    pub fn component_matches<T: Component>(
        self,
        predicate: impl FnOnce(&T) -> bool,
        msg: &str,
    ) -> Self {
        let component = self.world
            .get_component::<T>(self.entity)
            .unwrap_or_else(|| panic!(
                "entity {:?} missing component {}",
                self.entity,
                std::any::type_name::<T>()
            ));
        assert!(predicate(component), "entity {:?}: {msg}", self.entity);
        self
    }

    pub fn world(self) -> WorldAssert<'w> {
        WorldAssert { world: self.world }
    }
}