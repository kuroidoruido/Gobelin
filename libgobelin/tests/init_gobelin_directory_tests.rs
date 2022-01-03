use libgobelin::{init_gobelin_directory, Locale};

#[test]
fn it_should_create_all_need_files() {
    let accounts: Vec<String> = Vec::new();
    let actual = init_gobelin_directory(&accounts, None, 2021, 12).unwrap();
    assert_eq!(
        actual.into_keys().collect::<Vec<String>>(),
        vec![
            String::from("2021/12.gobelin"),
            String::from("gobelin.toml"),
        ]
    );
}

mod gobelin_toml {
    use super::*;

    #[test]
    fn it_should_create_gobelin_toml_correctly_by_default() {
        let accounts: Vec<String> = Vec::new();
        let actual = init_gobelin_directory(&accounts, None, 2021, 12).unwrap();
        assert_eq!(
            actual.get(&String::from("gobelin.toml")),
            Some(&String::from(
                "[general]
locale = \"EN\"

[accounts]

[accounts.1]
name = \"Main account\"

"
            ))
        );
    }

    #[test]
    fn it_should_create_gobelin_toml_correctly_with_only_locale() {
        let accounts: Vec<String> = Vec::new();
        let actual = init_gobelin_directory(&accounts, Some(Locale::FR), 2021, 12).unwrap();
        assert_eq!(
            actual.get(&String::from("gobelin.toml")),
            Some(&String::from(
                "[general]
locale = \"FR\"

[accounts]

[accounts.1]
name = \"Compte principal\"

"
            ))
        );
    }

    #[test]
    fn it_should_create_gobelin_toml_correctly_without_locale() {
        let accounts: Vec<String> =
            vec![String::from("Compte principal"), String::from("Livret 1")];
        let actual = init_gobelin_directory(&accounts, None, 2021, 12).unwrap();
        assert_eq!(
            actual.get(&String::from("gobelin.toml")),
            Some(&String::from(
                "[general]
locale = \"EN\"

[accounts]

[accounts.1]
name = \"Compte principal\"

[accounts.2]
name = \"Livret 1\"

"
            ))
        );
    }

    #[test]
    fn it_should_create_gobelin_toml_correctly_with_all_parameters() {
        let accounts: Vec<String> =
            vec![String::from("Compte principal"), String::from("Livret 1")];
        let actual = init_gobelin_directory(&accounts, Some(Locale::FR), 2021, 12).unwrap();
        assert_eq!(
            actual.get(&String::from("gobelin.toml")),
            Some(&String::from(
                "[general]
locale = \"FR\"

[accounts]

[accounts.1]
name = \"Compte principal\"

[accounts.2]
name = \"Livret 1\"

"
            ))
        );
    }
}

mod month_file {
    use super::*;

    #[test]
    fn it_should_create_month_file_correctly_by_default() {
        let accounts: Vec<String> = Vec::new();
        let actual = init_gobelin_directory(&accounts, None, 2021, 12).unwrap();
        assert_eq!(
            actual.get(&String::from("2021/12.gobelin")),
            Some(&String::from(
                "# December 2021

## Transactions

### Main account

01/12 + 100.01 initial amount [<=>]

## Balance

- Main account = + 100.01
"
            ))
        );
    }

    #[test]
    fn it_should_create_month_file_correctly_with_locale_only() {
        let accounts: Vec<String> = Vec::new();
        let actual = init_gobelin_directory(&accounts, Some(Locale::FR), 2021, 12).unwrap();
        assert_eq!(
            actual.get(&String::from("2021/12.gobelin")),
            Some(&String::from(
                "# Décembre 2021

## Transactions

### Compte principal

01/12 + 100.01 montant initial [<=>]

## Balance

- Compte principal = + 100.01
"
            ))
        );
    }

    #[test]
    fn it_should_create_month_file_correctly_without_locale() {
        let accounts: Vec<String> =
            vec![String::from("Compte principal"), String::from("Livret 1")];
        let actual = init_gobelin_directory(&accounts, None, 2021, 12).unwrap();
        assert_eq!(
            actual.get(&String::from("2021/12.gobelin")),
            Some(&String::from(
                "# December 2021

## Transactions

### Compte principal

01/12 + 100.01 initial amount [<=>]

### Livret 1

01/12 + 100.02 initial amount [<=>]

## Balance

- Compte principal = + 100.01
- Livret 1         = + 100.02
"
            ))
        );
    }

    #[test]
    fn it_should_create_month_file_correctly_with_all_parameters() {
        let accounts: Vec<String> =
            vec![String::from("Compte principal"), String::from("Livret 1")];
        let actual = init_gobelin_directory(&accounts, Some(Locale::FR), 2021, 12).unwrap();
        assert_eq!(
            actual.get(&String::from("2021/12.gobelin")),
            Some(&String::from(
                "# Décembre 2021

## Transactions

### Compte principal

01/12 + 100.01 montant initial [<=>]

### Livret 1

01/12 + 100.02 montant initial [<=>]

## Balance

- Compte principal = + 100.01
- Livret 1         = + 100.02
"
            ))
        );
    }
}
