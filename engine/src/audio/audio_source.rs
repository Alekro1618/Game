use std::sync::Arc;

use crate::asset::{asset::Asset, loader::{AssetLoader, LoadContext}};



#[derive(Debug, Clone)]
pub struct AudioSource {
    pub bytes: Arc<[u8]>,
}

impl Asset for AudioSource {}

impl AsRef<[u8]> for AudioSource {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}


#[derive(Default)]
pub struct AudioLoader;

impl AssetLoader for AudioLoader {
    type Asset = AudioSource;
    type Error = std::io::Error;
    
    fn load(
        &self,
        reader: &mut Vec<u8>,
        load_context: &mut LoadContext,
    ) -> impl Future<Output = Result<Self::Asset, Self::Error>> + Send {
        async move {
            Ok(AudioSource {
                bytes: reader.drain(..).as_slice().into(),
            })
        }
    }

    
}
