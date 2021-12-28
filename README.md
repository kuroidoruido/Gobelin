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

