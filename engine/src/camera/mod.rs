use crate::{ecs::components::{Component, Mutable, StorageType}, math::{rect::Rect, transform::Transform, vec::Vec2}};


pub struct Camera {
    view : Rect,
    rotation : f32,
    direction : Vec2
    
}

impl Component for Camera {
    const STORAGE_TYPE : StorageType = StorageType::Table;

    type Mutability = Mutable;
}