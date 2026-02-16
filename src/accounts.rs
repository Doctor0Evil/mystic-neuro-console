use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{NeuroError, NeuroResult};

pub type AccountId = String;

/// Account metadata and balances.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: AccountId,
    pub alias: String,
    pub balances: Vec<TokenBalance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenBalance {
    pub symbol: String,
    pub amount: u128,
}

impl Account {
    pub fn new<S: Into<String>>(alias: S) -> Self {
        let id = Uuid::new_v4().to_string();
        Self {
            id,
            alias: alias.into(),
            balances: Vec::new(),
        }
    }

    pub fn get_balance_mut(&mut self, symbol: &str) -> Option<&mut TokenBalance> {
        self.balances.iter_mut().find(|b| b.symbol == symbol)
    }

    pub fn get_balance(&self, symbol: &str) -> Option<u128> {
        self.balances
            .iter()
            .find(|b| b.symbol == symbol)
            .map(|b| b.amount)
    }

    pub fn credit(&mut self, symbol: &str, amount: u128) -> NeuroResult<()> {
        if amount == 0 {
            return Err(NeuroError::InvalidAmount(amount));
        }

        match self.get_balance_mut(symbol) {
            Some(entry) => {
                entry.amount = entry
                    .amount
                    .checked_add(amount)
                    .ok_or(NeuroError::Overflow)?;
            }
            None => {
                self.balances.push(TokenBalance {
                    symbol: symbol.to_owned(),
                    amount,
                });
            }
        }

        Ok(())
    }

    pub fn debit(&mut self, symbol: &str, amount: u128) -> NeuroResult<()> {
        if amount == 0 {
            return Err(NeuroError::InvalidAmount(amount));
        }

        let entry = self
            .get_balance_mut(symbol)
            .ok_or_else(|| NeuroError::TokenNotFound(symbol.to_owned()))?;

        if entry.amount < amount {
            return Err(NeuroError::InsufficientBalance {
                required: amount,
                available: entry.amount,
            });
        }

        entry.amount -= amount;
        Ok(())
    }
}
