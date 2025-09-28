use std::collections::HashMap;
use brick_ids::{AccountId, account_id_from_name};
use smallvec::{SmallVec, smallvec};

#[derive(Clone, Debug)]
pub enum TxKind { Mint, Transfer }

/// Transaction touches a small set of accounts (writes).
#[derive(Clone, Debug)]
pub struct Transaction {
    pub id: u64,
    pub kind: TxKind,
    pub from: Option<AccountId>,
    pub to: AccountId,
    pub amount: u128,
    pub memo: String,                         // truncated to 128 bytes at API edge
    pub writes: SmallVec<[AccountId; 2]>,     // sorted + deduped
}

#[derive(Default)]
pub struct Ledger {
    next_id: u64,
    pub txs: Vec<Transaction>,
    pub balances: HashMap<AccountId, u128>,
}

impl Ledger {
    pub fn new() -> Self { Self { next_id: 1, txs: Vec::new(), balances: HashMap::new() } }

    fn bump_id(&mut self) -> u64 { let id = self.next_id; self.next_id += 1; id }

    pub fn total_supply(&self) -> u128 {
        self.balances.values().copied().sum()
    }

    /// Truncate memo to 128 bytes (not chars) for DOS safety.
    fn cap_memo(m: impl Into<String>) -> String {
        let s = m.into();
        let b = s.as_bytes();
        if b.len() <= 128 { s } else { String::from_utf8_lossy(&b[..128]).to_string() }
    }

    /// Sort by AccountId and dedup equal neighbors.
    fn sort_dedup_writes(v: &mut SmallVec<[AccountId; 2]>) {
        v.sort_unstable_by_key(|a| a.0);
        v.dedup_by(|a, b| a.0 == b.0); // <-- fix: call on SmallVec, not slice
    }

    // ---------- Name-based convenience (UI/CLI) ----------
    pub fn mint(&mut self, to_name: &str, amount: u128, memo: impl Into<String>) -> &Transaction {
        let to = account_id_from_name(to_name);
        self.mint_id(to, amount, memo)
    }

    pub fn transfer(&mut self, from_name: &str, to_name: &str, amount: u128, memo: impl Into<String>)
        -> Result<&Transaction, &'static str>
    {
        let from = account_id_from_name(from_name);
        let to = account_id_from_name(to_name);
        self.transfer_id(from, to, amount, memo)
    }

    // ---------- Id-based hot-path ----------
    pub fn mint_id(&mut self, to: AccountId, amount: u128, memo: impl Into<String>) -> &Transaction {
        let id = self.bump_id();
        let memo = Self::cap_memo(memo);

        // construct writes with inline cap 2 (even if 1 elem)
        let mut writes: SmallVec<[AccountId; 2]> = smallvec![to];
        Self::sort_dedup_writes(&mut writes);

        let tx = Transaction {
            id, kind: TxKind::Mint, from: None, to, amount, memo, writes,
        };
        self.apply(&tx).expect("mint cannot fail");
        self.txs.push(tx);
        self.txs.last().unwrap()
    }

    pub fn transfer_id(&mut self, from: AccountId, to: AccountId, amount: u128, memo: impl Into<String>)
        -> Result<&Transaction, &'static str>
    {
        if from == to { return Err("self-transfer not allowed"); }
        if self.balance_of(from) < amount { return Err("insufficient funds"); }
        let id = self.bump_id();
        let memo = Self::cap_memo(memo);

        let mut writes: SmallVec<[AccountId; 2]> = smallvec![from, to];
        Self::sort_dedup_writes(&mut writes);

        let tx = Transaction {
            id, kind: TxKind::Transfer, from: Some(from), to, amount, memo, writes,
        };
        self.apply(&tx)?;
        self.txs.push(tx);
        Ok(self.txs.last().unwrap())
    }

    pub fn balance_of(&self, who: AccountId) -> u128 { *self.balances.get(&who).unwrap_or(&0) }

    /// Apply (infallible for mint; may fail for transfer).
    fn apply(&mut self, tx: &Transaction) -> Result<(), &'static str> {
        match tx.kind {
            TxKind::Mint => {
                let e = self.balances.entry(tx.to).or_default();
                *e = e.saturating_add(tx.amount);
                Ok(())
            }
            TxKind::Transfer => {
                let from = tx.from.ok_or("missing from")?;
                let from_bal = self.balances.entry(from).or_default();
                if *from_bal < tx.amount { return Err("insufficient funds"); }
                *from_bal -= tx.amount;
                let to_bal = self.balances.entry(tx.to).or_default();
                *to_bal = *to_bal + tx.amount;
                Ok(())
            }
        }
    }

    /// For executors that apply a cloned tx; do not swallow errors.
    pub fn apply_tx_clone(&mut self, tx: &Transaction) -> Result<(), &'static str> {
        self.apply(tx)
    }
}
