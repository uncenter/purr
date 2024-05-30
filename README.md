# purr

Query data about the Catppuccin project, the [catppuccin/userstyles](https://github.com/catppuccin/userstyles) subproject, maintainers, or even initialize a new port/userstyle from a template.

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
purr <COMMAND> [-h | --help] [-V | --version]
```

### `query`

```
purr query [-c | --count] [-o | --output]
```

Query the ports.yml data source. With no arguments the names of the ports are displayed.

#### `-o` / `--output`

Both query commands can be used with the `-o`/`--output` option to change the output format to either `json` and `plain` (defaults to `json`).

#### `--count`

Count the number of ports.

#### `maintained`

```
purr query maintained [--by <NAME>] [-n | --not] [-c | --count] [-o | --output]
```

**Examples**:

- List maintained ports.

  ```
  purr query maintained
  ```

- Count the number of maintained ports.

  ```
  purr query maintained --count
  ```

- List *un*maintained ports.

  ```
  purr query maintained --not
  ```

- Count the number of *un*maintained ports.

  ```
  purr query maintained --not --count
  ```

- List ports maintained by `<username>`.

  ```
  purr query maintained --by "<username>"
  ```

- Count the number of ports maintained by `<username>`.

  ```
  purr query maintained --by "<username>" --count
  ```

- List ports _not_ maintained by `<username>`.

  ```
  purr query maintained --by "<username>" --not
  ```

- Count the number of ports _not_ maintained by `<username>`.

  ```
  purr query maintained --by "<username>" --not --count
  ```

#### `has`

```
purr query has [PROPERTIES] [-n | --not] [-c | --count] [-o | --output]
```

**Properties:**

- `--name <NAME>`
- `--category <CATEGORIES>`
- `--icon <ICON>`
- `--color <COLOR>`
- `--url <URL>`

**Examples**:

- List ports with `color` set to `mauve`.

  ```
  purr query has --color mauve
  ```

- Count the number of ports that have `color` set to `mauve`.

  ```
  purr query has --color mauve --count
  ```

- Count the number of ports that have `color` set to anything other than `mauve`.

  ```
  purr query has --color mauve --not --count
  ```

- List ports that do not have `icon` defined.

  ```
  purr query has --icon --not
  ```

#### `stars`

```
purr query stars [--for <REPOSITORY>] [--archived]
```

**Examples**:

- Get the total stars for all repositories across the organization.

  ```
  purr query stars
  ```

- Get the total stars for all non-archived repositories across the organization.

  ```
  purr query stars --archived false
  ```

- Get the total stars for only archived repositories across the organization.

  ```
  purr query stars --archived true
  ```

- Get the stars for a repository called `<repository>`.

  ```
  purr query stars --for "<repository>"
  ```

### `init`

The `init` command accepts each value (name, categories, etc.) via arguments, though if not provided a series of prompts will be displayed instead.

```
purr init [PROPERTIES]
```

**Properties**:

- `--name <NAME>`
- `--url <URL>`

### `userstyles`

#### `query`

```
purr userstyles query [-c | --count] [-o | --output]
```

Query the userstyles.yml data source. With no arguments the names of the userstyles are displayed.

##### `-o` / `--output`

Both query commands can be used with the `-o`/`--output` option to change the output format to either `json` and `plain` (defaults to `json`).

##### `--count`

Count the number of userstyles.

##### `maintained`

```
purr userstyles query maintained [--by <NAME>] [-n | --not] [-c | --count] [-o | --output]
```

**Examples**:

- List maintained userstyles.

  ```
  purr userstyles query maintained
  ```

- Count the number of maintained userstyles.

  ```
  purr userstyles query maintained --count
  ```

- List *un*maintained userstyles.

  ```
  purr userstyles query maintained --not
  ```

- Count the number of *un*maintained userstyles.

  ```
  purr userstyles query maintained --not --count
  ```

- List userstyles maintained by `<username>`.

  ```
  purr userstyles query maintained --by "<username>"
  ```

- Count the number of userstyles maintained by `<username>`.

  ```
  purr userstyles query maintained --by "<username>" --count
  ```

- List userstyles _not_ maintained by `<username>`.

  ```
  purr userstyles query maintained --by "<username>" --not
  ```

- Count the number of userstyles _not_ maintained by `<username>`.

  ```
  purr userstyles query maintained --by "<username>" --not --count
  ```

##### `has`

```
purr userstyles query has [PROPERTIES] [-n | --not] [-c | --count] [-o | --output]
```

**Properties:**

- `--name <NAME>`
- `--category <CATEGORIES>`
- `--icon <ICON>`
- `--color <COLOR>`
- `--app-link <APP_LINK>`

**Examples**:

- List userstyles with `color` set to `mauve`.

  ```
  purr userstyles query has --color mauve
  ```

- Count the number of userstyles that have `color` set to `mauve`.

  ```
  purr userstyles query has --color mauve --count
  ```

- Count the number of userstyles that have `color` set to anything other than `mauve`.

  ```
  purr userstyles query has --color mauve --not --count
  ```

- List userstyles that do not have `icon` defined.

  ```
  purr userstyles query has --icon --not
  ```

#### `init`

The `init` command accepts each value (name, categories, etc.) via arguments, though if not provided a series of prompts will be displayed instead.

```
purr userstyles init [PROPERTIES]
```

**Properties**:

- `--name <NAME>`
- `--category <CATEGORIES>`
- `--icon <ICON>`
- `--color <COLOR>`
- `--app-link <APP_LINK>`

## License

[MIT](LICENSE)
