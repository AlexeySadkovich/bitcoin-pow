use crate::hash::hash;
use std::hash::Hash;

#[derive(Hash)]
pub struct CBlockHeader {
    pub n_bits: u32,
    pub n_nonce: u32,
}

impl CBlockHeader {
    pub fn get_hash(&self) -> u64 {
        hash::serialize_hash(&self)
    }
}

