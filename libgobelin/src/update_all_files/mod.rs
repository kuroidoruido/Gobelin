use crate::{format_gobelin_file, parse_gobelin_file, Balance, Config, ExactFloat};
use std::collections::BTreeMap;

pub fn update_all_files(
    config: &Config,
    files: &BTreeMap<String, String>,
) -> Result<BTreeMap<String, String>, String> {
    let mut update_files: BTreeMap<String, String> = BTreeMap::new();
    let mut previous: Option<Vec<Balance>> = None;
    for (file_path, file_content) in files.iter() {
        let mut current_file = parse_gobelin_file(config, file_content.clone())?;
        if let Some(previous) = previous {
            current_file.balance = previous.clone();
        } else {
            current_file.balance = config
                .accounts
                .iter()
                .map(|a| Balance {
                    name: a.name.clone(),
                    amount: ExactFloat::new(0, 0),
                })
                .collect();
        }
        let transactions_balance: Vec<Balance> = current_file
            .transactions
            .iter()
            .map(|bucket| Balance {
                name: bucket.name.clone(),
                amount: bucket.transactions.iter().map(|t| t.amount).sum(),
            })
            .collect();

        current_file.balance = current_file
            .balance
            .iter()
            .map(|current_balance| {
                let transaction_balance = transactions_balance
                    .iter()
                    .find(|b| b.name == current_balance.name);
                if let Some(transaction_balance) = transaction_balance {
                    Balance {
                        name: current_balance.name.clone(),
                        amount: current_balance.amount + transaction_balance.amount,
                    }
                } else {
                    current_balance.clone()
                }
            })
            .collect();

        update_files.insert(
            file_path.clone(),
            format_gobelin_file(config, &current_file)?,
        );
        previous = Some(current_file.balance.clone());
    }
    Ok(update_files)
}
