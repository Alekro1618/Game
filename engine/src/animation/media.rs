use serde::Deserialize;

use crate::{animation::Animatable, asset::{asset::Asset, loader::LoadContext}};

pub trait AnimatableMedia<A: Animatable> : Asset + 'static {
    type Deserializer : for <'de> Deserialize<'de>;

    fn load(deserializer : Self::Deserializer, context : &mut LoadContext) -> Self;
}

impl<A: Animatable> AnimatableMedia<A> for () {
    type Deserializer = ();

    fn load(_deserializer : Self::Deserializer, _context : &mut LoadContext) -> Self {
        ()
    }
}

impl<A, B, M1, M2> AnimatableMedia<(A, B)> for (M1, M2)
where
    A : Animatable, 
    B : Animatable,
    M1 : AnimatableMedia<A>,
    M2 : AnimatableMedia<B> 
{
    type Deserializer = (M1::Deserializer, M2::Deserializer);

    fn load(deserializer : Self::Deserializer, context : &mut LoadContext) -> Self {
        (
            M1::load(deserializer.0, context),
            M2::load(deserializer.1, context)
        )
    }
}
