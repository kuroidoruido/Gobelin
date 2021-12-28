use chrono::NaiveDate;
use libgobelin::{ExactFloat, Transaction};

mod from_str {
    use super::*;

    #[test]
    fn it_should_parse_not_formatted_transaction() {
        let str_transaction = String::from("2/12 +100 the description goes here [tag]");

        let expected = Transaction {
            date: NaiveDate::from_ymd(1, 12, 2),
            amount: ExactFloat::new(100, 0),
            description: String::from("the description goes here"),
            tag: Some(String::from("tag")),
        };
        assert_eq!(str_transaction.parse::<Transaction>(), Ok(expected));
    }

    #[test]
    fn it_should_parse_formatted_transaction() {
        let str_transaction = String::from("02/12 +   1 100    the description goes here [tag]");

        let expected = Transaction {
            date: NaiveDate::from_ymd(1, 12, 2),
            amount: ExactFloat::new(1100, 0),
            description: String::from("the description goes here"),
            tag: Some(String::from("tag")),
        };
        assert_eq!(str_transaction.parse::<Transaction>(), Ok(expected));
    }

    #[test]
    fn it_should_parse_transaction_without_tag() {
        let str_transaction = String::from("2/12 +100 the description goes here");

        let expected = Transaction {
            date: NaiveDate::from_ymd(1, 12, 2),
            amount: ExactFloat::new(100, 0),
            description: String::from("the description goes here"),
            tag: None,
        };
        assert_eq!(str_transaction.parse::<Transaction>(), Ok(expected));
    }

    #[test]
    fn it_should_trim_description() {
        let str_transaction = String::from("2/12 +100   the description goes here  ");

        let expected = Transaction {
            date: NaiveDate::from_ymd(1, 12, 2),
            amount: ExactFloat::new(100, 0),
            description: String::from("the description goes here"),
            tag: None,
        };
        assert_eq!(str_transaction.parse::<Transaction>(), Ok(expected));
    }
}
