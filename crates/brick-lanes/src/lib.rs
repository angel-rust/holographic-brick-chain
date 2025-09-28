use brick_core::Transaction;
use std::collections::HashSet;

/// Deterministic greedy packing by write-set disjointness.
pub fn pack_lanes(txs: &[Transaction]) -> Vec<Vec<Transaction>> {
    let mut v = txs.to_vec();
    v.sort_by_key(|t| t.id);
    let mut lanes: Vec<(HashSet<u64>, Vec<Transaction>)> = Vec::new();

    'outer: for tx in v {
        let w: HashSet<u64> = tx.writes.iter().map(|a| a.0).collect();
        for (keys, lane) in lanes.iter_mut() {
            if keys.is_disjoint(&w) {
                keys.extend(w.iter().cloned());
                lane.push(tx.clone());
                continue 'outer;
            }
        }
        lanes.push((w, vec![tx]));
    }
    lanes.into_iter().map(|(_, lane)| lane).collect()
}
