use crate::{format_month_word, GobelinFile, Locale};

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
