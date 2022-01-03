use crate::GobelinFile;

pub fn format_balance_by_category(res: &mut String, file: &GobelinFile) {
    if file.balance_by_category.is_empty() {
        return;
    }
    res.push_str("\n## Balance by category\n\n");
    let name_padding = file
        .balance_by_category
        .iter()
        .map(|b| b.name.len())
        .max()
        .or(Some(0))
        .unwrap();
    let amount_padding = file
        .balance_by_category
        .iter()
        .map(|b| b.amount.numerator_digit_count())
        .max()
        .or(Some(0))
        .unwrap();
    let amount_padding = amount_padding
        + if amount_padding > 1 {
            (amount_padding - 1) / 3
        } else {
            0
        };
    for balance in file.balance_by_category.iter() {
        res.push_str(
            format!(
                "- {: <name_pad$} = {} {: >amount_pad$}\n",
                balance.name,
                balance.amount.sign(),
                format!("{}", balance.amount.abs()),
                name_pad = name_padding,
                amount_pad = (amount_padding + if balance.amount.denominator > 0 { 3 } else { 0 })
            )
            .as_str(),
        );
    }
}
