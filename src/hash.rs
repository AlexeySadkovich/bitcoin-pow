use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash};

pub fn serialize_hash<T: Hash>(t: &T) {
    let mut s = DefaultHasher::new();
}
