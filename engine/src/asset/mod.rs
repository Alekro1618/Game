use std::{any::TypeId, sync::{Arc, PoisonError, RwLock, RwLockWriteGuard}};

use macroquad::miniquad::fs::load_file;

use crate::{asset::{asset::{Asset, Assets}, handle::{Handle, StrongHandle}, info::AssetInfos, loader::{AssetLoader, AssetLoaders}}, ecs::{components::{Component, Mutable, StorageType}, resource::Resource, worlds::World}, error::error::Error};


mod io;
mod info;
mod loader;
pub mod asset;
pub mod handle;


#[derive(Clone)]
pub struct AssetServer {
    pub(crate) data: Arc<AssetServerData>,
}

impl Component for AssetServer {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Mutable;
}

impl Resource for AssetServer {}

pub(crate) struct AssetServerData {
    pub(crate) infos: RwLock<AssetInfos>,
    pub(crate) loaders: Arc<RwLock<AssetLoaders>>,
}

/// The "asset mode" the server is currently in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AssetServerMode {
    /// This server loads unprocessed assets.
    Unprocessed,
    /// This server loads processed assets.
    Processed,
}

impl AssetServer {
    pub fn new() -> Self {
        Self {
            data: Arc::new(AssetServerData {
                infos: RwLock::new(AssetInfos::new()),
                loaders: Arc::new(RwLock::new(AssetLoaders::default()))
            })
        }
    }

     pub(crate) fn write_infos(&self) -> RwLockWriteGuard<'_, AssetInfos> {
        self.data
            .infos
            .write()
            .unwrap_or_else(PoisonError::into_inner)
    }


    fn write_loaders(&self) -> RwLockWriteGuard<'_, AssetLoaders> {
        self.data
            .loaders
            .write()
            .unwrap_or_else(PoisonError::into_inner)
    }



    pub fn register_loader<L: AssetLoader>(&self, loader: L) {
        self.write_loaders();//.push(loader);
    }

    pub fn register_asset<A: Asset>(&self, assets: &Assets<A>) {
    
    }

   
    pub fn load<A: Asset>(&self, path: impl Into<&'static str>) -> Handle<A> {
        let path = path.into();

        let mut infos = self.write_infos();
        let (handle, should_load) = infos.get_or_create_path_handle::<A>(
            path.clone(),
        );

        if should_load {
            self.spawn_load_task(handle.clone().into(), path, infos);
        }

        handle
    }
    

    pub(crate) fn spawn_load_task<'a>(
        &self,
        handle: StrongHandle,
        path: &'a str,
        mut infos: RwLockWriteGuard<AssetInfos>,
    ) {
        let server = self.clone();
    
        let task = load_file(path,  move |result| {
            //server.load_internal(Some(handle), path, false).await;
        });

        // let task = IoTaskPool::get().spawn(async move {
        //     if let Err(err) = server
        //         .load_internal(Some(owned_handle), path, false, None)
        //         .await
        //     {
        //         println!("{}", err);
        //     }
        // });
    }
}

#[derive( Clone, Debug)]
pub enum LoadState {
    /// The asset has not started loading yet
    NotLoaded,

    /// The asset is in the process of loading.
    Loading,

    /// The asset has been loaded and has been added to the [`World`]
    Loaded,

    /// The asset failed to load.
    Failed
}