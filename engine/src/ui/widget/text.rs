use crate::{
    ecs::components::{Component, Mutable, StorageType},
    render::color::Color,
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Text(pub String);

impl Component for Text {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Mutable;
}

impl Text {
    /// Makes a new text component.
    pub fn new(text: impl Into<String>) -> Self {
        Self(text.into())
    }
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone)]
pub struct TextFont(pub macroquad::text::Font);

impl Component for TextFont {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Mutable;
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct TextSize(pub u16);

impl Component for TextSize {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Mutable;
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TextColor(pub Color);

impl Component for TextColor {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Mutable;
}
