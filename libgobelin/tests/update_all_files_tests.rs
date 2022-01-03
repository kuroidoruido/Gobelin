use libgobelin::{
    update_all_files, AccountConfig, Config, EmptyTransactionConfig, GeneralConfig, Locale,
};
use std::collections::BTreeMap;

#[test]
fn it_should_compute_all_balances() {
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
    let files = BTreeMap::from([
        (
            String::from("2021/10.gobelin"),
            String::from(
                "# October 2021

## Transactions

### Main account

01/10 +100 initial

### Savings account

01/10 +1000 initial

",
            ),
        ),
        (
            String::from("2021/11.gobelin"),
            String::from(
                "# November 2021

## Transactions

### Main account

01/11 +1800 salary [salary]
02/11 -200 savings [<=>]
03/11 -29.99 internet [telecom]
03/11 -19.99 mobile [telecom]

### Savings account

02/11 +200 savings [<=>]

## Balance

- Main account = 0
- Savings account = 0

",
            ),
        ),
        (
            String::from("2021/12.gobelin"),
            String::from(
                "# December 2021

## Transactions

### Main account

01/12 +1800 salary [salary]
02/12 -200 savings [<=>]
03/12 -29.99 internet [telecom]
03/12 -19.99 mobile [telecom]

### Savings account

02/12 +200 savings [<=>]

## Balance

- Main account = 0
- Savings account = 0

",
            ),
        ),
        (
            String::from("2022/01.gobelin"),
            String::from(
                "# January 2022

## Transactions

### Main account

01/01 +1800 salary [salary]
02/01 -200 savings [<=>]
03/01 -29.99 internet [telecom]
03/01 -19.99 mobile [telecom]

### Savings account

02/01 +200 savings [<=>]

## Balance

- Main account = 0
- Savings account = 0

",
            ),
        ),
    ]);
    let expected = BTreeMap::from([
        (
            String::from("2021/10.gobelin"),
            String::from(
                "# October 2021

## Transactions

### Main account

01/10 +   100    initial

### Savings account

01/10 + 1 000    initial

## Balance

- Main account    =   100
- Savings account = 1 000
",
            ),
        ),
        (
            String::from("2021/11.gobelin"),
            String::from(
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
            ),
        ),
        (
            String::from("2021/12.gobelin"),
            String::from(
                "# December 2021

## Transactions

### Main account

01/12 + 1 800    salary   [salary]
02/12 -   200    savings  [<=>]
03/12 -    29.99 internet [telecom]
03/12 -    19.99 mobile   [telecom]

### Savings account

02/12 +   200    savings  [<=>]

## Balance

- Main account    = 3 200.04
- Savings account = 1 400
",
            ),
        ),
        (
            String::from("2022/01.gobelin"),
            String::from(
                "# January 2022

## Transactions

### Main account

01/01 + 1 800    salary   [salary]
02/01 -   200    savings  [<=>]
03/01 -    29.99 internet [telecom]
03/01 -    19.99 mobile   [telecom]

### Savings account

02/01 +   200    savings  [<=>]

## Balance

- Main account    = 4 750.06
- Savings account = 1 600
",
            ),
        ),
    ]);

    assert_eq!(update_all_files(&config, &files), Ok(expected));
}
