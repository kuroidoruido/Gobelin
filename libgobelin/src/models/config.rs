use crate::models::Locale;

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
}
