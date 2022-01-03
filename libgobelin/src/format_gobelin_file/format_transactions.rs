use crate::GobelinFile;

pub fn format_transactions(res: &mut String, file: &GobelinFile) {
    res.push_str("## Transactions\n\n");
    let amount_padding = file
        .transactions
        .iter()
        .flat_map(|b| {
            b.transactions
                .iter()
                .map(|t| t.amount.numerator_digit_count())
        })
        .max()
        .or(Some(0))
        .unwrap();
    let amount_padding =
        amount_padding + (amount_padding - if amount_padding > 0 { 1 } else { 0 }) / 3;
    let description_padding = file
        .transactions
        .iter()
        .flat_map(|b| b.transactions.iter().map(|t| t.description.chars().count()))
        .max()
        .or(Some(0))
        .unwrap();
    for bucket in file.transactions.iter() {
        res.push_str(format!("### {}\n\n", bucket.name).as_str());
        for transaction in bucket.transactions.iter() {
            let tag = if transaction.tag.is_some() {
                format!(" [{}]", transaction.tag.clone().unwrap())
            } else {
                String::default()
            };
            res.push_str(
                format!(
                    "{} {} {: >amount_padding$}{: >fill_decimal$} {: <description_padding$}{}\n",
                    transaction.date.format("%d/%m"),
                    transaction.amount.sign(),
                    format!("{}", transaction.amount.abs()),
                    "",
                    transaction.description,
                    tag,
                    amount_padding = amount_padding
                        + if transaction.amount.denominator > 0 {
                            3
                        } else {
                            0
                        },
                    fill_decimal = if transaction.amount.denominator > 0 {
                        0
                    } else {
                        3
                    },
                    description_padding = description_padding,
                )
                .as_str(),
            );
        }
        res.push('\n');
    }
}
