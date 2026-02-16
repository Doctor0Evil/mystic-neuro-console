pub mod error;
pub mod accounts;
pub mod ledger;
pub mod token;

pub use crate::accounts::{Account, AccountId};
pub use crate::error::{NeuroError, NeuroResult};
pub use crate::ledger::{Ledger, TransferRequest};
pub use crate::token::{TokenMeta, TokenSupply};

/// Initialize logging for applications embedding Neurotoken.
pub fn init_logging() {
    let _ = env_logger::builder()
        .is_test(false)
        .try_init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_flow_mint_and_transfer() {
        init_logging();
        let mut ledger = Ledger::new();

        let alice = ledger.create_account("alice").expect("alice");
        let bob = ledger.create_account("bob").expect("bob");

        let meta = TokenMeta::new("NEURO", 6, "Neuro Token");
        ledger
            .mint(&alice, 1_000_000, meta.clone())
            .expect("mint");

        let req = TransferRequest::new(alice.clone(), bob.clone(), 250_000, meta.symbol.clone());
        ledger.transfer(req).expect("transfer");

        let alice_balance = ledger.balance_of(&alice, &meta.symbol).unwrap_or(0);
        let bob_balance = ledger.balance_of(&bob, &meta.symbol).unwrap_or(0);

        assert_eq!(alice_balance, 750_000);
        assert_eq!(bob_balance, 250_000);
    }
}
