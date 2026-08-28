
use crate::ecs::{
    components::id::{ComponentId, ComponentIdMap},
    storage::element_ref::{ElementRef, ElementRefMut},
};

#[derive(Clone)]
pub struct RowRef<'a> {
    elements: ComponentIdMap<ElementRef<'a>>,
}

impl<'a> RowRef<'a> {
    pub fn from(vec: ComponentIdMap<ElementRef<'a>>) -> Self {
        Self { elements: vec }
    }

    pub fn into_iter(self) -> impl Iterator<Item = ElementRef<'a>> {
        self.elements.into_iter().map(|(_, value)| value)
    }

    pub fn element_ref(&'a self, component_id: ComponentId) -> ElementRef<'a> {
        self.elements[&component_id]
    }
}

#[derive(Clone)]
pub struct RowRefMut<'a> {
    elements: ComponentIdMap<ElementRefMut<'a>>,
}

impl<'a> RowRefMut<'a> {
    pub fn from(vec: ComponentIdMap<ElementRefMut<'a>>) -> Self {
        Self { elements: vec }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn into_iter(self) -> impl Iterator<Item = ElementRefMut<'a>> {
        self.elements.into_iter().map(|(_, value)| value)
    }

    pub fn element_ref(&'a self, component_id: ComponentId) -> ElementRefMut<'a> {
        self.elements[&component_id]
    }
}
