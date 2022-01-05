use crate::{
    format_gobelin_file, main_account_name, AccountConfig, Balance, Config, EmptyTransactionConfig,
    ExactFloat, GeneralConfig, GobelinFile, Locale, Transaction, TransactionBucket,
};
use chrono::NaiveDate;
use std::collections::BTreeMap;

pub fn init_gobelin_directory(
    accounts: &[String],
    locale: Option<Locale>,
    year: i32,
    month: u32,
) -> Result<BTreeMap<String, String>, String> {
    let (accounts, locale) = default_parameters(accounts, locale)?;

    Ok(BTreeMap::from([
        (
            String::from("gobelin.toml"),
            build_gobelin_toml(&accounts, locale)?,
        ),
        (
            format!("{}/{}.gobelin", year, month),
            build_month_file(&accounts, locale, year, month)?,
        ),
    ]))
}

fn default_parameters(
    accounts: &[String],
    locale: Option<Locale>,
) -> Result<(Vec<String>, Locale), String> {
    let locale = locale.or_else(|| Some(Locale::default())).unwrap();
    let accounts: Vec<String> = if accounts.is_empty() {
        vec![main_account_name(locale)]
    } else {
        accounts.to_vec()
    };
    Ok((accounts, locale))
}

fn build_gobelin_toml(accounts: &[String], locale: Locale) -> Result<String, String> {
    let base = format!(
        "[general]
locale = \"{:?}\"

[accounts]

",
        locale
    );
    let accounts = accounts
        .iter()
        .enumerate()
        .map(|(i, a)| format!("[accounts.{}]\nname = \"{}\"\n", i + 1, a))
        .collect::<Vec<_>>()
        .join("\n");
    Ok(format!("{}{}\n", base, accounts))
}

fn build_month_file(
    accounts: &[String],
    locale: Locale,
    year: i32,
    month: u32,
) -> Result<String, String> {
    let config = Config {
        general: GeneralConfig { locale },
        accounts: accounts
            .iter()
            .cloned()
            .map(|name| AccountConfig {
                name,
                empty_transaction: EmptyTransactionConfig::default(),
            })
            .collect(),
    };
    let transactions: Vec<TransactionBucket> = accounts
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, name)| {
            let decimal: u8 = (i % 100).try_into().unwrap();
            TransactionBucket {
                name,
                transactions: vec![Transaction {
                    date: NaiveDate::from_ymd(year, month, 1),
                    amount: ExactFloat::new(100, decimal + 1),
                    description: init_amount(locale),
                    tag: Some(String::from("<=>")),
                }],
            }
        })
        .collect();
    let file = GobelinFile {
        month: NaiveDate::from_ymd(year, month, 1),
        transactions,
        tags: Vec::new(),
        balance: accounts
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, name)| {
                let decimal: u8 = (i % 100).try_into().unwrap();
                Balance {
                    name,
                    amount: ExactFloat::new(100, decimal + 1),
                }
            })
            .collect(),
        balance_by_category: Vec::new(),
    };
    format_gobelin_file(&config, &file)
}

fn init_amount(locale: Locale) -> String {
    match locale {
        Locale::FR => String::from("montant initial"),
        Locale::EN => String::from("initial amount"),
    }
}
