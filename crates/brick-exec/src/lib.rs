use brick_core::{Ledger, Transaction};

/// Execute lanes. Default: sequential baseline.
pub fn execute_lanes(ledger: &mut Ledger, lanes: &[Vec<Transaction>]) -> Result<(), &'static str> {
    for lane in lanes {
        for tx in lane {
            ledger.apply_tx_clone(tx)?;
            ledger.txs.push(tx.clone());
        }
    }
    Ok(())
}
