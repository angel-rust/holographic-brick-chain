use brick_hash::{Hash, hash_parts};

#[derive(Clone, Debug)]
pub struct DaCommit { pub root: Hash }

#[derive(Clone, Debug)]
pub struct DaShare { pub index: u32, pub data: Vec<u8> }

/// Minimal DA commitment: tag + data hash.
/// (Erasure coding can be swapped in later without changing callers.)
pub fn encode(data: &[u8]) -> (DaCommit, Vec<DaShare>) {
    let root = hash_parts(&[b"BRICK/DAv1", data]);
    (DaCommit { root }, vec![DaShare { index: 0, data: data.to_vec() }])
}
