use brick_core::Ledger;
use brick_chain::{tx_root, state_merkle};
use brick_da as da;
use brick_super::{ZoneCommit, super_root as compute_super_root};
use brick_attest::{AttestationHeader, Attestation, issue as issue_att};
use ed25519_dalek::SigningKey;
use brick_hash::Hash;

#[derive(Clone, Debug)]
pub struct ZoneConfig { pub node_id: u32, pub zone_id: u32, pub batch_max: usize }

#[derive(Clone, Debug)]
pub struct Claim { pub commit: ZoneCommit, pub tx_count: usize }

#[derive(Clone, Debug)]
pub struct TickOutput { pub claim: Option<Claim>, pub super_root: Hash, pub attestation: Attestation }

pub struct HoloNode {
    pub cfg: ZoneConfig,
    pub ledger: Ledger,
    key: SigningKey,
    epoch: u64,
    batch_height: u64,
    sealed_idx: usize,
    prev_att_hash: Option<Hash>,
}

impl HoloNode {
    pub fn new(cfg: ZoneConfig, key: SigningKey) -> Self {
        Self { cfg, ledger: Ledger::new(), key, epoch: 0, batch_height: 0, sealed_idx: 0, prev_att_hash: None }
    }

    pub fn pending_len(&self) -> usize { self.ledger.txs.len().saturating_sub(self.sealed_idx) }

    pub fn tick(&mut self) -> TickOutput {
        let pending = &self.ledger.txs[self.sealed_idx..];
        let take = pending.len().min(self.cfg.batch_max);
        let batch = &pending[..take];

        let batch_tx_root = tx_root(batch);
        let (state_root, _tree) = state_merkle(&self.ledger.balances);

        let (da_commit, _shares) = da::encode(batch_tx_root.as_bytes());

        let claim = if !batch.is_empty() {
            Some(Claim {
                commit: ZoneCommit { zone: self.cfg.zone_id, height: self.batch_height, state_root, da_root: da_commit.root },
                tx_count: batch.len(),
            })
        } else { None };

        let super_root = match &claim {
            Some(c) => compute_super_root(&[c.commit.clone()]),
            None => compute_super_root(&[ZoneCommit { zone: self.cfg.zone_id, height: self.batch_height, state_root, da_root: da_commit.root }]),
        };

        let header = AttestationHeader {
            node_id: self.cfg.node_id,
            epoch: self.epoch,
            zone: self.cfg.zone_id,
            height: self.batch_height,
            tx_root: batch_tx_root,
            state_root,
            da_root: da_commit.root,
            super_root: Some(super_root),
            prev: self.prev_att_hash,
        };
        let att = issue_att(&self.key, header);
        self.prev_att_hash = Some(att.hash);

        if let Some(c) = &claim { self.sealed_idx += c.tx_count; self.batch_height += 1; }
        self.epoch += 1;

        TickOutput { claim, super_root, attestation: att }
    }
}
