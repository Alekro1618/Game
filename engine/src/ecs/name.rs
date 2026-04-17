use std::{borrow::Cow, hash::{Hash, Hasher}};

use crate::ecs::components::{Component, Mutable, StorageType};

// pub struct Name(Cow<'static, str>);

// impl Component for Name {
//     const STORAGE_TYPE: super::components::StorageType = StorageType::Table;

//     type Mutability = Mutable;
// }

// impl Default for Name {
//     fn default() -> Self {
//         Self("".into())
//     }
// }

// impl Name {
//     #[inline]
//     pub fn from(name : impl Into<Cow<'static, str>>) -> Self {
//         Self(name.into())
//     }

//     pub fn as_str(&self) -> &str {
//         &self.0
//     }
// }


// impl core::fmt::Display for Name {
//     #[inline(always)]
//     fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
//         core::fmt::Display::fmt(&self.0, f)
//     }
// }

// impl core::fmt::Debug for Name {
//     #[inline(always)]
//     fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
//         core::fmt::Debug::fmt(&self.0, f)
//     }
// }

// impl From<&Name> for String {
//     #[inline(always)]
//     fn from(val: &Name) -> String {
//         val.as_str().to_owned()
//     }
// }

// impl From<Name> for String {
//     #[inline(always)]
//     fn from(val: Name) -> String {
//         val.0.into_owned()
//     }
// }

// impl Hash for Name {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.0.hash(state);
//     }
// }

// impl PartialEq for Name {
//     fn eq(&self, other: &Self) -> bool {
//         self.0.eq(&other.0)
//     }
// }

// impl Eq for Name {}

pub struct Name(String);

impl Component for Name {
    const STORAGE_TYPE: super::components::StorageType = StorageType::Table;

    type Mutability = Mutable;
}

impl Default for Name {
    fn default() -> Self {
        Self("".into())
    }
}

impl Name {
    #[inline]
    pub fn from(name : impl ToString) -> Self {
        Self(name.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}


impl core::fmt::Display for Name {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.0, f)
    }
}

impl core::fmt::Debug for Name {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.0, f)
    }
}

impl From<&Name> for String {
    #[inline(always)]
    fn from(val: &Name) -> String {
        val.as_str().to_owned()
    }
}

impl From<Name> for String {
    #[inline(always)]
    fn from(val: Name) -> String {
        val.0
    }
}

impl Hash for Name {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for Name {}