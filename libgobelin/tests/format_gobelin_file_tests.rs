use chrono::NaiveDate;
use libgobelin::{
    format_gobelin_file, AccountConfig, Balance, Config, EmptyTransactionConfig, ExactFloat,
    GeneralConfig, GobelinFile, Locale, Transaction, TransactionBucket,
};

#[test]
fn it_should_format_correctly_minimal_file() {
    let config = Config {
        general: GeneralConfig { locale: Locale::FR },
        accounts: vec![AccountConfig {
            name: String::from("Compte principal"),
            empty_transaction: EmptyTransactionConfig::default(),
        }],
    };
    let file = GobelinFile {
        month: NaiveDate::from_ymd(2022, 2, 1),
        transactions: vec![TransactionBucket {
            name: String::from("Compte principal"),
            transactions: vec![],
        }],
        tags: vec![],
        balance: vec![Balance {
            name: String::from("Compte principal"),
            amount: ExactFloat::new(400, 69),
        }],
    };
    let expected = String::from(
        "# Février 2022

## Transactions

### Compte principal


## Balance

- Compte principal = 400.69
",
    );
    assert_eq!(format_gobelin_file(&config, &file), Ok(expected));
}

#[test]
fn it_should_format_balance_with_correct_padding() {
    let config = Config {
        general: GeneralConfig { locale: Locale::FR },
        accounts: vec![AccountConfig {
            name: String::from("Compte principal"),
            empty_transaction: EmptyTransactionConfig::default(),
        }],
    };
    let file = GobelinFile {
        month: NaiveDate::from_ymd(2022, 2, 1),
        transactions: vec![TransactionBucket {
            name: String::from("Compte principal"),
            transactions: vec![],
        }],
        tags: vec![],
        balance: vec![
            Balance {
                name: String::from("Compte principal"),
                amount: ExactFloat::new(400, 69),
            },
            Balance {
                name: String::from("Livret 1"),
                amount: ExactFloat::new(1400, 69),
            },
        ],
    };
    let expected = String::from(
        "# Février 2022

## Transactions

### Compte principal


## Balance

- Compte principal =   400.69
- Livret 1         = 1 400.69
",
    );
    assert_eq!(format_gobelin_file(&config, &file), Ok(expected));
}

