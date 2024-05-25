# purr

> "**P**ower **U**serstyles **R**eal **R**ealer" - [@isabelroses](https://github.com/isabelroses)

Utility commands for managing [catppuccin/userstyles](https://github.com/catppuccin/userstyles). Query data about the repository, the userstyles, and the maintainers, or initialize a new userstyle from the template.

## Installation

### Cargo

```sh
cargo install --git https://github.com/uncenter/purr.git
```

### Nix

```
nix run github:uncenter/purr
```

## Usage

```
Usage: purr <COMMAND>

Commands:
  query
  init
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### `query`

Both of the query commands can be used with the `-o`/`--output` option to change the output format (`json` or `plain`).

```
Usage: purr query [OPTIONS] [COMMAND]

Commands:
  maintained
  has
  help        Print this message or the help of the given subcommand(s)

Options:
  -c, --count
  -o, --output <FORMAT>  [default: json] [possible values: json, plain]
  -h, --help             Print help
```

**Examples**:

- List the userstyles.

  ```
  purr query
  ```

- Count the number of userstyles.

  ```
  purr query --count
  ```

#### `maintained`

```
Usage: purr query maintained [OPTIONS]

Options:
      --by <NAME>
  -n, --not
  -c, --count
  -o, --output <FORMAT>  [default: json] [possible values: json, plain]
  -h, --help             Print help
```

**Examples**:

- List maintained userstyles.

  ```
  purr query maintained
  ```

- Count the number of maintained userstyles.

  ```
  purr query maintained --count
  ```

- List *un*maintained userstyles.

  ```
  purr query maintained --not
  ```

- Count the number of *un*maintained userstyles.

  ```
  purr query maintained --not --count
  ```

- List userstyles maintained by `<username>`.

  ```
  purr query maintained --by "<username>"
  ```

- Count the number of userstyles maintained by `<username>`.

  ```
  purr query maintained --by "<username>" --count
  ```

- List userstyles _not_ maintained by `<username>`.

  ```
  purr query maintained --by "<username>" --not
  ```

- Count the number of userstyles _not_ maintained by `<username>`.

  ```
  purr query maintained --by "<username>" --not --count
  ```

#### `has`

```
Usage: purr query has [OPTIONS]

Options:
      --name <NAME>
      --category <CATEGORIES>
      --icon <ICON>
      --color <COLOR>
      --app-link <APP_LINK>
  -c, --count
  -l, --list
  -n, --not
  -o, --output <FORMAT>        [default: json] [possible values: json, plain]
  -h, --help                   Print help
```

**Examples**:

- Check if userstyles exist with `color` set to `mauve`.

  ```
  purr query has --color mauve
  ```

- List userstyles that have `color` set to `mauve`.

  ```
  purr query has --color mauve --list
  ```

- Count the number of userstyles that have `color` set to `mauve`.

  ```
  purr query has --color mauve --count
  ```

- Count the number of userstyles that have `color` set to anything other than `mauve`.

  ```
  purr query has --color mauve --not --count
  ```

- List userstyles that do not have `icon` defined.

  ```
  purr query has --icon --not --list
  ```

### `init`

The `init` command accepts each value (name, categories, etc.) via arguments, though if not provided a series of prompts will be displayed instead.

```
Usage: purr init [OPTIONS]

Options:
      --name <NAME>
      --category <CATEGORIES>
      --icon <ICON>
      --color <COLOR>
      --app-link <APP_LINK>
  -h, --help                   Print help
```

## Roadmap

- [ ] Query GitHub stars
- [ ] Query contributors
- [ ] Query maintainers
- [ ] Query PRs (per userstyle, dated/at what time, by who)

## License

[MIT](LICENSE)
