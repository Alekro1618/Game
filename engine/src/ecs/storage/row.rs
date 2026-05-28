use std::{any::TypeId, collections::HashMap};

use crate::ecs::{components::{Component, id::ComponentId}, storage::element_ref::ElementRef};

#[derive(Clone)]
pub struct RowRef<'a> {
    elements : Vec<ElementRef<'a>>,
}


impl<'a> RowRef<'a> {


    pub fn from( slice : Vec<ElementRef<'a>>) -> Self {
        Self {
            elements:  slice
        }
    }

    pub fn into_iter(self)  -> impl Iterator<Item = ElementRef<'a>> {
        self.elements.into_iter()
    }
}