#[test]
fn it_should_format_correctly_realistic_file() {
    let config = Config {
        general: GeneralConfig { locale: Locale::FR },
        accounts: vec![
            AccountConfig {
                name: String::from("Compte principal"),
                empty_transaction: EmptyTransactionConfig::default(),
            },
            AccountConfig {
                name: String::from("Livret 1"),
                empty_transaction: EmptyTransactionConfig::default(),
            },
            AccountConfig {
                name: String::from("Livret 2"),
                empty_transaction: EmptyTransactionConfig::default(),
            },
            AccountConfig {
                name: String::from("Compte joint"),
                empty_transaction: EmptyTransactionConfig::default(),
            },
        ],
    };
    let file = GobelinFile {
        month: NaiveDate::from_ymd(2021, 12, 1),
        transactions: vec![
            TransactionBucket {
                name: String::from("Compte principal"),
                transactions: vec![
                    String::from("02/12 + 1 800.67 Salaire [salaire]")
                        .parse::<Transaction>()
                        .unwrap(),
                    String::from("02/12 -   500    Épagne 1 [<=>]")
                        .parse::<Transaction>()
                        .unwrap(),
                    String::from("02/12 -   850    compte joint décembre [<=>]")
                        .parse::<Transaction>()
                        .unwrap(),
                    String::from("05/12 -    29.99 Internet [telecom]")
                        .parse::<Transaction>()
                        .unwrap(),
                    String::from("07/12 -    19.99 Mobile [telecom]")
                        .parse::<Transaction>()
                        .unwrap(),
                ],
            },
            TransactionBucket {
                name: String::from("Livret 1"),
                transactions: vec![String::from("02/12 +   500    épagne 1 [<=>]")
                    .parse::<Transaction>()
                    .unwrap()],
            },
            TransactionBucket {
                name: String::from("Livret 2"),
                transactions: vec![],
            },
            TransactionBucket {
                name: String::from("Compte joint"),
                transactions: vec![
                    String::from("02/12 +   850    compte joint décembre [<=>]")
                        .parse::<Transaction>()
                        .unwrap(),
                    String::from("02/12 +   520    compte joint décembre")
                        .parse::<Transaction>()
                        .unwrap(),
                    String::from("07/12 -   780.42 Loyer appartement [logement]")
                        .parse::<Transaction>()
                        .unwrap(),
                    String::from("07/12 -    60    Loyer box [logement]")
                        .parse::<Transaction>()
                        .unwrap(),
                    String::from("10/12 -    80    courses [courses]")
                        .parse::<Transaction>()
                        .unwrap(),
                    String::from("21/12 -   250    Voiture [voiture]")
                        .parse::<Transaction>()
                        .unwrap(),
                ],
            },
        ],
        tags: vec![
            String::from("courses"),
            String::from("logement"),
            String::from("salaire"),
            String::from("telecom"),
            String::from("voiture"),
        ],
        balance: vec![
            Balance {
                name: String::from("Compte principal"),
                amount: ExactFloat::new(400, 69),
            },
            Balance {
                name: String::from("Livret 1"),
                amount: ExactFloat::new(8000, 0),
            },
            Balance {
                name: String::from("Livret 2"),
                amount: ExactFloat::new(15000, 0),
            },
            Balance {
                name: String::from("Compte joint"),
                amount: ExactFloat::new(110, 0),
            },
        ],
    };
    let expected = String::from(
        "# Décembre 2021

## Transactions

### Compte principal

02/12 + 1 800.67 Salaire               [salaire]
02/12 -   500    Épagne 1              [<=>]
02/12 -   850    compte joint décembre [<=>]
05/12 -    29.99 Internet              [telecom]
07/12 -    19.99 Mobile                [telecom]

### Livret 1

02/12 +   500    épagne 1              [<=>]

### Livret 2


### Compte joint

02/12 +   850    compte joint décembre [<=>]
02/12 +   520    compte joint décembre
07/12 -   780.42 Loyer appartement     [logement]
07/12 -    60    Loyer box             [logement]
10/12 -    80    courses               [courses]
21/12 -   250    Voiture               [voiture]

## Balance

- Compte principal =    400.69
- Livret 1         =  8 000
- Livret 2         = 15 000
- Compte joint     =    110
",
    );
    assert_eq!(format_gobelin_file(&config, &file), Ok(expected));
}

