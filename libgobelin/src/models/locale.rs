use std::default::Default;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Locale {
    FR,
    EN,
}

impl FromStr for Locale {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "fr" => Ok(Locale::FR),
            "en" => Ok(Locale::EN),
            _ => Err(format!(
                "Unknown locale '{}'. You should choose any of these locale: EN, FR",
                s
            )),
        }
    }
}

impl Default for Locale {
    fn default() -> Self {
        Self::EN
    }
}

pub fn parse_month_word(s: &str) -> Result<u32, String> {
    match s.to_lowercase().as_str() {
        "janvier" | "january" => Ok(1),
        "février" | "fevrier" | "february" => Ok(2),
        "mars" | "march" => Ok(3),
        "avril" | "april" => Ok(4),
        "mai" | "may" => Ok(5),
        "juin" | "june" => Ok(6),
        "juillet" | "july" => Ok(7),
        "août" | "aout" | "august" => Ok(8),
        "septembre" | "september" => Ok(9),
        "octobre" | "october" => Ok(10),
        "novembre" | "november" => Ok(11),
        "décembre" | "decembre" | "december" => Ok(12),
        _ => Err(format!(
            "Cannot parse as month title because month word '{}' is not recognized",
            s
        )),
    }
}

pub fn format_month_word(month: u32, locale: Locale) -> Result<String, String> {
    fn unknown_month(month: u32) -> Result<String, String> {
        Err(format!("Invalid month {}", month))
    }
    match locale {
        Locale::FR => match month {
            1 => Ok(String::from("Janvier")),
            2 => Ok(String::from("Février")),
            3 => Ok(String::from("Mars")),
            4 => Ok(String::from("Avril")),
            5 => Ok(String::from("Mai")),
            6 => Ok(String::from("Juin")),
            7 => Ok(String::from("Juillet")),
            8 => Ok(String::from("Août")),
            9 => Ok(String::from("Septembre")),
            10 => Ok(String::from("Octobre")),
            11 => Ok(String::from("Novembre")),
            12 => Ok(String::from("Décembre")),
            _ => unknown_month(month),
        },
        Locale::EN => match month {
            1 => Ok(String::from("January")),
            2 => Ok(String::from("February")),
            3 => Ok(String::from("March")),
            4 => Ok(String::from("April")),
            5 => Ok(String::from("May")),
            6 => Ok(String::from("June")),
            7 => Ok(String::from("July")),
            8 => Ok(String::from("August")),
            9 => Ok(String::from("September")),
            10 => Ok(String::from("October")),
            11 => Ok(String::from("November")),
            12 => Ok(String::from("December")),
            _ => unknown_month(month),
        },
    }
}

pub fn main_account_name(locale: Locale) -> String {
    match locale {
        Locale::FR => String::from("Compte principal"),
        Locale::EN => String::from("Main account"),
    }
}
