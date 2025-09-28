use brick_hash::{Hash, hash_parts};

#[derive(Clone, Debug)]
pub struct ZoneCommit { pub zone: u32, pub height: u64, pub state_root: Hash, pub da_root: Hash }

pub fn super_root(commits: &[ZoneCommit]) -> Hash {
    let mut v = commits.to_vec();
    v.sort_by(|a,b| a.zone.cmp(&b.zone).then(a.height.cmp(&b.height)));
    let mut bytes = Vec::with_capacity(64 * v.len());
    for c in &v {
        bytes.extend_from_slice(&c.zone.to_le_bytes());
        bytes.extend_from_slice(&c.height.to_le_bytes());
        bytes.extend_from_slice(c.state_root.as_bytes());
        bytes.extend_from_slice(c.da_root.as_bytes());
    }
    hash_parts(&[b"BRICK/SUPER/v1", &bytes])
}
