use std::marker::PhantomData;

use engine_macros::Message;

use crate::asset::asset::{Asset, AssetIndex};


#[derive(Message)]
pub enum AssetEvent<A: Asset> {
    /// Emitted whenever an [`Asset`] is added.
    Added { id: AssetIndex },
    /// Emitted whenever an [`Asset`] value is modified.
    Modified { id: AssetIndex },
    /// Emitted whenever an [`Asset`] is removed.
    Removed { id: AssetIndex },
    /// Emitted when the last [`Handle::Strong`](`super::Handle::Strong`) of an [`Asset`] is dropped.
    Unused { id: AssetIndex },

    _Phantom {phantom : PhantomData<A>}
}
