use serde::{Deserialize, Serialize};

/// Token supply tracking.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TokenSupply {
    pub total: u128,
}

impl TokenSupply {
    pub fn new() -> Self {
        Self { total: 0 }
    }

    pub fn mint(&mut self, amount: u128) -> Option<u128> {
        if amount == 0 {
            return None;
        }
        self.total = self.total.checked_add(amount)?;
        Some(self.total)
    }

    pub fn burn(&mut self, amount: u128) -> Option<u128> {
        if amount == 0 || amount > self.total {
            return None;
        }
        self.total -= amount;
        Some(self.total)
    }
}

/// Token metadata, symbol is the primary key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenMeta {
    pub symbol: String,
    pub decimals: u8,
    pub name: String,
}

impl TokenMeta {
    pub fn new<S: Into<String>>(symbol: S, decimals: u8, name: S) -> Self {
        Self {
            symbol: symbol.into(),
            decimals,
            name: name.into(),
        }
    }
}
