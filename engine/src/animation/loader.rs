use std::{collections::HashMap, marker::PhantomData};

use serde::{Deserialize, Serialize};

use crate::{
    animation::{Animatable, manager::{AnimationManager, AnimationManagerErased, Transition}, media::AnimatableMedia, player::Player}, asset::{AssetPath, loader::AssetLoader}, error::error::Error,
};

pub struct AnimatableMediaLoader<M: AnimatableMedia<A>, A: Animatable>(PhantomData<(M, A)>);

impl<M: AnimatableMedia<A>, A: Animatable> Default for AnimatableMediaLoader<M, A> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

pub enum AnimatableMediaLoadingError {
    Unknown,
}

impl Into<Error> for AnimatableMediaLoadingError {
    fn into(self) -> Error {
        todo!("")
    }
}

impl<A: Animatable, M: AnimatableMedia<A>> AssetLoader
    for AnimatableMediaLoader<M, A>
{
    type Asset = M;
    type Error = AnimatableMediaLoadingError;

    fn load(
        &self,
        reader: &mut Vec<u8>,
        load_context: &mut crate::asset::loader::LoadContext,
    ) -> impl Future<Output = Result<Self::Asset, Self::Error>> + Send {
        async move {
            let media =
                postcard::from_bytes::<M::Deserializer>(reader)
                    .map_err(|_| AnimatableMediaLoadingError::Unknown)?;
            
            Ok(M::load(media, load_context))
        }
    }
}

pub struct AnimatableManagerMediaLoader<P: Player>(PhantomData<P >);


impl<P : Player> Default for AnimatableManagerMediaLoader<P> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<P: Player> AssetLoader
    for AnimatableManagerMediaLoader<P>
{
    type Asset = AnimationManager<P::State, P>;
    type Error = AnimatableMediaLoadingError;

    fn load(
        &self,
        reader: &mut Vec<u8>,
        load_context: &mut crate::asset::loader::LoadContext,
    ) -> impl Future<Output = Result<Self::Asset, Self::Error>> + Send {
        async move {
            let manager =
                postcard::from_bytes::<AnimationManagerErased>(reader)
                    .map_err(|_| AnimatableMediaLoadingError::Unknown)?;
            
            let recordings = manager
                .recordings
                .into_iter()
                .map(|(name, path)|{
                    (name, load_context.asset_server.load(path))
                })
                .collect();
            
            Ok(AnimationManager::new(manager.transitions, recordings))
        }
    }
}