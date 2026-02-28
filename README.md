# ghostty-pane-splitter

CLI tool to split panes on Ghostty Terminal.

Automates Ghostty's pane splitting by simulating keyboard inputs via [enigo](https://github.com/enigo-rs/enigo), enabling cross-platform support (macOS / Linux).

## Features

- Split panes by number (e.g. `4` → 2x2 grid)
- Split panes by grid spec (e.g. `2x3` → 2 cols x 3 rows)
- Cross-platform: macOS and Linux
- Configurable keybindings to match your Ghostty setup

## Requirements

- [Ghostty](https://ghostty.org/) terminal
- Rust toolchain (for building from source)
- Linux: `libxdo-dev` (`sudo apt install libxdo-dev`)

## Installation

### From source

```bash
git clone https://github.com/rikeda71/ghostty-pane-splitter.git
cd ghostty-pane-splitter
cargo install --path .
```

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

## Usage

```bash
# Split into 4 panes (2x2 grid)
ghostty-pane-splitter 4

# Split into 6 panes (3x2 grid)
ghostty-pane-splitter 6

# Split into 2 cols x 3 rows
ghostty-pane-splitter 2x3

# Show help
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

## License

[MIT](LICENSE)
