use chrono::NaiveDate;
use libgobelin::{
    create_month_file, AccountConfig, Balance, Config, EmptyTransactionConfig, ExactFloat,
    GeneralConfig, GobelinFile, Locale,
};

#[test]
fn it_should_use_year_and_month() {
    let config = Config {
        general: GeneralConfig { locale: Locale::FR },
        accounts: Vec::new(),
    };
    let previous_month = GobelinFile {
        month: NaiveDate::from_ymd(1, 1, 1),
        transactions: Vec::new(),
        tags: Vec::new(),
        balance: Vec::new(),
        balance_by_category: Vec::new(),
    };

    let actual = create_month_file(&config, &previous_month, 2021, 2).unwrap();

    let expected = String::from(
        "# Février 2021

## Transactions

## Balance

",
    );
    assert_eq!(actual, expected);
}

#[test]
fn it_should_reuse_previous_month_balance() {
    let config = Config {
        general: GeneralConfig { locale: Locale::FR },
        accounts: Vec::new(),
    };
    let previous_month = GobelinFile {
        month: NaiveDate::from_ymd(1, 1, 1),
        transactions: Vec::new(),
        tags: Vec::new(),
        balance: vec![
            Balance {
                name: String::from("Main account"),
                amount: ExactFloat::new(111, 11),
            },
            Balance {
                name: String::from("Savings account"),
                amount: ExactFloat::new(222, 22),
            },
        ],
        balance_by_category: Vec::new(),
    };

    let actual = create_month_file(&config, &previous_month, 2021, 12).unwrap();

    let expected = String::from(
        "# Décembre 2021

## Transactions

## Balance

- Main account    = 111.11
- Savings account = 222.22
",
    );
    assert_eq!(actual, expected);
}

#[test]
fn it_should_use_config_account_list_to_create_transactions_sections() {
    let config = Config {
        general: GeneralConfig { locale: Locale::FR },
        accounts: vec![
            AccountConfig {
                name: String::from("Main account"),
                empty_transaction: EmptyTransactionConfig::default(),
            },
            AccountConfig {
                name: String::from("Savings account"),
                empty_transaction: EmptyTransactionConfig::default(),
            },
        ],
    };
    let previous_month = GobelinFile {
        month: NaiveDate::from_ymd(1, 1, 1),
        transactions: Vec::new(),
        tags: Vec::new(),
        balance: Vec::new(),
        balance_by_category: Vec::new(),
    };

    let actual = create_month_file(&config, &previous_month, 2021, 12).unwrap();

    let expected = String::from(
        "# Décembre 2021

## Transactions

### Main account


### Savings account


## Balance

",
    );
    assert_eq!(actual, expected);
}

#[test]
fn it_should_create_new_month_correctly() {
    let config = Config {
        general: GeneralConfig { locale: Locale::FR },
        accounts: vec![
            AccountConfig {
                name: String::from("Main account"),
                empty_transaction: EmptyTransactionConfig::default(),
            },
            AccountConfig {
                name: String::from("Savings account"),
                empty_transaction: EmptyTransactionConfig::default(),
            },
        ],
    };
    let previous_month = GobelinFile {
        month: NaiveDate::from_ymd(1, 1, 1),
        transactions: Vec::new(),
        tags: Vec::new(),
        balance: vec![
            Balance {
                name: String::from("Main account"),
                amount: ExactFloat::new(111, 11),
            },
            Balance {
                name: String::from("Savings account"),
                amount: ExactFloat::new(222, 22),
            },
        ],
        balance_by_category: Vec::new(),
    };

    let actual = create_month_file(&config, &previous_month, 2022, 1).unwrap();

    let expected = String::from(
        "# Janvier 2022

## Transactions

### Main account


### Savings account


## Balance

- Main account    = 111.11
- Savings account = 222.22
",
    );
    assert_eq!(actual, expected);
}
