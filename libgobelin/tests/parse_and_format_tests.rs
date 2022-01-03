use chrono::NaiveDate;
use libgobelin::{
    format_gobelin_file, parse_gobelin_file, AccountConfig, Balance, Config,
    EmptyTransactionConfig, ExactFloat, GeneralConfig, GobelinFile, Locale, Transaction,
    TransactionBucket,
};

fn sample_config() -> Config {
    Config {
        general: GeneralConfig { locale: Locale::FR },
        accounts: vec![
            AccountConfig {
                name: String::from("Main account"),
                empty_transaction: EmptyTransactionConfig::default(),
            },
            AccountConfig {
                name: String::from("Savings account 1"),
                empty_transaction: EmptyTransactionConfig::default(),
            },
            AccountConfig {
                name: String::from("Savings account 2"),
                empty_transaction: EmptyTransactionConfig::default(),
            },
            AccountConfig {
                name: String::from("Shared account"),
                empty_transaction: EmptyTransactionConfig::default(),
            },
        ],
    }
}

fn sample() -> GobelinFile {
    GobelinFile {
        month: NaiveDate::from_ymd(2021, 12, 1),
        transactions: vec![
            TransactionBucket {
                name: String::from("Main account"),
                transactions: vec![
                    String::from("02/12 + 1 800.67 Salary [salary]")
                        .parse::<Transaction>()
                        .unwrap(),
                    String::from("02/12 -   500    saving 1 [<=>]")
                        .parse::<Transaction>()
                        .unwrap(),
                    String::from("02/12 -   850    Shared account décembre [<=>]")
                        .parse::<Transaction>()
                        .unwrap(),
                    String::from("05/12 -    29.99 Internet [telecom]")
                        .parse::<Transaction>()
                        .unwrap(),
                    String::from("07/12 -    19.99 Phone [telecom]")
                        .parse::<Transaction>()
                        .unwrap(),
                ],
            },
            TransactionBucket {
                name: String::from("Savings account 1"),
                transactions: vec![String::from("02/12 +   500    saving 1 [<=>]")
                    .parse::<Transaction>()
                    .unwrap()],
            },
            TransactionBucket {
                name: String::from("Savings account 2"),
                transactions: vec![],
            },
            TransactionBucket {
                name: String::from("Shared account"),
                transactions: vec![
                    String::from("02/12 +   850    Shared account décembre [<=>]")
                        .parse::<Transaction>()
                        .unwrap(),
                    String::from("02/12 +   520    Shared account décembre")
                        .parse::<Transaction>()
                        .unwrap(),
                    String::from("07/12 -   780.42 apartment rent [home]")
                        .parse::<Transaction>()
                        .unwrap(),
                    String::from("07/12 -    60    box rent [home]")
                        .parse::<Transaction>()
                        .unwrap(),
                    String::from("10/12 -    80    Shopping [food]")
                        .parse::<Transaction>()
                        .unwrap(),
                    String::from("21/12 -   250    Car [car]")
                        .parse::<Transaction>()
                        .unwrap(),
                ],
            },
        ],
        tags: vec![
            String::from("car"),
            String::from("food"),
            String::from("home"),
            String::from("salary"),
            String::from("telecom"),
        ],
        balance: vec![
            Balance {
                name: String::from("Main account"),
                amount: ExactFloat::new(400, 69),
            },
            Balance {
                name: String::from("Savings account 1"),
                amount: ExactFloat::new(8000, 0),
            },
            Balance {
                name: String::from("Savings account 2"),
                amount: ExactFloat::new(15000, 0),
            },
            Balance {
                name: String::from("Shared account"),
                amount: ExactFloat::new(110, 0),
            },
        ],
    }
}

#[test]
fn formatted_file_should_be_parseable() {
    let config = sample_config();
    let file = sample();
    assert_eq!(
        parse_gobelin_file(&config, format_gobelin_file(&config, &file).unwrap()),
        Ok(file)
    );
}

#[test]
fn formatting_should_be_consistent_over_parse_and_format_operation() {
    let config = sample_config();
    let file = sample();
    let formatted_1 = format_gobelin_file(&config, &file).unwrap();
    let formatted_2 = format_gobelin_file(
        &config,
        &parse_gobelin_file(&config, formatted_1.clone()).unwrap(),
    )
    .unwrap();
    let formatted_3 = format_gobelin_file(
        &config,
        &parse_gobelin_file(&config, formatted_2.clone()).unwrap(),
    )
    .unwrap();
    let formatted_4 = format_gobelin_file(
        &config,
        &parse_gobelin_file(&config, formatted_3.clone()).unwrap(),
    )
    .unwrap();

    assert_eq!(formatted_2, formatted_1);
    assert_eq!(formatted_3, formatted_1);
    assert_eq!(formatted_4, formatted_1);
}
