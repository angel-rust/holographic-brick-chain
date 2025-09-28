use unicode_normalization::UnicodeNormalization;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct AccountId(pub u64);

fn normalize(name: &str) -> String {
    name.nfkc().collect::<String>().to_lowercase().trim().to_string()
}

pub fn account_id_from_name(name: &str) -> AccountId {
    let norm = normalize(name);
    let mut h = blake3::Hasher::new();
    h.update(b"BRICK/ACCTID/v1");
    h.update(norm.as_bytes());
    let bytes = h.finalize();
    let mut lane = [0u8; 8];
    lane.copy_from_slice(&bytes.as_bytes()[..8]);
    AccountId(u64::from_le_bytes(lane))
}
