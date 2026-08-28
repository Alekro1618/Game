use std::{
    any::TypeId,
    collections::HashMap,
    sync::Arc,
};

use crate::{
    asset::loader::{AssetLoader, ErasedAssetLoader},
    ecs::components::info::TypeIdMap,
};

#[derive(Default)]
pub(crate) struct AssetLoaders {
    loaders: Vec<Arc<dyn ErasedAssetLoader>>,
    type_id_to_loaders: TypeIdMap<Vec<usize>>,
    extension_to_loaders: HashMap<Box<str>, Vec<usize>>,
    path_to_loader: HashMap<&'static str, usize>,
    path_to_preregistered_loader: HashMap<&'static str, usize>,
}

impl AssetLoaders {
    fn get_by_index(&self, index: usize) -> Option<Arc<dyn ErasedAssetLoader>> {
        self.loaders.get(index).cloned()
    }

    pub(crate) fn push<L: AssetLoader>(&mut self, loader: L) {
        let type_name = L::type_name();
        let loader_asset_type = TypeId::of::<L::Asset>();

        let loader = Arc::new(loader);

        let (loader_index, is_new) =
            if let Some(index) = self.path_to_preregistered_loader.remove(type_name) {
                (index, false)
            } else {
                (self.loaders.len(), true)
            };

        if is_new {
            let existing_loaders_for_type_id = self.type_id_to_loaders.get(&loader_asset_type);

            let mut duplicate_extensions = Vec::new();

            for extension in AssetLoader::extensions(&*loader) {
                let list = self
                    .extension_to_loaders
                    .entry((*extension).into())
                    .or_default();

                if !list.is_empty()
                    && let Some(existing_loaders_for_type_id) = existing_loaders_for_type_id
                    && list
                        .iter()
                        .any(|index| existing_loaders_for_type_id.contains(index))
                {
                    duplicate_extensions.push(extension);
                }

                list.push(loader_index);
            }

            self.path_to_loader.insert(type_name, loader_index);

            self.type_id_to_loaders
                .entry(loader_asset_type)
                .or_default()
                .push(loader_index);

            self.loaders.push(loader);
        } else {
            let _ = core::mem::replace(self.loaders.get_mut(loader_index).unwrap(), loader.clone());
        }
    }

    pub(crate) fn find(
        &self,
        asset_type_id: Option<TypeId>,
        extension: Option<&str>,
        asset_path: Option<&'static str>,
    ) -> Option<Arc<dyn ErasedAssetLoader>> {
        // Try by asset type
        let candidates = if let Some(type_id) = asset_type_id {
            Some(self.type_id_to_loaders.get(&type_id)?)
        } else {
            None
        };

        if let Some(candidates) = candidates {
            if candidates.is_empty() {
                return None;
            } else if candidates.len() == 1 {
                let index = candidates.first().copied().unwrap();
                return self.get_by_index(index);
            }
        }

        let try_extension = |extension| {
            if let Some(indices) = self.extension_to_loaders.get(extension) {
                if let Some(candidates) = candidates {
                    if candidates.is_empty() {
                        indices.last()
                    } else {
                        indices
                            .iter()
                            .rev()
                            .find(|index| candidates.contains(index))
                    }
                } else {
                    indices.last()
                }
            } else {
                None
            }
        };

        // Try the provided extension
        if let Some(extension) = extension
            && let Some(&index) = try_extension(extension)
        {
            return self.get_by_index(index);
        }

        // Try extracting the extension from the path
        if let Some(full_extension) = asset_path {
            if let Some(&index) = try_extension(full_extension) {
                return self.get_by_index(index);
            }
        }

        // Fallback if no resolution step was conclusive
        match candidates?
            .last()
            .copied()
            .and_then(|index| self.get_by_index(index))
        {
            Some(loader) => {
                println!(
                    "Multiple AssetLoaders found for Asset: {:?}; Path: {:?}; Extension: {:?}",
                    asset_type_id, asset_path, extension
                );
                Some(loader)
            }
            None => {
                println!(
                    "No AssetLoader found for Asset: {:?}; Path: {:?}; Extension: {:?}",
                    asset_type_id, asset_path, extension
                );
                None
            }
        }
    }
}
