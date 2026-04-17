use crate::{ecs::components::{Component, Mutable, StorageType}, math::{rect::Rect, vec::Vec2}};


pub struct Camera {
    _view : Rect,
    _rotation : f32,
    _direction : Vec2
    
}

impl Component for Camera {
    const STORAGE_TYPE : StorageType = StorageType::Table;

    type Mutability = Mutable;
}