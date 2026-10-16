use std::{
    any::TypeId,
    collections::{HashMap, hash_map::Entry},
    sync::{Arc, Weak},
};


use crate::{
    asset::{
        LoadState, asset::{Asset, AssetIndex}, handle::{AssetHandleProvider, ErasedHandle, Handle, StrongHandle}, loader::ErasedLoadedAsset,
    }, ecs::{components::info::TypeIdMap, worlds::World}, error::error::{Error, ErrorKind},
};




#[derive(Debug)]
pub(crate) struct AssetInfo {
    weak_handle: Weak<StrongHandle>,
    pub(crate) path: Option<&'static str>,
    pub(crate) load_state: LoadState,
}

impl AssetInfo {
    fn new(weak_handle: Weak<StrongHandle>, path: Option<&'static str>) -> Self {
        Self {
            weak_handle,
            path,
            load_state: LoadState::NotLoaded,
        }
    }
}

#[derive(Default)]
pub(crate) struct AssetInfos {
    path_to_index: HashMap<&'static str, HashMap<TypeId, AssetIndex>>,
    infos: HashMap<AssetIndex, AssetInfo>,
    //TODO:macroquad
    pub(crate) pending_tasks:
        HashMap<AssetIndex, macroquad::experimental::coroutines::Coroutine<()>>,
    pub(crate) handle_providers: TypeIdMap<AssetHandleProvider>,
}

impl AssetInfos {
    pub(crate) fn new() -> Self {
        Self {
            path_to_index: HashMap::new(),
            infos: HashMap::new(),
            pending_tasks: HashMap::new(),
            handle_providers: Default::default(),
        }
    }

    pub(crate) fn get(&self, index: AssetIndex) -> Option<&AssetInfo> {
        self.infos.get(&index)
    }

    pub(crate) fn contains_key(&self, index: AssetIndex) -> bool {
        self.infos.contains_key(&index)
    }

    pub(crate) fn get_mut(&mut self, index: AssetIndex) -> Option<&mut AssetInfo> {
        self.infos.get_mut(&index)
    }

    pub(crate) fn get_path_and_type_id_handle(
        &self,
        path: &'_ str,
        type_id: TypeId,
    ) -> Option<Arc<StrongHandle>> {
        let index = *self.path_to_index.get(path)?.get(&type_id)?;
        self.get_index_handle(index)
    }

    pub(crate) fn get_index_handle(&self, index: AssetIndex) -> Option<Arc<StrongHandle>> {
        let info = self.infos.get(&index)?;
        let strong_handle = info.weak_handle.upgrade()?;
        Some(strong_handle)
    }

    pub(crate) fn get_or_create_path_handle<A: Asset>(
        &mut self,
        path: &'static str,
        lazy : bool
    ) -> (Handle<A>, bool) {
        let result = self.get_or_create_path_handle_internal(path, Some(TypeId::of::<A>()), lazy);

        let (handle, should_load) = result.unwrap();

        let handle: Handle<A> = handle.into();

        if lazy {
            (handle.lazy(), false)
        } else {
            (handle, should_load)
        }        
    }

    pub(crate) fn get_or_create_path_handle_erased(
        &mut self,
        path: &'static str,
        type_id : TypeId,
        lazy : bool,
    ) -> (ErasedHandle, bool) {
        let result = self.get_or_create_path_handle_internal(path, Some(type_id), lazy);

        let (handle, should_load) = result.unwrap();

        let handle: ErasedHandle = handle.into();

        if lazy {
            (handle.lazy(), false)
        } else {
            (handle, should_load)
        }        
    }

    pub(crate) fn get_path_handle<A: Asset>(
        &mut self,
        path: &'static str,
    ) -> Option<Handle<A>> {
        self
            .get_path_handle_internal(path, Some(TypeId::of::<A>()))
            .ok()
            .map(|strong| strong.into())
    }

    pub(crate) fn get_or_create_path_handle_internal<'a>(
        &mut self,
        path: &'static str,
        type_id: Option<TypeId>,
        lazy : bool
    ) -> Result<(Arc<StrongHandle>, bool), Error> {
        let handles = self.path_to_index.entry(path).or_default();

        let type_id = type_id
            .or_else(|| {
                // If a TypeId is not provided, we may be able to infer it if only a single entry exists
                if handles.len() == 1 {
                    Some(*handles.keys().next().unwrap())
                } else {
                    None
                }
            })
            .expect(&format!("Couldn't infer type id for {}", path));

        match handles.entry(type_id) {
            Entry::Occupied(entry) => {
                let index = *entry.get();
                let info = self.infos.get_mut(&index).unwrap();

                if let Some(strong_handle) = info.weak_handle.upgrade() {
                    Ok((strong_handle, false))
                } else {
                    Err(Error::new(
                        ErrorKind::Asset,
                        format!("Handle for the asset {} was dropped", path),
                    ))
                }
            }
            Entry::Vacant(entry) => {
                let should_load = !lazy;
                let handle = Self::create_handle_internal(
                    &mut self.infos,
                    self.handle_providers.get_mut(&type_id).expect(&format!("Asset wasn't initialised for asset {}", path)),
                    type_id,
                    path,
                    should_load,
                )?;
                entry.insert(handle.index());
                Ok((handle, should_load))
            }
        }
    }

    pub(crate) fn get_path_handle_internal<'a>(
        &mut self,
        path: &'static str,
        type_id: Option<TypeId>
    ) -> Result<Arc<StrongHandle>, Error> {
        let handles = self.path_to_index.entry(path).or_default();

        let type_id = type_id
            .or_else(|| {
                // If a TypeId is not provided, we may be able to infer it if only a single entry exists
                if handles.len() == 1 {
                    Some(*handles.keys().next().unwrap())
                } else {
                    None
                }
            })
            .expect(&format!("Couldn't infer type id for {}", path));

        match handles.entry(type_id) {
            Entry::Occupied(entry) => {
                let index = *entry.get();
                let info = self.infos.get_mut(&index).unwrap();

                info.load_state = LoadState::Loading;

                if let Some(strong_handle) = info.weak_handle.upgrade() {
                    Ok(strong_handle)
                } else {
                    Err(Error::new(
                        ErrorKind::Asset,
                        format!("Handle for the asset {} was dropped", path),
                    ))
                }
            }
            Entry::Vacant(_entry) => {
                Err(Error::new(
                    ErrorKind::Asset, 
                    format!("Trying to get handle of not previosly prepared asset {}", path)
                ))
            }
        }
    }

    fn create_handle_internal(
        infos: &mut HashMap<AssetIndex, AssetInfo>,
        provider: &mut AssetHandleProvider,
        _type_id: TypeId,
        path: &'static str,
        should_load: bool,
    ) -> Result<Arc<StrongHandle>, Error> {
        let handle = provider.reserve_handle(path.into());
        let mut info = AssetInfo::new(Arc::downgrade(&handle), Some(path));
        if should_load {
            info.load_state = LoadState::Loading;
        }
        infos.insert(handle.index(), info);

        Ok(handle)
    }

    pub(crate) fn process_asset_load(
        &mut self,
        loaded_asset_index: AssetIndex,
        loaded_asset: ErasedLoadedAsset,
        world: &mut World,
    ) {
        if !self.infos.contains_key(&loaded_asset_index) {
            return;
        }

        loaded_asset.value.insert(loaded_asset_index, world);
    }
}
