use crate::models::Locale;
use std::default::Default;

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub general: GeneralConfig,
    pub accounts: Vec<AccountConfig>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeneralConfig {
    pub locale: Locale,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccountConfig {
    pub name: String,
    pub empty_transaction: EmptyTransactionConfig,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EmptyTransactionConfig {
    Show,
    Hide,
}

impl Default for EmptyTransactionConfig {
    fn default() -> Self {
        Self::Show
    }
}
