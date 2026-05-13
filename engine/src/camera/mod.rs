use crate::{ecs::components::{Component, Mutable, StorageType}, math::{rect::Rect, vec::Vec2}};


pub struct Camera {
    rotation: f32,
    zoom: Vec2,
    target: Vec2,
    offset: Vec2,
    render_target: Option<u32>,  //TODO  
}

impl Component for Camera {
    const STORAGE_TYPE : StorageType = StorageType::Table;

    type Mutability = Mutable;
}
