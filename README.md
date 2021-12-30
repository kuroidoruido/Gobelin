# Gobelin
Gobelin is a text based personal accounting system.

## Architecture

- gobelin: CLI interface for libgobelin
- libgobelin: tool lib to format, edit, update, (re)compute everthing over all files

## Gobelin usage

### global parameters

These parameters should be passed before any command.

#### root

This parameter will allow you to change project root.

Default: .

Example:

```sh
gobelin ./sample
```

### init

This command will initialize directory structure with base files (gobelin.toml, current month file).

Example:

```sh
gobelin init
```

#### accounts

This will let you specify which accounts you wants. You can add more manually later but this will init all at initialization time for you.

Default: "Main account" for EN locale, "Compte principal" for FR locale

Example:

```sh
gobelin init --account "Main account" "Savings account 1" "Savings account 2" "Shared account"
gobelin init -a "Main account" "Savings account 1" "Savings account 2" "Shared account"
gobelin init --account "Main account" --account "Savings account 1" --account "Savings account 2" --account "Shared account"
gobelin init -a "Main account" -a "Savings account 1" -a "Savings account 2" -a "Shared account"
```
#### locale

This will let you specify which locale you wants. You can set it later manually.

Default: EN

Example:

```sh
gobelin init --locale FR
```

#### verbose

This will give some indication about what Gobelin will do.

Example:

```sh
gobelin init -v
gobelin init --verbose
```


### fmt

This command will format given files.

Note: if a file is already formatted, the file will not be rewritten.

Example:

```sh
gobelin fmt 2021/12.gobelin
gobelin fmt 2021/11.gobelin 2021/12.gobelin
```

#### verbose

This will give some indication about what Gobelin will do.

Example:

```sh
gobelin fmt 2021/12.gobelin -v
gobelin fmt 2021/11.gobelin --verbose
```

