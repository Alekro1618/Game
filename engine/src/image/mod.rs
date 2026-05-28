use std::sync::LazyLock;

use crate::{asset::{asset::{Asset, AssetIndex}, handle::Handle}, math::vec::Vec2};


pub struct Image {
    pub data: Vec<u8>,
    pub width: u16,
    pub height: u16,
}
impl Image {
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32)
    }
}

impl Asset for Image {}

pub const TRANSPARENT_IMAGE_HANDLE: LazyLock<Handle<Image>>
    = LazyLock::new(|| Handle::new(AssetIndex::from_bits(0xFFFFFFFFFFFFFFFF)));

pub const BLANK_IMAGE_HANDLE: LazyLock<Handle<Image>>
    = LazyLock::new(|| Handle::new(AssetIndex::from_bits(0xFFFFFFFFFFFFFFFF)));