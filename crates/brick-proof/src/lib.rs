use blake3::Hasher;
use brick_hash::Hash;

#[derive(Clone, Debug)]
pub struct MerkleTree { pub levels: Vec<Vec<Hash>> }

#[derive(Clone, Debug)]
pub struct Proof { pub siblings: Vec<Hash> }

fn hash_pair(a: &Hash, b: &Hash) -> Hash {
    let mut h = Hasher::new();
    h.update(a.as_bytes());
    h.update(b.as_bytes());
    h.finalize()
}

pub fn build(leaves: &[Hash]) -> MerkleTree {
    let mut levels = Vec::new();
    if leaves.is_empty() { levels.push(vec![Hasher::new().finalize()]); return MerkleTree { levels }; }
    levels.push(leaves.to_vec());
    while levels.last().unwrap().len() > 1 {
        let prev = levels.last().unwrap();
        let mut next = Vec::with_capacity((prev.len() + 1) / 2);
        for chunk in prev.chunks(2) {
            let h = if chunk.len() == 2 { hash_pair(&chunk[0], &chunk[1]) } else { hash_pair(&chunk[0], &chunk[0]) };
            next.push(h);
        }
        levels.push(next);
    }
    MerkleTree { levels }
}

pub fn root(tree: &MerkleTree) -> Hash { tree.levels.last().unwrap()[0] }

pub fn prove(tree: &MerkleTree, mut index: usize) -> Option<Proof> {
    if tree.levels.is_empty() || tree.levels[0].is_empty() { return None; }
    if index >= tree.levels[0].len() { return None; }
    let mut siblings = Vec::new();
    for lvl in &tree.levels[..tree.levels.len()-1] {
        let is_right = index & 1 == 1;
        let sib_idx = if is_right { index - 1 } else { index + 1 };
        let sib = if sib_idx < lvl.len() { lvl[sib_idx] } else { lvl[index] };
        siblings.push(sib);
        index >>= 1;
    }
    Some(Proof { siblings })
}

pub fn verify(root: Hash, mut leaf: Hash, proof: &Proof, mut index: usize) -> bool {
    for sib in &proof.siblings {
        let is_right = index & 1 == 1;
        leaf = if is_right { hash_pair(sib, &leaf) } else { hash_pair(&leaf, sib) };
        index >>= 1;
    }
    leaf == root
}
