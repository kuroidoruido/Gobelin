use crate::{format_month_word, Config, GobelinFile, Locale};

pub fn format_gobelin_file(config: &Config, file: &GobelinFile) -> Result<String, String> {
    let mut res = String::default();
    format_month(&mut res, &file, config.general.locale)?;
    format_transactions(&mut res, &file);
    format_balance(&mut res, &file);
    return Ok(res);
}

pub fn format_month(res: &mut String, file: &GobelinFile, locale: Locale) -> Result<(), String> {
    res.push_str(
        format!(
            "# {} {}\n\n",
            format_month_word(
                file.month.format("%m").to_string().parse::<u32>().unwrap(),
                locale
            )?,
            file.month.format("%Y")
        )
        .as_str(),
    );
    Ok(())
}

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
        .flat_map(|b| b.transactions.iter().map(|t| t.description.len()))
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
                    description_padding = description_padding - 1,
                )
                .as_str(),
            );
        }
        res.push_str("\n");
    }
}

pub fn format_balance(res: &mut String, file: &GobelinFile) {
    res.push_str("## Balance\n\n");
    let name_padding = file.balance.iter().map(|b| b.name.len()).max().unwrap();
    let amount_padding = file
        .balance
        .iter()
        .map(|b| b.amount.numerator_digit_count())
        .max()
        .or(Some(0))
        .unwrap();
    let amount_padding = amount_padding + (amount_padding - 1) / 3;
    for balance in file.balance.iter() {
        res.push_str(
            format!(
                "- {: <name_pad$} = {: >amount_pad$}\n",
                balance.name,
                format!("{}", balance.amount),
                name_pad = name_padding,
                amount_pad = (amount_padding + if balance.amount.denominator > 0 { 3 } else { 0 })
            )
            .as_str(),
        );
    }
}
