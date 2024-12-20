# How

`which` for your executables.

A Linux/MacOS CLI to discover which package manager installed a command.

Useful when updating, removing, or troubleshooting packages.

## Usage

```sh
how <command>
```

Tells you which package managers installed `<command>`.

### Example

```sh
$ how git
> git installed by apt
```

```sh
$ how starship
> starship installed by cargo
> starship installed by brew
```

## Installation

You must have Rust installed. If you don't, you can install it with [rustup](https://rustup.rs/).

Then, run:

```sh
cargo install --git https://github.com/danilofuchs/how.git
```

## Supported package managers

- `apt`
- Homebrew / Linuxbrew
- `npm -g`
- `pip` and `pip3`
- `snap`
  <!-- - `flatpak` -->
- `cargo`
  <!-- - Bash aliases -->
  <!-- - Zsh aliases -->
