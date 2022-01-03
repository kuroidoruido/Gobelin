use crate::Balance;

pub fn parse_balance_by_category<'a, I>(file: &mut I) -> Vec<Balance>
where
    I: Iterator<Item = &'a str>,
{
    let mut balances: Vec<Balance> = Vec::new();

    // file.next(); // skip "## Balance by category" title

    file.take_while(|x| x.starts_with('-'))
        .filter_map(|x| x[1..].split_once('='))
        .for_each(|(k, v)| {
            balances.push(Balance {
                name: k.trim().to_string(),
                amount: v.parse().unwrap(),
            });
        });
    balances.sort_by_key(|b| b.amount);
    balances.iter().rev().cloned().collect::<Vec<_>>()
}
