use std::{collections::HashMap, sync::LazyLock};

use image::{ImageError, ImageFormat};

use crate::{
    app::{main_schedule::ScheduleLabel, plugin::Plugin}, asset::{
        asset::{Asset, AssetIndex, Assets},
        handle::Handle,
        loader::AssetLoader,
    }, ecs::{
        change_detection::params::Mut,
        components::{Component, Mutable},
        resource::Resource,
        worlds::FromWorld,
    }, error::error::{Error, ErrorKind}, math::vec::Vec2, utils::SparseArray,
};

#[derive(Clone)]
pub struct Image {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl Image {
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32)
    }
}

pub struct ImageLoader {
    extension: ImageFormat,
}

///TODO:macroquad
#[derive(Default)]
pub struct TempQuadImageStorage {
    index_to_texture: SparseArray<macroquad::texture::Texture2D, AssetIndex>,
}

impl Component for TempQuadImageStorage {
    type Mutability = Mutable;
}

impl Resource for TempQuadImageStorage {}

impl TempQuadImageStorage {
    pub fn get(&self, asset_index: AssetIndex) -> Option<&macroquad::texture::Texture2D> {
        self.index_to_texture.get(asset_index)
    }
}

impl AssetLoader for ImageLoader {
    type Asset = Image;
    type Error = ImageError;

    fn load(
        &self,
        reader: &mut Vec<u8>,
        _load_context: &mut crate::asset::loader::LoadContext,
    ) -> impl Future<Output = Result<Self::Asset, Self::Error>> + Send {
        async move {
            let img = image::load_from_memory_with_format(&reader, self.extension)?;

            let (width, height) = (img.width(), img.height());
            let rgba_bytes = img.to_rgba8().into_raw();
            Ok(Image {
                data: rgba_bytes,
                width,
                height,
            })
        }
    }
}

impl FromWorld for ImageLoader {
    fn from_world(_world: &mut crate::ecs::worlds::World) -> Self {
        Self {
            extension: ImageFormat::Png,
        }
    }
}

impl Into<Error> for ImageError {
    fn into(self) -> Error {
        Error::new(ErrorKind::Asset, self.to_string())
    }
}
/// For the time being we clone all the handles from our system to macroquad.
fn translate_images_to_macroquad(
    mut temp_storage: Mut<TempQuadImageStorage>,
    mut assets: Mut<Assets<Image>>,
) {
    let indicies: Vec<AssetIndex> = assets.iter_indicies().collect();
    for asset_index in indicies {
        let Some(asset) = assets.remove(asset_index) else {
            continue;
        };

        let texture_2d = macroquad::texture::Texture2D::from_image(&asset.into());

        temp_storage
            .index_to_texture
            .insert(asset_index, texture_2d);
    }
}

impl Asset for Image {}

pub const TRANSPARENT_IMAGE_HANDLE: LazyLock<Handle<Image>> =
    LazyLock::new(|| Handle::new(AssetIndex::from_bits(u64::MAX), "".into()));

pub const BLANK_IMAGE_HANDLE: LazyLock<Handle<Image>> =
    LazyLock::new(|| Handle::new(AssetIndex::from_bits(u64::MAX-1), "".into()));

pub struct ImagePlugin;

impl Plugin for ImagePlugin {
    fn build(self, app: &mut crate::app::App) {
        app.init_asset::<Image>()
            .init_asset_loader::<ImageLoader>()
            .add_systems(
                ScheduleLabel::PostUpdate,
                translate_images_to_macroquad,
            );
    }
}
