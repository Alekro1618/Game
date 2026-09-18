use std::{
    any::{Any, TypeId},
    panic::AssertUnwindSafe,
    sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use async_channel::{Receiver, Sender, unbounded};
use serde::{Deserialize, Serialize};

use crate::{
    app::{main_schedule::ScheduleLabel, plugin::Plugin}, asset::{
        asset::{Asset, AssetIndex, Assets},
        handle::{AssetHandleProvider, Handle, StrongHandle},
        info::AssetInfos,
        loader::{AssetLoader, ErasedAssetLoader, ErasedLoadedAsset, LoadContext},
        service::loaders::AssetLoaders,
    }, ecs::{
        change_detection::params::Mut, components::{Component, Mutable, StorageType}, resource::Resource, worlds::World,
    }, error::error::{Error, ErrorKind},
};

pub mod asset;
pub mod handle;
mod info;
pub mod loader;
mod service;

#[derive(Debug, Clone, Copy)]
pub struct AssetPath {
    path : &'static str
}

impl Into<AssetPath> for &'static str {
    fn into(self) -> AssetPath {
        AssetPath {
            path: self
        }
    }
}

impl Into<&'static str> for AssetPath {
    fn into(self) -> &'static str {
        self.path
    }
}

impl Into<&'static str> for &AssetPath {
    fn into(self) -> &'static str {
        self.path
    }
}

impl From<Box<str>> for AssetPath {
    fn from(value: Box<str>) -> Self {
        Self {
            path: Box::leak(value)
        }
    }
}


impl Serialize for AssetPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(self.path)
    }
}

impl<'de> Deserialize<'de> for AssetPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let path = String::deserialize(deserializer)?;

        Ok(path.into_boxed_str().into())
    }
}

#[derive(Clone)]
pub struct AssetServer {
    pub(crate) data: Arc<AssetServerData>,
    pub(crate) path : &'static str
}

impl Default for  AssetServer {
    fn default() -> Self {
        Self {
            data: Default::default(), 
            path: "" 
        }
    }
}

impl Component for AssetServer {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Mutable;
}

impl Resource for AssetServer {}

pub(crate) struct AssetServerData {
    pub(crate) infos: RwLock<AssetInfos>,
    pub(crate) loaders: Arc<RwLock<AssetLoaders>>,
    pub sender: Sender<InternalAssetEvent>,
    pub reciever: Receiver<InternalAssetEvent>,
}

pub(crate) enum InternalAssetEvent {
    Loaded {
        index: AssetIndex,
        loaded_asset: ErasedLoadedAsset,
    },
    Failed {
        index: AssetIndex,
        path: &'static str,
        error: Error,
    },
}
/// The "asset mode" the server is currently in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AssetServerMode {
    /// This server loads unprocessed assets.
    Unprocessed,
    /// This server loads processed assets.
    Processed,
}

impl Default for AssetServerData {
    fn default() -> Self {
        let (sender, reciever) = unbounded();
        Self {
            infos: Default::default(),
            loaders: Default::default(),
            sender,
            reciever,
        }
    }
}

impl AssetServer {
    pub fn new(path : &'static str) -> Self {
        let (sender, reciever) = unbounded();
        Self {
            data: Arc::new(AssetServerData {
                infos: RwLock::new(AssetInfos::new()),
                loaders: Arc::new(RwLock::new(AssetLoaders::default())),
                sender,
                reciever,
            }),
            path
        }
    }

