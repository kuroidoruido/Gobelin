use crate::parse_gobelin_file::compute_tags::compute_tags;
use crate::parse_gobelin_file::parse_month_title::parse_month_title;
use crate::{Balance, Config, GobelinFile, Transaction, TransactionBucket};

pub fn parse_gobelin_file(config: &Config, file: &String) -> Result<GobelinFile, String> {
    let mut file = file.split("\n").map(str::trim).filter(|x| !x.is_empty());

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

    // tags contains every tags used for transactions
    let tags = compute_tags(&transactions);

    Ok(GobelinFile {
        month,
        transactions,
        tags,
        balance,
    })
}

fn parse_transactions<'a, I>(config: &Config, file: &mut I) -> Vec<TransactionBucket>
where
    I: Iterator<Item = &'a str>,
{
    let mut transactions: Vec<TransactionBucket> = Vec::new();
    file.next(); // skip "## Transactions"

    let mut current_account_title: String = String::default();
    let mut current_account_transactions: Vec<Transaction> = Vec::new();
    while let Some(row) = file.next() {
        if row == "## Balance" {
            break;
        }
        if row.starts_with("###") {
            if !current_account_title.is_empty() {
                current_account_transactions.sort_by_key(|x| x.date);
                transactions.push(TransactionBucket {
                    name: current_account_title,
                    transactions: current_account_transactions,
                });
            }
            current_account_title = row[3..].trim().to_string();
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

    return transactions;
}

fn parse_balance<'a, I>(config: &Config, file: &mut I) -> Vec<Balance>
where
    I: Iterator<Item = &'a str>,
{
    let mut balances: Vec<Balance> = Vec::new();

    file.take_while(|x| x.starts_with("-"))
        .filter_map(|x| x[1..].split_once("="))
        .for_each(|(k, v)| {
            balances.push(Balance {
                name: k.trim().to_string(),
                amount: v.parse().unwrap(),
            });
        });
    balances.sort_by_key(|b| {
        config
            .accounts
            .iter()
            .position(|a| a.name == b.name)
            .or(Some(usize::MAX))
            .unwrap()
    });
    return balances;
}
