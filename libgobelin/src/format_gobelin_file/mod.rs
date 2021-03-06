mod format_balance;
mod format_balance_by_category;
mod format_month;
mod format_transactions;

use crate::{Config, GobelinFile};
use format_balance::format_balance;
use format_balance_by_category::format_balance_by_category;
use format_month::format_month;
use format_transactions::format_transactions;

pub fn format_gobelin_file(config: &Config, file: &GobelinFile) -> Result<String, String> {
    let mut res = String::default();
    format_month(&mut res, file, config.general.locale)?;
    format_transactions(config, &mut res, file);
    format_balance(&mut res, file);
    format_balance_by_category(&mut res, file);
    Ok(res)
}
