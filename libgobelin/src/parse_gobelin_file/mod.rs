mod compute_tags;
mod parse_balance;
mod parse_balance_by_category;
mod parse_month_title;
mod parse_transactions;

use crate::parse_gobelin_file::compute_tags::compute_tags;
use crate::parse_gobelin_file::parse_balance::parse_balance;
use crate::parse_gobelin_file::parse_balance_by_category::parse_balance_by_category;
use crate::parse_gobelin_file::parse_month_title::parse_month_title;
use crate::parse_gobelin_file::parse_transactions::parse_transactions;
use crate::{Config, GobelinFile};

pub fn parse_gobelin_file(config: &Config, file: String) -> Result<GobelinFile, String> {
    let mut file = file.split('\n').map(str::trim).filter(|x| !x.is_empty());

    // first line should be month title
    // exp: # December 2021
    let month = parse_month_title(file.next().unwrap())?;

    // first part should start with ## Transactions
    // then a sequence of section of type
    // ### Account name
    // - transaction 1
    // - transaction 2
    let transactions = parse_transactions(config, &mut file);

    let balance = parse_balance(config, &mut file);

    let balance_by_category = parse_balance_by_category(&mut file);

    // tags contains every tags used for transactions
    let tags = compute_tags(&transactions);

    Ok(GobelinFile {
        month,
        transactions,
        tags,
        balance,
        balance_by_category,
    })
}
