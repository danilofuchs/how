# How

`which` for your executables.

A Linux CLI to discover which package manager installed a command.

Useful for updating, removing, or troubleshooting packages.

## Usage

```sh
how <command>
```

Tells you which package managers installed `<command>`.

## Installation

```sh
cargo install --git https://github.com/danilofuchs/how.git
```

## Supported package managers

- `apt`
- Linuxbrew
- `npm -g`
  <!-- - `pip` -->
  <!-- - `snap` -->
  <!-- - `flatpak` -->
- `cargo`
  <!-- - Bash aliases -->
  <!-- - Zsh aliases -->
