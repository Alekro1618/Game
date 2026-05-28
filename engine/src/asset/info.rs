use std::{any::TypeId, collections::{HashMap, hash_map::Entry}, rc::Rc, sync::{Arc, Weak}};


use crate::{asset::{LoadState, asset::{Asset, AssetIndex}, handle::{Handle, StrongHandle}, loader::AssetLoader}, error::error::{Error, ErrorKind}};


#[derive(Debug)]
pub(crate) struct AssetInfo {
    weak_handle: Weak<StrongHandle>,
    pub(crate) path: Option<&'static str>,
    pub(crate) load_state: LoadState,
}

impl AssetInfo {
    fn new(
        weak_handle : Weak<StrongHandle>, 
        path: Option<&'static str>,
    ) -> Self {
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
}



impl AssetInfos {
    pub(crate) fn new() -> Self {
        Self { 
            path_to_index: HashMap::new(),
            infos: HashMap::new()
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
        path: &'static str
    ) -> (Handle<A>, bool) {
        let result = self.get_or_create_path_handle_internal(
            path,
            TypeId::of::<A>(),
        );

        let (handle, should_load) = result.unwrap();
    
        (handle.into(), should_load)
    }

    pub(crate) fn get_or_create_path_handle_internal<'a>(
        &mut self,
        path: &'static str,
        type_id: TypeId,
    ) -> Result<(Arc<StrongHandle>, bool), Error> {
        let handles = self.path_to_index.entry(path).or_default();

        match handles.entry(type_id) {
            Entry::Occupied(entry) => {
                let index = *entry.get();
                let info = self
                    .infos
                    .get_mut(&index)
                    .unwrap();
                
                if let Some(strong_handle) = info.weak_handle.upgrade() {
                    Ok((strong_handle, false))
                } else {
                    Err(Error::new(ErrorKind::Asset, format!("Handle for the asset {} was dropped", path)))
                }
            }
            Entry::Vacant(entry) => {
                let should_load = true;
                let handle = Self::create_handle_internal(
                    &mut self.infos,
                    type_id,
                    Some(path),
                    should_load,
                )?;
                entry.insert(handle.index());
                Ok((handle, true))
            }
        }
    }

    fn create_handle_internal<'a>(
        infos: &mut HashMap<AssetIndex, AssetInfo>,
        type_id: TypeId,
        path: Option<&'a str>,
        loading: bool,
    ) -> Result<Arc<StrongHandle>, Error> {
        // let handle = provider.reserve_handle_internal(true, path.clone());
        // let mut info = AssetInfo::new(Arc::downgrade(&handle), path);
        // if loading {
        //     info.load_state = LoadState::Loading;
        // }
        // infos.insert(handle.index(), info);

        // Ok(handle)
        todo!("")
    }
}
