# Gobelin
Gobelin is a text based personal accounting system.

## Architecture

- gobelin: CLI interface for libgobelin
- libgobelin: tool lib to format, edit, update, (re)compute everthing over all files
- gobelin-vscode: VSCode extension (available for [VSCode](https://marketplace.visualstudio.com/items?itemName=kuroidoruido.gobelin-vscode) and [VSCodium and other compatible editor](https://open-vsx.org/extension/kuroidoruido/gobelin-vscode))

## Gobelin usage

If no command is specified, update command with default parameters is used.

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

#### vscode

This will add some configuration to have a better VSCode integration.

Example:

```sh
gobelin init --vscode
```

#### verbose

This will give some indication about what Gobelin will do.

Example:

```sh
gobelin init -v
gobelin init --verbose
```

### add month

This command will create specified month.

Note 1: the previous month should exists.
Note 2: if the spefified month already exists, nothing will be done.

Example:

```sh
gobelin add month 2021 12
```

#### verbose

This will give some indication about what Gobelin will do.

Example:

```sh
gobelin add month 2021 12 -v
gobelin add month 2021 12 --verbose
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

### update

This command will recompute all balances.

Note: if a file is up to date, it will not be rewritten or edited.

Example:

```sh
gobelin update
```

#### verbose

This will give some indication about what Gobelin will do.

Example:

```sh
gobelin update -v
gobelin update --verbose
```

