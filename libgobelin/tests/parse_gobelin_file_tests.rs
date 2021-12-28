use chrono::NaiveDate;
use libgobelin::{
    parse_gobelin_file, AccountConfig, Balance, Config, ExactFloat, GeneralConfig, GobelinFile,
    Locale, Transaction, TransactionBucket,
};

#[test]
fn should_parse_formatted_file() {
    let config = Config {
        general: GeneralConfig { locale: Locale::FR },
        accounts: vec![
            AccountConfig {
                name: String::from("Compte principal"),
            },
            AccountConfig {
                name: String::from("Livret 1"),
            },
            AccountConfig {
                name: String::from("Livret 2"),
            },
            AccountConfig {
                name: String::from("Compte joint"),
            },
        ],
    };
    let file = String::from(
        "# Décembre 2021

## Transactions

### Compte principal

07/12 -    19.99 Mobile                [telecom]
05/12 -    29.99 Internet              [telecom]
02/12 + 1 800.67 Salaire               [salaire]
02/12 -   500    Épagne 1              [<=>]
02/12 -   850    compte joint décembre [<=>]

### Livret 1

02/12 +   500    épagne 1              [<=>]

### Livret 2


### Compte joint

02/12 +   850    compte joint décembre [<=>]
02/12 +   520    compte joint décembre
07/12 -   780.42 Loyer appartement     [logement]
07/12 -    60    Loyer box             [logement]
21/12 -   250    Voiture               [voiture]
10/12 -    80    courses               [courses]

## Balance

- Compte principal =    400.69
- Livret 1         =  8 000
- Livret 2         = 15 000
- Compte joint     =    110

",
    );
    let expected = GobelinFile {
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

    assert_eq!(parse_gobelin_file(&config, &file), Ok(expected));
}

#[test]
fn should_parse_not_formatted_file() {
    let config = Config {
        general: GeneralConfig { locale: Locale::FR },
        accounts: vec![
            AccountConfig {
                name: String::from("Compte principal"),
            },
            AccountConfig {
                name: String::from("Livret 1"),
            },
            AccountConfig {
                name: String::from("Livret 2"),
            },
            AccountConfig {
                name: String::from("Compte joint"),
            },
        ],
    };
    let file = String::from(
        "# December 2021

## Transactions

###  Main account

07/12 -19.99 Phone [telecom]
05/12 -29.99 Internet [telecom]
02/12 +1800.67 Salary [salary]
02/12 -500 saving 1 [<=>]
02/12 -850 Shared account décembre [<=>]

### Savings account 1
02/12 +500 saving 1 [<=>]

### Savings account 2


### Shared account

02/12 +850 Shared account décembre [<=>]
02/12 +520 Shared account décembre
07/12 -780.42 apartment rent [home]
07/12 -60 box rent [home]
21/12 -250 Car [car]
10/12 -80 Shopping [food]

## Balance

- Main account = 400.69
- Savings account 1= 8 000
- Savings account 2 =15 000
- Shared account=110

",
    );
    let expected = GobelinFile {
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
    };

    assert_eq!(parse_gobelin_file(&config, &file), Ok(expected));
}
