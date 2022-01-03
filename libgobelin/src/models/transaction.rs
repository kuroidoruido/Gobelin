use crate::exact_float::ExactFloat;
use chrono::NaiveDate;
use regex::Regex;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct TransactionBucket {
    pub name: String,
    pub transactions: Vec<Transaction>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Transaction {
    pub date: NaiveDate,
    pub amount: ExactFloat,
    pub description: String,
    pub tag: Option<String>,
}

impl FromStr for Transaction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let re = Regex::new(r"^(\d{1,2}/\d{2}) ([\-+0-9\. ]+) ([^\[]+)( \[(.+)\])?$").unwrap();
        if let Some(capture) = re.captures(s) {
            let date = parse_date(capture.get(1).unwrap().as_str())?;
            let amount = parse_amount(capture.get(2).unwrap().as_str())?;
            let description: String = String::from(capture.get(3).unwrap().as_str().trim());
            let tag: Option<String> = capture.get(5).map(|x| x.as_str()).map(String::from);
            Ok(Self {
                date,
                amount,
                description,
                tag,
            })
        } else {
            Err(format!("Cannot parse transaction \"{}\"", s))
        }
    }
}
fn parse_date(date: &str) -> Result<NaiveDate, <Transaction as FromStr>::Err> {
    if let Some((day, month)) = date.split_once('/') {
        let parsed_month = month.parse::<u32>();
        if let Ok(month) = parsed_month {
            let parsed_day = day.parse::<u32>();
            if let Ok(day) = parsed_day {
                Ok(NaiveDate::from_ymd(1, month, day))
            } else {
                Err(format!("Cannot day as u32 '{}'", day))
            }
        } else {
            Err(format!("Cannot month as u32 '{}'", month))
        }
    } else {
        Err(format!("Cannot find day and month from date '{}'", date))
    }
}

fn parse_amount(amount: &str) -> Result<ExactFloat, <Transaction as FromStr>::Err> {
    let parsed_amount = amount.parse::<ExactFloat>();
    if parsed_amount.is_ok() {
        parsed_amount
    } else {
        Err(format!(
            "Cannot parse amount '{}' as ExactFloat because {:?}",
            amount, parsed_amount
        ))
    }
}
