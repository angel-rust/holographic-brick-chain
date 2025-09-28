use brick_core::Ledger;
use brick_ids::account_id_from_name;

#[test]
fn mint_and_transfer() {
    let mut l = Ledger::new();
    let _ = l.mint("alice", 1_000, "boot");
    assert_eq!(l.balance_of(account_id_from_name("alice")), 1000);
    assert!(l.transfer("alice","bob",250,"pay").is_ok());
    assert_eq!(l.balance_of(account_id_from_name("alice")), 750);
    assert_eq!(l.balance_of(account_id_from_name("bob")), 250);
}
