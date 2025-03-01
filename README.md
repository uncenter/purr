# purr

An unofficial CLI for Catppuccin developers; query data about Catppuccin ports and [userstyles](https://github.com/catppuccin/userstyles), star counts of the organization as a whole or individual repositories, the [Whiskers](https://github.com/catppuccin/whiskers) port creation tool migration, or even initialize a new port/userstyle from the upstream template.

## Installation

### Cargo

```sh
cargo install catppuccin-purr
# or
cargo install --git https://github.com/uncenter/purr.git
```

### Arch

[purr](https://aur.archlinux.org/packages/purr/) is available as an AUR package.
It can be installed with an AUR helper (e.g. `paru`):

```sh
paru -S purr
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
- [`init`](#init)
- [`whiskerify`](#whiskerify)

### `query`

```
purr query [--for <PORT>] [-g | --get] [-c | --count] [--userstyles | --no-userstyles | --only-userstyles]
```

Query the ports.yml data source. With no arguments, all ports are displayed. The `--count` and userstyles-related flags work for all of the query subcommands.

<details>
<summary>Examples</summary>

- List all ports.

  ```
  purr query
  ```

- List all ports excluding userstyles.

  ```
  purr query --no-userstyles
  ```

- List all userstyles.

  ```
  purr query --only-userstyles
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
- `--upstreamed`
- `--platform <PLATFORM>`
- `--icon <ICON>`
- `--color <COLOR>`
- `--alias`
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

- List repositories Whiskers _is_ applicable for.

  ```
  purr query whiskers --is not-applicable --not
  ```

</details>

### `init`

The `init` command accepts each value (name, categories, etc.) via arguments, though if not provided a series of prompts will be displayed instead.

```
purr init <TEMPLATE> [PROPERTIES]
```

#### Templates

| Template    | Available Properties/Flags                 |
| ----------- | ------------------------------------------ |
| `port`      | `name`, `url`                              |
| `userstyle` | `name`, `category`, `icon`, `color`, `url` |

### `whiskerify`

```
purr whiskerify <PATH> [-o | --output <PATH>]
```

Whiskerify a file by replacing Catppuccin colors and names with Tera expressions. Prints the output or writes to the `--output` file path if given.

## License

[MIT](LICENSE)
