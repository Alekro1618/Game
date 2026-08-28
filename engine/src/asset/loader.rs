use std::{
    any::{Any, TypeId},
    pin::Pin,
};

use downcast_rs::{Downcast, impl_downcast};

use crate::{
    asset::{
        AssetServer,
        asset::{Asset, AssetIndex, Assets},
        handle::StrongHandle,
    },
    ecs::worlds::World,
    error::error::Error,
};

pub trait AssetLoader: Send + Sync + 'static {
    type Asset: Asset + Send;
    type Error: Into<Error> + Send;

    fn load(
        &self,
        reader: &mut Vec<u8>,
        load_context: &mut LoadContext,
    ) -> impl Future<Output = Result<Self::Asset, Self::Error>> + Send;

    fn type_name() -> &'static str {
        std::any::type_name::<Self::Asset>()
    }
    /// Returns a list of extensions supported by this [`AssetLoader`], without the preceding dot.
    /// Note that users of this [`AssetLoader`] may choose to load files with a non-matching extension.
    fn extensions(&self) -> &[&str] {
        &[]
    }
}

/// Provides type-erased access to an [`AssetLoader`].
pub trait ErasedAssetLoader: Send + Sync + 'static {
    /// Asynchronously loads the asset(s) from the bytes provided by [`Reader`].
    fn load<'a>(
        &'a self,
        reader: &'a mut Vec<u8>,
        load_context: LoadContext<'a>,
    ) -> Pin<Box<dyn Future<Output = Result<ErasedLoadedAsset, Error>> + Send + 'a>>;

    fn extensions(&self) -> &[&str];
    fn type_id(&self) -> TypeId;
    fn asset_type_name(&self) -> &'static str;
    fn asset_type_id(&self) -> TypeId;
}

impl<L> ErasedAssetLoader for L
where
    L: AssetLoader + Send + Sync,
{
    /// Processes the asset in an asynchronous closure.
    fn load<'a>(
        &'a self,
        reader: &'a mut Vec<u8>,
        mut load_context: LoadContext<'a>,
    ) -> Pin<Box<dyn Future<Output = Result<ErasedLoadedAsset, Error>> + Send + 'a>> {
        Box::pin(async move {
            let asset = <L as AssetLoader>::load(self, reader, &mut load_context)
                .await
                .map_err(Into::into)?;
            Ok(load_context.finish(asset).into())
        })
    }

    fn extensions(&self) -> &[&str] {
        <L as AssetLoader>::extensions(self)
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<L>()
    }

    fn asset_type_name(&self) -> &'static str {
        core::any::type_name::<L::Asset>()
    }

    fn asset_type_id(&self) -> TypeId {
        TypeId::of::<L::Asset>()
    }
}

pub(crate) struct LabeledAsset {
    pub(crate) asset: ErasedLoadedAsset,
    pub(crate) handle: StrongHandle,
}

pub struct LoadedAsset<A: Asset> {
    pub(crate) value: A,
}

impl<A: Asset> LoadedAsset<A> {
    /// Create a new loaded asset. This will use [`VisitAssetDependencies`](crate::VisitAssetDependencies) to populate `dependencies`.
    pub fn new(value: A) -> Self {
        LoadedAsset { value }
    }

    /// Cast (and take ownership) of the [`Asset`] value of the given type.
    pub fn take(self) -> A {
        self.value
    }

    /// Retrieves a reference to the internal [`Asset`] type.
    pub fn get(&self) -> &A {
        &self.value
    }
}

impl<A: Asset> From<A> for LoadedAsset<A> {
    fn from(asset: A) -> Self {
        LoadedAsset::new(asset)
    }
}

/// A "type erased / boxed" counterpart to [`LoadedAsset`]. This is used in places where the loaded type is not statically known.
pub struct ErasedLoadedAsset {
    pub(crate) value: Box<dyn AssetContainer>,
}

impl<A: Asset> From<LoadedAsset<A>> for ErasedLoadedAsset {
    fn from(asset: LoadedAsset<A>) -> Self {
        ErasedLoadedAsset {
            value: Box::new(asset.value),
        }
    }
}

impl ErasedLoadedAsset {
    /// Cast (and take ownership) of the [`Asset`] value of the given type. This will return [`Some`] if
    /// the stored type matches `A` and [`None`] if it does not.
    pub fn take<A: Asset>(self) -> Option<A> {
        self.value.downcast::<A>().map(|a| *a).ok()
    }

    /// Retrieves a reference to the internal [`Asset`] type, if it matches the type `A`. Otherwise returns [`None`].
    pub fn get<A: Asset>(&self) -> Option<&A> {
        self.value.downcast_ref::<A>()
    }

    /// Retrieves the [`TypeId`] of the stored [`Asset`] type.
    pub fn asset_type_id(&self) -> TypeId {
        (*self.value).type_id()
    }

    /// Retrieves the `type_name` of the stored [`Asset`] type.
    pub fn asset_type_name(&self) -> &'static str {
        self.value.asset_type_name()
    }

    pub fn downcast<A: Asset>(mut self) -> Result<LoadedAsset<A>, ErasedLoadedAsset> {
        match self.value.downcast::<A>() {
            Ok(value) => Ok(LoadedAsset { value: *value }),
            Err(value) => {
                self.value = value;
                Err(self)
            }
        }
    }
}

/// A type erased container for an [`Asset`] value that is capable of inserting the [`Asset`] into a [`World`]'s [`Assets`] collection.
pub(crate) trait AssetContainer: Downcast + Any + Send + Sync + 'static {
    fn insert(self: Box<Self>, id: AssetIndex, world: &mut World);
    fn asset_type_name(&self) -> &'static str;
}

impl_downcast!(AssetContainer);

impl<A: Asset> AssetContainer for A {
    fn insert(self: Box<Self>, index: AssetIndex, world: &mut World) {
        // We only ever call this if we know the asset is still alive, so it is fine to unwrap here.
        world
            .resource_mut::<Assets<A>>()
            .insert(index, *self)
            .expect("the AssetIndex is still valid");
    }

    fn asset_type_name(&self) -> &'static str {
        core::any::type_name::<A>()
    }
}

/// A context that provides access to assets in [`AssetLoader`]s, tracks dependencies, and collects asset load state.
///
/// Any asset state accessed by [`LoadContext`] will be tracked and stored for use in dependency events and asset preprocessing.
pub struct LoadContext<'a> {
    pub(crate) asset_server: &'a AssetServer,
    asset_path: &'static str,
}

impl<'a> LoadContext<'a> {
    /// Creates a new [`LoadContext`] instance.
    pub(crate) fn new(asset_server: &'a AssetServer, asset_path: &'static str) -> Self {
        Self {
            asset_server,
            asset_path,
        }
    }

    /// "Finishes" this context by populating the final [`Asset`] value.
    pub fn finish<A: Asset>(self, value: A) -> LoadedAsset<A> {
        LoadedAsset { value }
    }

    /// Gets the source asset path for this load context.
    pub fn path(&self) -> &'static str {
        &self.asset_path
    }
}
