# NitroPkg (npk)

> The official package manager for **NitroOS**.

NitroPkg is a fast, lightweight, and user-friendly package manager designed for NitroOS. It provides a simple command-line interface for installing, removing, updating, and managing software packages.

## Features

- Fast package management
- Simple and intuitive commands
- Search packages
- System updates and upgrades
- Cache cleanup
- Written in Rust

## Installation

NitroPkg comes pre-installed with NitroOS.

Verify the installation:

```bash
npk --version
```

## Usage

```bash
npk <COMMAND>
```

## Commands

| Command | Alias | Description |
|---------|:-----:|-------------|
| `install` | `i` | Install one or more packages |
| `remove` | `rm` | Remove installed packages |
| `search` | `s` | Search for packages |
| `update` | — | Synchronize package databases |
| `upgrade` | — | Upgrade all installed packages |
| `clean` | — | Remove unused package cache |
| `help` | — | Display help information |

## Examples

Install a package

```bash
npk install firefox
```

or

```bash
npk i firefox
```

Remove a package

```bash
npk remove firefox
```

or

```bash
npk rm firefox
```

Search for a package

```bash
npk search neovim
```

or

```bash
npk s neovim
```

Update package databases

```bash
npk update
```

Upgrade the system

```bash
npk upgrade
```

Clean package cache

```bash
npk clean
```

Display help

```bash
npk --help
```

Display version

```bash
npk --version
```
