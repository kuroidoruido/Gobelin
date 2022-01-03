use crate::{Config, Transaction, TransactionBucket};

pub fn parse_transactions<'a, I>(config: &Config, file: &mut I) -> Vec<TransactionBucket>
where
    I: Iterator<Item = &'a str>,
{
    let mut transactions: Vec<TransactionBucket> = Vec::new();
    file.next(); // skip "## Transactions"

    let mut current_account_title: String = String::default();
    let mut current_account_transactions: Vec<Transaction> = Vec::new();
    for row in file {
        if row == "## Balance" {
            break;
        }
        if let Some(account_title) = row.strip_prefix("###") {
            if !current_account_title.is_empty() {
                current_account_transactions.sort_by_key(|x| x.date);
                transactions.push(TransactionBucket {
                    name: current_account_title,
                    transactions: current_account_transactions,
                });
            }
            current_account_title = account_title.trim().to_string();
            current_account_transactions = Vec::new();
        } else {
            let transaction = row.parse::<Transaction>();
            if let Ok(transaction) = transaction {
                current_account_transactions.push(transaction);
            } else {
                panic!("Invalid transaction: {:?}", transaction)
            }
        }
    }
    if !current_account_title.is_empty() {
        current_account_transactions.sort_by_key(|x| x.date);
        transactions.push(TransactionBucket {
            name: current_account_title,
            transactions: current_account_transactions,
        });
    }

    transactions.sort_by_key(|b| {
        config
            .accounts
            .iter()
            .position(|a| a.name == b.name)
            .or(Some(usize::MAX))
            .unwrap()
    });

    transactions
}
