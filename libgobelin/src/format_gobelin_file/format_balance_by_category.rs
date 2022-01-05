use crate::{compute_unicode_gauge, GobelinFile};

pub fn format_balance_by_category(res: &mut String, file: &GobelinFile) {
    if file.balance_by_category.is_empty() {
        return;
    }
    res.push_str("\n## Balance by category\n\n");
    let abs_max = file
        .balance_by_category
        .iter()
        .map(|b| b.amount.numerator.abs())
        .max()
        .or(Some(0))
        .unwrap();
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
        let percent_of_max = balance.amount * 100 / abs_max;
        let gauge = compute_unicode_gauge(percent_of_max);
        res.push_str(
            format!(
                "- {: <name_pad$} = {} {: >amount_pad$}{: >fill_decimal$} {: >3}% {}\n",
                balance.name,
                balance.amount.sign(),
                format!("{}", balance.amount.abs()),
                "",
                percent_of_max,
                gauge,
                name_pad = name_padding,
                amount_pad = (amount_padding + if balance.amount.denominator > 0 { 3 } else { 0 }),
                fill_decimal = if balance.amount.denominator > 0 { 0 } else { 3 }
            )
            .as_str(),
        );
    }
}
