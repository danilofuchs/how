# How

`which` for your executables.

A Linux CLI to discover which package manager installed a command.

Useful for updating, removing, or troubleshooting packages.

## Usage

```sh
how <command>
```

Tells you which package manager installed `<command>`.

If the command is not found in your path, `how` will return an error code and the message `Package manager not found`.

## Supported package managers

- `apt`
- Linuxbrew
- `npm -g`
- `pip`
- `snap`
- `flatpak`
- `cargo`
- Bash aliases
- Zsh aliases

## Copilot instructions

Written in Rust, using the Clay library for CLI parsing and Nom for parsing strings.

Given an executable, `how` will search for it in the PATH environment variable.

If not found, it will return an error code and the message `Command not found`.

If found, it will search for the package manager that installed it. A command may have
been installed by multiple package managers, and `how` should return all of them.

The first approach should be running the package manager command that checks if a package is installed, for example `apt list --installed` for `apt`, or `npm list -g --depth=0` for `npm -g`.
