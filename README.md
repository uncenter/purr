# purr

Query data about the Catppuccin project, the [catppuccin/userstyles](https://github.com/catppuccin/userstyles) subproject, maintainers, or even initialize a new port/userstyle from a template.

## Installation

### Cargo

```sh
cargo install catppuccin-purr
# or
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

### Commands

- [`query`](#query)
  - [`maintained`](#maintained)
  - [`has`](#has)
  - [`stars`](#stars)
  - [`whiskers`](#whiskers)
- [`whiskerify`](#whiskerify)
- [`init`](#init)
- [`userstyles`](#userstyles)
  - [`query`](#query-1)
    - [`maintained`](#maintained-1)
    - [`has`](#has-1)
  - [`init`](#init-1)

### `query`

```
purr query [--for <PORT>] [-g | --get] [-c | --count]
```

Query the ports.yml data source. With no arguments, all ports are displayed.

<details>
<summary>Examples</summary>

- List all ports.

  ```
  purr query
  ```

- Count the number of ports.

  ```
  purr query --count
  ```


- List the names of all ports.

  ```
  purr query --get name
  ```

- List the current maintainers of the `nvim` port.

  ```
  purr query --for nvim --get current-maintainers
  ```

</details>

#### `maintained`

```
purr query maintained [--by <NAME>] [-n | --not] [-c | --count]
```

<details>
<summary>Examples</summary>

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

</details>

#### `has`

```
purr query has [PROPERTIES] [-n | --not] [-c | --count]
```

**Properties:**

- `--name <NAME>`
- `--category <CATEGORIES>`
- `--icon <ICON>`
- `--color <COLOR>`
- `--url <URL>`

<details>
<summary>Examples</summary>

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

- List ports with categories of `application_launcher` and `system`.

  ```
  purr query has --category application_launcher,system
  ```

</details>

#### `stars`

```
purr query stars [--for <REPOSITORY>] [--archived]
```

<details>
<summary>Examples</summary>

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

</details>

#### `whiskers`

```
purr query whiskers [--is <STATE>] [-n | --not] [-c | --count | -p | --percentage]
```

<details>
<summary>Examples</summary>

- Get the overall statistics of the Whiskerification process.
  
  ```
  purr query whiskers
  ```

- List Whiskerified repositories.

  ```
  purr query whiskers --is true
  ```

- List non-Whiskerified repositories.

  ```
  purr query whiskers --is false
  ```

- List repositories Whiskers is not applicable for.

  ```
  purr query whiskers --is not-applicable
  ```

- List repositories Whiskers *is* applicable for.

  ```
  purr query whiskers --is not-applicable --not
  ```

</details>

### `whiskerify`

```
purr whiskerify <PATH> [--dry-run]
```

Whiskerify a specific file by replacing Catppuccin colors with Tera expressions. Writes back to the original file unless `--dry-run` is passed.

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
purr userstyles query [--for <USERSTYLE>] [-g | --get] [-c | --count]
```

Query the userstyles.yml data source. With no arguments, all userstyles are displayed.

<details>
<summary>Examples</summary>

- List all userstyles.

  ```
  purr userstyles query
  ```

- Count the number of userstyles.

  ```
  purr userstyles query --count
  ```


- List the names of all userstyles.

  ```
  purr userstyles query --get name
  ```

- List the current maintainers of the `youtube` userstyle.

  ```
  purr userstyles query --for youtube --get current-maintainers 
  ```

</details>

##### `maintained`

```
purr userstyles query maintained [--by <NAME>] [-n | --not] [-c | --count]
```

<details>
<summary>Examples</summary>

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

</details>

##### `has`

```
purr userstyles query has [PROPERTIES] [-n | --not] [-c | --count]
```

**Properties:**

- `--name <NAME>`
- `--category <CATEGORIES>`
- `--icon <ICON>`
- `--color <COLOR>`
- `--app-link <APP_LINK>`

<details>
<summary>Examples</summary>

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

</details>

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
