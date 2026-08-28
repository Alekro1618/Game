use core::hash::{BuildHasher, Hasher};

pub struct EntityHash;

impl BuildHasher for EntityHash {
    type Hasher = EntityHasher;

    fn build_hasher(&self) -> Self::Hasher {
        Self::Hasher::default()
    }
}

#[derive(Debug, Default)]
pub struct EntityHasher {
    hash: u64,
}

impl Hasher for EntityHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }

    fn write(&mut self, _bytes: &[u8]) {
        panic!("EntityHasher can only hash u64 fields.");
    }

    #[inline]
    fn write_u64(&mut self, bits: u64) {
        const UPPER_PHI: u64 = 0x9e37_79b9_0000_0001;
        self.hash = bits.wrapping_mul(UPPER_PHI);
    }
}
