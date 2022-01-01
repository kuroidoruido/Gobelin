use crate::{format_gobelin_file, AccountConfig, Config, GobelinFile, TransactionBucket};
use chrono::NaiveDate;

pub fn create_month_file(
    config: &Config,
    previous_month: &GobelinFile,
    year: i32,
    month: u32,
) -> Result<String, String> {
    let new_month = GobelinFile {
        month: NaiveDate::from_ymd(year, month, 1),
        transactions: config
            .accounts
            .iter()
            .map(|AccountConfig { name }| TransactionBucket {
                name: name.clone(),
                transactions: Vec::new(),
            })
            .collect(),
        tags: Vec::new(),
        balance: previous_month.balance.clone(),
    };
    Ok(format_gobelin_file(config, &new_month)?)
}
