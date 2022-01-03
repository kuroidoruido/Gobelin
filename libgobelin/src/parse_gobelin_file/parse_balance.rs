use crate::{Balance, Config};

pub fn parse_balance<'a, I>(config: &Config, file: &mut I) -> Vec<Balance>
where
    I: Iterator<Item = &'a str>,
{
    let mut balances: Vec<Balance> = Vec::new();

    file.take_while(|x| x.starts_with('-'))
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
    balances
}
