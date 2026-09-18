//! This module contains the basic building blocks of Bevy's UI

mod button;
mod image;
mod label;
mod text;
mod viewport;

pub use button::*;
use engine_macros::Component;
pub use image::*;
pub use label::*;
pub use text::*;
pub use viewport::*;

use crate::{ecs::worlds::World, ui::{modifier::Modifier, ui_node::Node}};

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct WidgetId(u32);

pub trait Widget {

    fn compose(&self, widget_id : WidgetId, world : &mut World);

    fn update(&mut self, widget_id : WidgetId, world : &mut World);
}

pub struct WidgetTree {
    widget_id : WidgetId,
    widget : Box<dyn Widget>,
    inner : Vec<Box<WidgetTree>>
}

impl WidgetTree {
    fn compose(&self, world : &mut World) {
        self.widget.compose(self.widget_id, world);
        self.inner.iter().for_each(|wt| wt.compose(world));
    }

    fn update(&mut self, world : &mut World) {
        self.widget.update(self.widget_id, world);
        self.inner.iter_mut().for_each(|wt| wt.update(world));
    }
}