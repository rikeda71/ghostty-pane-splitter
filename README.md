# ghostty-pane-splitter

[![CI](https://github.com/rikeda71/ghostty-pane-splitter/actions/workflows/ci.yml/badge.svg)](https://github.com/rikeda71/ghostty-pane-splitter/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/ghostty-pane-splitter.svg)](https://crates.io/crates/ghostty-pane-splitter)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

CLI tool to split panes on Ghostty Terminal.

Automates Ghostty's pane splitting by simulating keyboard inputs via [enigo](https://github.com/enigo-rs/enigo), enabling cross-platform support (macOS / Linux).

## Installation

### Homebrew (macOS)

```bash
brew install rikeda71/tap/ghostty-pane-splitter
```

### curl (GitHub Releases)

```bash
# macOS (Apple Silicon)
curl -fsSL https://github.com/rikeda71/ghostty-pane-splitter/releases/latest/download/ghostty-pane-splitter-aarch64-apple-darwin.tar.gz | tar xz
sudo mv ghostty-pane-splitter /usr/local/bin/

# macOS (Intel)
curl -fsSL https://github.com/rikeda71/ghostty-pane-splitter/releases/latest/download/ghostty-pane-splitter-x86_64-apple-darwin.tar.gz | tar xz
sudo mv ghostty-pane-splitter /usr/local/bin/

# Linux (x86_64)
curl -fsSL https://github.com/rikeda71/ghostty-pane-splitter/releases/latest/download/ghostty-pane-splitter-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv ghostty-pane-splitter /usr/local/bin/
```

### Cargo

```bash
cargo install ghostty-pane-splitter
```

### From source

```bash
git clone https://github.com/rikeda71/ghostty-pane-splitter.git
cd ghostty-pane-splitter
cargo install --path .
```

> **Note**: Linux では `libxdo-dev` が必要です (`sudo apt install libxdo-dev`)

## Usage

```
ghostty-pane-splitter <LAYOUT>
```

`<LAYOUT>` にはペイン数またはグリッド指定を渡します。

```bash
# 4ペインに分割 (2x2 グリッド)
ghostty-pane-splitter 4

# 6ペインに分割 (3x2 グリッド)
ghostty-pane-splitter 6

# 2列 x 3行 で分割
ghostty-pane-splitter 2x3

# バージョン表示
ghostty-pane-splitter --version

# ヘルプ表示
ghostty-pane-splitter --help
```

### Layout examples

| Input | Result | Description |
|-------|--------|-------------|
| `2`   | 2x1    | 2 columns |
| `4`   | 2x2    | 2x2 grid |
| `6`   | 3x2    | 3 cols x 2 rows |
| `9`   | 3x3    | 3x3 grid |
| `2x3` | 2x3    | Explicit grid spec |

## Configuration

This tool reads keybindings directly from your Ghostty config file. Add the following keybindings to your Ghostty config:

```
keybind = super+d=new_split:right
keybind = super+shift+d=new_split:down
keybind = super+ctrl+right_bracket=goto_split:next
keybind = super+ctrl+left_bracket=goto_split:previous
keybind = super+ctrl+shift+equal=equalize_splits
```

Ghostty config file locations:
- **macOS**: `~/Library/Application Support/com.mitchellh.ghostty/config`
- **Linux**: `~/.config/ghostty/config`

The tool will show an error if the config file is not found or required keybindings are missing.

## Requirements

- [Ghostty](https://ghostty.org/) terminal
- Linux: `libxdo-dev` (`sudo apt install libxdo-dev`)

## License

[MIT](LICENSE)
