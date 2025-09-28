use brick_core::Ledger;
use brick_chain::{balance_proof, verify_balance};
use brick_ids::account_id_from_name;

#[test]
fn balance_proof_roundtrip() {
    let mut l = Ledger::new();
    let _ = l.mint("alice", 1000, "boot");
    let _ = l.transfer("alice","bob",250,"pay");
    let acct = account_id_from_name("alice");
    let (bal, proof, idx, root) = balance_proof(&l.balances, acct).expect("proof");
    assert!(verify_balance(acct, bal, idx, root, &proof));
}
