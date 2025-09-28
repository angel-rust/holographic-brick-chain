use std::collections::HashMap;
use brick_core::{Transaction, TxKind};
use brick_hash::Hash;
use brick_codec::{encode_tx_v3, TxKindTag};
use brick_ids::AccountId;
use brick_proof as m;

pub fn tx_root(txs: &[Transaction]) -> Hash {
    let mut v = txs.to_vec();
    v.sort_by_key(|t| t.id);
    let mut h = blake3::Hasher::new();
    for t in &v {
        let kind_tag = match t.kind { TxKind::Mint => TxKindTag::Mint as u8, TxKind::Transfer => TxKindTag::Transfer as u8 };
        let from_id = t.from.map(|a| a.0);
        let to_id = t.to.0;
        let writes: Vec<u64> = t.writes.iter().map(|a| a.0).collect();
        let bytes = encode_tx_v3(kind_tag, t.id, from_id, to_id, t.amount, &t.memo, &writes);
        h.update(&bytes);
    }
    h.finalize()
}

pub fn state_root_flat(balances: &HashMap<AccountId, u128>) -> Hash {
    let mut accounts: Vec<_> = balances.iter().collect();
    accounts.sort_by_key(|(acct, _)| *acct);
    let mut h = blake3::Hasher::new();
    for (acct, bal) in accounts {
        h.update(&acct.0.to_le_bytes());
        h.update(&bal.to_le_bytes());
    }
    h.finalize()
}

fn balance_leaf(acct: AccountId, bal: u128) -> Hash {
    let mut h = blake3::Hasher::new();
    h.update(b"BRICK/LEAF/v1");
    h.update(&acct.0.to_le_bytes());
    h.update(&bal.to_le_bytes());
    h.finalize()
}

pub fn state_merkle_leaves(balances: &HashMap<AccountId, u128>) -> (Vec<(AccountId, u128)>, Vec<Hash>) {
    let mut rows: Vec<_> = balances.iter().map(|(a,b)| (*a, *b)).collect();
    rows.sort_by_key(|(a, _)| *a);
    let leaves: Vec<Hash> = rows.iter().map(|(a,b)| balance_leaf(*a, *b)).collect();
    (rows, leaves)
}

pub fn state_merkle(balances: &HashMap<AccountId, u128>) -> (Hash, m::MerkleTree) {
    let (_rows, leaves) = state_merkle_leaves(balances);
    let tree = m::build(&leaves);
    (m::root(&tree), tree)
}

pub fn balance_proof(balances: &HashMap<AccountId, u128>, acct: AccountId) -> Option<(u128, m::Proof, usize, Hash)> {
    let (rows, leaves) = state_merkle_leaves(balances);
    let idx = rows.iter().position(|(a, _)| *a == acct)?;
    let bal = rows[idx].1;
    let tree = m::build(&leaves);
    let root = m::root(&tree);
    let proof = m::prove(&tree, idx)?;
    Some((bal, proof, idx, root))
}

pub fn verify_balance(acct: AccountId, bal: u128, idx: usize, root: Hash, proof: &m::Proof) -> bool {
    let leaf = balance_leaf(acct, bal);
    m::verify(root, leaf, proof, idx)
}
