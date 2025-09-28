use brick_hash::{Hash, hash_parts};
use ed25519_dalek::{Signer, Verifier, SigningKey, VerifyingKey, Signature};

#[derive(Clone, Debug)]
pub struct AttestationHeader {
    pub node_id: u32,
    pub epoch: u64,
    pub zone: u32,
    pub height: u64,
    pub tx_root: Hash,
    pub state_root: Hash,
    pub da_root: Hash,
    pub super_root: Option<Hash>,
    pub prev: Option<Hash>,
}

#[derive(Clone, Debug)]
pub struct Attestation {
    pub header: AttestationHeader,
    pub hash: Hash,
    pub sig: Signature,
    pub pubkey: VerifyingKey,
}

fn encode_header_v1(h: &AttestationHeader) -> Vec<u8> {
    let mut out = Vec::with_capacity(256);
    out.extend_from_slice(b"BRICK/ATTEST/v1");
    out.extend_from_slice(&h.node_id.to_le_bytes());
    out.extend_from_slice(&h.epoch.to_le_bytes());
    out.extend_from_slice(&h.zone.to_le_bytes());
    out.extend_from_slice(&h.height.to_le_bytes());
    out.extend_from_slice(h.tx_root.as_bytes());
    out.extend_from_slice(h.state_root.as_bytes());
    out.extend_from_slice(h.da_root.as_bytes());
    match h.super_root { Some(s) => { out.push(1); out.extend_from_slice(s.as_bytes()); }, None => out.push(0) }
    match h.prev { Some(p) => { out.push(1); out.extend_from_slice(p.as_bytes()); }, None => out.push(0) }
    out
}

fn header_hash(h: &AttestationHeader) -> Hash {
    let bytes = encode_header_v1(h);
    hash_parts(&[&bytes])
}

pub fn issue(signing: &SigningKey, header: AttestationHeader) -> Attestation {
    let bytes = encode_header_v1(&header);
    let h = header_hash(&header);
    let sig = signing.sign(&bytes);
    let pubkey = signing.verifying_key();
    Attestation { header, hash: h, sig, pubkey }
}

pub fn verify(att: &Attestation, prev: Option<Hash>) -> bool {
    if att.header.prev != prev { return false; }
    let bytes = encode_header_v1(&att.header);
    att.pubkey.verify(&bytes, &att.sig).is_ok() && header_hash(&att.header) == att.hash
}
