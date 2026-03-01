# ghostty-pane-splitter

[![CI](https://github.com/rikeda71/ghostty-pane-splitter/actions/workflows/ci.yml/badge.svg)](https://github.com/rikeda71/ghostty-pane-splitter/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/ghostty-pane-splitter.svg)](https://crates.io/crates/ghostty-pane-splitter)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Ghostty ターミナルの pane 分割を CLI コマンドで自動化するツールです。

[enigo](https://github.com/enigo-rs/enigo) によるキーボード入力シミュレーションで Ghostty の pane 分割を自動化し、macOS / Linux をサポートします。

## インストール

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

### ソースからビルド

```bash
git clone https://github.com/rikeda71/ghostty-pane-splitter.git
cd ghostty-pane-splitter
cargo install --path .
```

> **Note**: Linux では `libxdo-dev` が必要です (`sudo apt install libxdo-dev`)

## 使い方

```
ghostty-pane-splitter <LAYOUT>
```

`<LAYOUT>` にはペイン数、グリッド指定（`列x行`）、またはカスタム列レイアウト（カンマ区切りで各列の行数を指定）を渡します。

```bash
# 4ペインに分割 (2x2 グリッド)
ghostty-pane-splitter 4

# 6ペインに分割 (3x2 グリッド)
ghostty-pane-splitter 6

# 2列 x 3行 で分割
ghostty-pane-splitter 2x3

# カスタムレイアウト: 左 1 ペイン、右 3 ペイン
ghostty-pane-splitter 1,3

# カスタムレイアウト: 3列で各 2, 1, 3 行
ghostty-pane-splitter 2,1,3

# バージョン表示
ghostty-pane-splitter --version

# ヘルプ表示
ghostty-pane-splitter --help
```

### レイアウト例

| 入力    | 結果  | 説明 |
|---------|-------|------|
| `2`     | 2x1   | 2列 |
| `4`     | 2x2   | 2x2 グリッド |
| `6`     | 3x2   | 3列 x 2行 |
| `9`     | 3x3   | 3x3 グリッド |
| `2x3`   | 2x3   | 明示的なグリッド指定 |
| `1,3`   | 1+3   | 左: 1 ペイン、右: 3 ペイン |
| `2,1,3` | 2+1+3 | 3列で各 2, 1, 3 行 |

## 設定

このツールは Ghostty の設定ファイルからキーバインドを読み取ります。以下のキーバインドを Ghostty の設定ファイルに追加してください:

```
keybind = super+d=new_split:right
keybind = super+shift+d=new_split:down
keybind = super+ctrl+right_bracket=goto_split:next
keybind = super+ctrl+left_bracket=goto_split:previous
keybind = super+ctrl+shift+equal=equalize_splits
```

Ghostty 設定ファイルの場所:
- **macOS**: `~/Library/Application Support/com.mitchellh.ghostty/config`
- **Linux**: `~/.config/ghostty/config`

設定ファイルが見つからない場合や必要なキーバインドが不足している場合はエラーが表示されます。

## 動作要件

- [Ghostty](https://ghostty.org/) ターミナル
- Linux: `libxdo-dev` (`sudo apt install libxdo-dev`)

## ライセンス

[MIT](LICENSE)
