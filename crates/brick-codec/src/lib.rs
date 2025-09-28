fn put_u32_le(buf: &mut Vec<u8>, n: u32) { buf.extend_from_slice(&n.to_le_bytes()); }
fn put_u64_le(buf: &mut Vec<u8>, n: u64) { buf.extend_from_slice(&n.to_le_bytes()); }
fn put_u128_le(buf: &mut Vec<u8>, n: u128) { buf.extend_from_slice(&n.to_le_bytes()); }
fn put_bytes(buf: &mut Vec<u8>, b: &[u8]) { put_u32_le(buf, b.len() as u32); buf.extend_from_slice(b); }
fn put_str(buf: &mut Vec<u8>, s: &str) { put_bytes(buf, s.as_bytes()); }

pub enum TxKindTag { Mint = 1, Transfer = 2 }

pub fn encode_tx_v3(
    kind_tag: u8, id: u64, from_id: Option<u64>, to_id: u64,
    amount: u128, memo: &str, writes: &[u64]
) -> Vec<u8> {
    let mut out = Vec::with_capacity(128);
    out.extend_from_slice(b"BRICK/TXv3");
    put_u64_le(&mut out, id);
    out.push(kind_tag);
    match from_id { Some(f) => { out.push(1); put_u64_le(&mut out, f); }, None => out.push(0) }
    put_u64_le(&mut out, to_id);
    put_u128_le(&mut out, amount);
    put_str(&mut out, memo);
    put_u32_le(&mut out, writes.len() as u32);
    for &w in writes { put_u64_le(&mut out, w); }
    out
}
