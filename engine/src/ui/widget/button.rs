use engine_macros::Bundle;
use crate::ecs::bundle;

use crate::ecs::event::Event;
use crate::ui::click::{ClickEvent, OnClick};
use crate::ui::ui_node::Node;
use crate::ui::widget::Widget;
use crate::{ecs::components::{Component, Mutable, StorageType}};

/// Marker struct for buttons
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Button;

impl Component for Button {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Mutable;
}