    pub(crate) fn write_infos(&self) -> RwLockWriteGuard<'_, AssetInfos> {
        self.data
            .infos
            .write()
            .unwrap_or_else(PoisonError::into_inner)
    }

    pub(crate) fn read_infos(&self) -> RwLockReadGuard<'_, AssetInfos> {
        self.data
            .infos
            .read()
            .unwrap_or_else(PoisonError::into_inner)
    }

    fn write_loaders(&self) -> RwLockWriteGuard<'_, AssetLoaders> {
        self.data
            .loaders
            .write()
            .unwrap_or_else(PoisonError::into_inner)
    }

    fn read_loaders(&self) -> RwLockReadGuard<'_, AssetLoaders> {
        self.data
            .loaders
            .read()
            .unwrap_or_else(PoisonError::into_inner)
    }

    pub fn register_loader<L: AssetLoader>(&self, loader: L) {
        self.write_loaders().push(loader);
    }

    pub(crate) fn register_handle_provider(&self, handle_provider: AssetHandleProvider) {
        self.write_infos()
            .handle_providers
            .insert(handle_provider.type_id, handle_provider);
    }

    pub fn register_asset<A: Asset>(&self, assets: &Assets<A>) {
        self.register_handle_provider(assets.get_handle_provider());
    }

    pub fn load<A: Asset>(&self, path: impl Into<&'static str>) -> Handle<A> {
        let path = path.into();

        let mut infos = self.write_infos();
        let (handle, should_load) = infos.get_or_create_path_handle::<A>(path, false);

        if should_load {
            self.spawn_load_task(handle.clone().into(), path, infos);
        }

        handle
    }

    pub fn prepare_without_loading<A: Asset>(&self, path: impl Into<&'static str>) -> Handle<A> {
        let path = path.into();

        let mut infos = self.write_infos();
        let (handle, _) = infos.get_or_create_path_handle::<A>(path, true);

        handle
    }

    pub fn finally_load<A: Asset>(&self, path: impl Into<&'static str>) -> Handle<A> {
        let path = path.into();

        let mut infos = self.write_infos();
        let handle = infos.get_path_handle::<A>(path).expect(&format!("No handle for path {}", path));

        self.spawn_load_task(handle.clone().into(), path, infos);

        handle
    }

    pub(crate) fn spawn_load_task(
        &self,
        handle: StrongHandle,
        path: &'static str,
        infos: RwLockWriteGuard<AssetInfos>,
    ) {
        let handle_index = handle.index();
        let server = self.clone();

        //TODO:macroquad
        let task = macroquad::experimental::coroutines::start_coroutine(async move {
            println!("Trying to load asset {}", path);
            if let Err((_, path, error)) = server
                .load_internal(handle, path, false)
                .await {
                println!("Error while loading {}: {}", path, error);
            }
        });

        let mut infos = infos;
        infos.pending_tasks.insert(handle_index, task);
    }

    async fn load_internal(
        &self,
        input_handle: StrongHandle,
        path: &'static str,
        force: bool,
    ) -> Result<Option<Arc<StrongHandle>>, (AssetIndex, &'static str, Error)> {
        let input_handle_type_id = input_handle.type_id();
        let path_clone = path;
        let (loader, mut reader) = self
            .get_loader_and_reader(&path_clone, input_handle_type_id)
            .await
            .map_err(|err| (input_handle.index(), path, err))?;

        let asset_index = input_handle.index();
        let fetched_handle = None;
        let should_load = true;

        // Bail out earlier if we don't need to load the asset.
        if !should_load && !force {
            return Ok(fetched_handle);
        }

        // We don't actually need to use _base_handle, but we do need to keep the handle alive.
        // Dropping it would cancel the load of the base asset, which would make the load of this
        // subasset never complete.
        let (base_asset_id, _base_handle, base_path) = {
            let mut infos = self.write_infos();
            let base_handle = infos
                .get_or_create_path_handle_internal(path, Some(loader.asset_type_id()), false)
                .map_err(|err| (asset_index, path, err))?
                .0;
            (&base_handle.index(), Some(base_handle), path)
        };

        match self
            .load_with_settings_loader_and_reader(&base_path, &*loader, &mut reader)
            .await
        {
            Ok(loaded_asset) => {
                self.send_message(InternalAssetEvent::Loaded {
                    index: base_asset_id.clone(),
                    loaded_asset,
                })
                .await;

                Ok(fetched_handle)
            }
            Err(err) => Err((asset_index, path, err)),
        }
    }

    pub(crate) async fn load_with_settings_loader_and_reader(
        &self,
        asset_path: &'static str,
        loader: &dyn ErasedAssetLoader,
        reader: &mut Vec<u8>,
    ) -> Result<ErasedLoadedAsset, Error> {
        let load_context = LoadContext::new(self, asset_path);

        let load = AssertUnwindSafe(loader.load(reader, load_context));
        load.await
    }

    pub(crate) async fn get_loader_and_reader(
        &self,
        asset_path: &'static str,
        asset_type_id: TypeId,
    ) -> Result<(Arc<dyn ErasedAssetLoader>, Vec<u8>), Error> {
        //TODO:macroquad
        let reader = macroquad::prelude::load_file(asset_path).await
            .map_err(|_| Error::new(ErrorKind::IO, format!("Failed to load file from path {}", asset_path)))?;

        let loader = {
            self.read_loaders()
                .find(Some(asset_type_id), None, Some(asset_path))
                .unwrap()
        };

        Ok((loader, reader))
    }

    async fn send_message(&self, message: InternalAssetEvent) {
        self.data.sender.send(message).await.unwrap();
    }
}

pub fn handle_internal_asset_events(mut world: Mut<World>) {
    world.resource_scope::<AssetServer>(|world, asset_server| {
        let mut infos = asset_server.write_infos();
        while let Ok(message) = asset_server.data.reciever.try_recv() {
            match message {
                InternalAssetEvent::Loaded {
                    index,
                    loaded_asset,
                } => {
                    println!("Loaded {}", loaded_asset.asset_type_name());
                    infos.process_asset_load(index, loaded_asset, world);
                }
                InternalAssetEvent::Failed { index: _, path, error } => {
                    println!("Failed to load asset from {}. Reason: {}", path, error)
                }
            }
        }

        infos.pending_tasks.retain(|_, task| !task.is_done());
    });
}

#[derive(Clone, Debug)]
pub enum LoadState {
    /// The asset has not started loading yet
    NotLoaded,

    /// The asset is in the process of loading.
    Loading,

    /// The asset has been loaded and has been added to the [`World`]
    Loaded,

    /// The asset failed to load.
    Failed,
}

pub struct AssetPlugin {
    pub file_path: &'static str,
}

impl Default for AssetPlugin {
    fn default() -> Self {
        Self {
            file_path: Self::DEFAULT_FILE_PATH,
        }
    }
}

impl AssetPlugin {
    const DEFAULT_FILE_PATH: &'static str = "res";
}

impl Plugin for AssetPlugin {
    fn build(self, app: &mut crate::app::App) {
        app
            .insert_resource(AssetServer::new(self.file_path))
            .add_systems(ScheduleLabel::PreUpdate, handle_internal_asset_events);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        asset::{
            AssetServer,
            asset::{Asset, Assets},
            handle::Handle,
            handle_internal_asset_events,
            loader::AssetLoader,
        },
        ecs::{change_detection::params::Mut, worlds::World},
        error::error::{Error, ErrorKind},
    };
    use std::any::TypeId;

    pub struct A(pub String);

    impl Asset for A {}

    pub struct ALoader;

    impl AssetLoader for ALoader {
        type Asset = A;
        type Error = Error;

        fn load(
            &self,
            reader: &mut Vec<u8>,
            _load_context: &mut super::loader::LoadContext,
        ) -> impl Future<Output = Result<Self::Asset, Self::Error>> + Send {
            async move {
                String::from_utf8(reader.drain(..).collect())
                    .map(|v| A(v))
                    .map_err(|_| Error::new(ErrorKind::Asset, "failed to load asset A"))
            }
        }
    }
}