#[test]
fn it_should_format_correctly_realistic_file_2() {
    let config = Config {
        general: GeneralConfig { locale: Locale::EN },
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

    let file = GobelinFile {
        month: NaiveDate::from_ymd(2021, 11, 1),
        transactions: vec![
            TransactionBucket {
                name: String::from("Main account"),
                transactions: vec![
                    Transaction {
                        date: NaiveDate::from_ymd(1, 11, 1),
                        amount: ExactFloat::new(1800, 0),
                        description: String::from("salary"),
                        tag: Some(String::from("salary")),
                    },
                    Transaction {
                        date: NaiveDate::from_ymd(1, 11, 2),
                        amount: ExactFloat::new(-200, 0),
                        description: String::from("savings"),
                        tag: Some(String::from("<=>")),
                    },
                    Transaction {
                        date: NaiveDate::from_ymd(1, 11, 3),
                        amount: ExactFloat::new(-29, 99),
                        description: String::from("internet"),
                        tag: Some(String::from("telecom")),
                    },
                    Transaction {
                        date: NaiveDate::from_ymd(1, 11, 3),
                        amount: ExactFloat::new(-19, 99),
                        description: String::from("mobile"),
                        tag: Some(String::from("telecom")),
                    },
                ],
            },
            TransactionBucket {
                name: String::from("Savings account"),
                transactions: vec![Transaction {
                    date: NaiveDate::from_ymd(1, 11, 2),
                    amount: ExactFloat::new(200, 0),
                    description: String::from("savings"),
                    tag: Some(String::from("<=>")),
                }],
            },
        ],
        tags: vec![String::from("salary"), String::from("telecom")],
        balance: vec![
            Balance {
                name: String::from("Main account"),
                amount: ExactFloat::new(1650, 2),
            },
            Balance {
                name: String::from("Savings account"),
                amount: ExactFloat::new(1200, 0),
            },
        ],
    };
    let expected = String::from(
        "# November 2021

## Transactions

### Main account

01/11 + 1 800    salary   [salary]
02/11 -   200    savings  [<=>]
03/11 -    29.99 internet [telecom]
03/11 -    19.99 mobile   [telecom]

### Savings account

02/11 +   200    savings  [<=>]

## Balance

- Main account    = 1 650.02
- Savings account = 1 200
",
    );
    assert_eq!(format_gobelin_file(&config, &file), Ok(expected));
}

#[test]
fn it_should_format_ignoring_accounts_without_transactions_when_config_empty_transaction_is_set_to_hide(
) {
    let config = Config {
        general: GeneralConfig { locale: Locale::EN },
        accounts: vec![
            AccountConfig {
                name: String::from("Main account"),
                empty_transaction: EmptyTransactionConfig::default(),
            },
            AccountConfig {
                name: String::from("Savings account 1"),
                empty_transaction: EmptyTransactionConfig::Hide,
            },
            AccountConfig {
                name: String::from("Savings account 2"),
                empty_transaction: EmptyTransactionConfig::default(),
            },
        ],
    };

    let file = GobelinFile {
        month: NaiveDate::from_ymd(2021, 11, 1),
        transactions: vec![
            TransactionBucket {
                name: String::from("Main account"),
                transactions: vec![
                    Transaction {
                        date: NaiveDate::from_ymd(1, 11, 1),
                        amount: ExactFloat::new(1800, 0),
                        description: String::from("salary"),
                        tag: Some(String::from("salary")),
                    },
                    Transaction {
                        date: NaiveDate::from_ymd(1, 11, 2),
                        amount: ExactFloat::new(-200, 0),
                        description: String::from("savings"),
                        tag: Some(String::from("<=>")),
                    },
                    Transaction {
                        date: NaiveDate::from_ymd(1, 11, 3),
                        amount: ExactFloat::new(-29, 99),
                        description: String::from("internet"),
                        tag: Some(String::from("telecom")),
                    },
                    Transaction {
                        date: NaiveDate::from_ymd(1, 11, 3),
                        amount: ExactFloat::new(-19, 99),
                        description: String::from("mobile"),
                        tag: Some(String::from("telecom")),
                    },
                ],
            },
            TransactionBucket {
                name: String::from("Savings account 1"),
                transactions: vec![],
            },
            TransactionBucket {
                name: String::from("Savings account 2"),
                transactions: vec![],
            },
        ],
        tags: vec![String::from("salary"), String::from("telecom")],
        balance: vec![
            Balance {
                name: String::from("Main account"),
                amount: ExactFloat::new(1650, 2),
            },
            Balance {
                name: String::from("Savings account 1"),
                amount: ExactFloat::new(1200, 0),
            },
            Balance {
                name: String::from("Savings account 2"),
                amount: ExactFloat::new(1200, 0),
            },
        ],
    };
    let expected = String::from(
        "# November 2021

## Transactions

### Main account

01/11 + 1 800    salary   [salary]
02/11 -   200    savings  [<=>]
03/11 -    29.99 internet [telecom]
03/11 -    19.99 mobile   [telecom]

### Savings account 2


## Balance

- Main account      = 1 650.02
- Savings account 1 = 1 200
- Savings account 2 = 1 200
",
    );
    assert_eq!(format_gobelin_file(&config, &file), Ok(expected));
}
