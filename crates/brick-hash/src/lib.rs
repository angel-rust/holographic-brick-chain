pub use blake3::Hash;

/// Hash multiple byte slices in order.
pub fn hash_parts(parts: &[&[u8]]) -> Hash {
    let mut h = blake3::Hasher::new();
    for p in parts { h.update(p); }
    h.finalize()
}

/// Short hex (first 16 chars) for logs.
pub fn hex16(h: &Hash) -> String {
    let b = h.as_bytes();
    let mut s = String::with_capacity(16);
    for i in 0..8 { s.push_str(&format!("{:02x}", b[i])); }
    s
}
