use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct EntityId {
    id: u32,
    #[serde(skip_serializing)]
    gnr: u32,
}

impl Default for EntityId {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Into<u64> for EntityId {
    fn into(self) -> u64 {
        ((self.id as u64) << 32) | self.gnr as u64
    }
}

impl From<u64> for EntityId {
    fn from(num: u64) -> Self {
        let id = (num >> 32) as u32;
        let gnr = (num & 0xFFFF_FFFF) as u32;
        Self { id, gnr }
    }
}

impl EntityId {
    pub const PLACEHOLDER: Self = EntityId { id: 0, gnr: 0 };

    pub fn new(id: u32, gnr: u32) -> Self {
        Self { id, gnr }
    }

    #[inline(always)]
    pub fn to_bits(self) -> u64 {
        self.into()
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn gnr(&self) -> u32 {
        self.gnr
    }
}

impl Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.id.to_string().as_str())
    }
}
