# ghostty-pane-splitter Implementation Design

## Status

Draft

## Background

Ghostty ターミナルには pane の分割機能が搭載されているが、GUI 操作（キーバインド）でしか分割できないため、複数 pane を一括で作成するのが煩雑。先行研究として [ghostty-layout](https://github.com/nicholascw/ghostty-layout) が存在するが、Swift で書かれており macOS の Accessibility API に依存しているためクロスプラットフォーム対応ができない。

本ツールは enigo クレートを使い、キーボード入力のシミュレーションにより Ghostty の pane 分割を自動化する。これにより macOS / Linux のクロスプラットフォーム対応を実現する。

## Goals

- 数値指定（例: `4`）またはグリッド指定（例: `2x3`）で pane を分割
- macOS と Linux に対応
- 最小限の依存でシンプルな CLI ツールを提供
- ユーザーが自分の Ghostty keybind 設定に合わせてキーバインドを設定できる

## Non-Goals

- Windows 対応（Ghostty が Windows 未サポートのため）
- tmux / Zellij など他のターミナルマルチプレクサとの連携
- pane 内でのコマンド自動実行
- GUI の提供

## Design

### CLI インターフェース

```
ghostty-pane-splitter <LAYOUT>
```

- 数値指定: `ghostty-pane-splitter 4` → 2x2 グリッド
- グリッド指定: `ghostty-pane-splitter 2x3` → 2 cols x 3 rows

引数パースは `std::env::args` で自前実装する。依存を最小化するため clap は使用しない。

### 分割アルゴリズム

数値指定の場合、正方形に近い因数分解で cols x rows を決定する。

| 入力 | cols | rows | 説明 |
|------|------|------|------|
| `2`  | 2    | 1    | 横2分割 |
| `3`  | 3    | 1    | 横3分割 |
| `4`  | 2    | 2    | 2x2 |
| `6`  | 3    | 2    | 3x2 |
| `9`  | 3    | 3    | 3x3 |
| `5`  | 5    | 1    | 素数は横一列 |
| `2x3`| 2    | 3    | グリッド指定 |

アルゴリズム:
1. 入力が `CxR` 形式の場合、そのまま cols = C, rows = R として使用
2. 入力が数値 N の場合、N の因数のうち √N に最も近いペアを選択
   - cols = ceil(√N) 方向から探索、rows = N / cols
   - 割り切れない場合は cols を増やす（素数の場合 cols = N, rows = 1）

### キーバインド戦略

Ghostty のキーバインドはユーザーごとに異なるため、ハードコードではなく設定ファイルでユーザーが自分のキーバインドを指定する方式を採用する。

**設定ファイルパス**: `~/.config/ghostty-pane-splitter/config`

```toml
split_right = "super+d"
split_down = "super+shift+d"
goto_next = "super+ctrl+right_bracket"
goto_previous = "super+ctrl+left_bracket"
equalize = "super+ctrl+shift+equal"
```

- 設定ファイルがない場合はエラーメッセージと設定例を表示し、ユーザーに設定を促す
- TOML ライクな `key = "value"` 形式の簡易パーサーを自前実装（依存最小化のため外部 TOML パーサーは使用しない）
- 修飾キー: `super`, `ctrl`, `shift`, `alt` を `+` で連結

### pane 分割の実行フロー

1. 設定ファイルを読み込み、キーバインドを取得
2. CLI 引数を解析し、cols x rows を算出
3. enigo でキーボード入力をシミュレーション:
   - 1行目: `split_right` を (cols - 1) 回実行
   - 各列の先頭 pane に移動し、`split_down` を (rows - 1) 回実行
   - 最後に `equalize` で pane サイズを均等化
4. 各操作間にスリープを挟み、Ghostty が処理を完了するのを待つ

### プラットフォーム抽象化

enigo クレートがキーボード入力シミュレーションのプラットフォーム差異を吸収する:
- macOS: Core Graphics Event API
- Linux: X11 (libxdo)

## Alternatives Considered

### CLI パーサー: clap vs std::env::args

clap は高機能だが、本ツールのインターフェースは `<LAYOUT>` 引数のみとシンプルなので、`std::env::args` で十分。依存を最小化するため自前実装を選択。

### キー入力: Accessibility API vs enigo

Accessibility API (macOS) は直接ウィンドウを操作でき信頼性が高いが、プラットフォーム固有の実装が必要。enigo はクロスプラットフォームでキーボード入力をシミュレーションでき、Ghostty のキーバインド経由で操作するため十分に動作する。

### キーバインド: ハードコード vs 設定ファイル

Ghostty のデフォルトキーバインドをハードコードする案もあったが、ユーザーがカスタマイズしている場合に対応できない。設定ファイル方式であれば、どのようなキーバインド設定でも対応可能。

## Testing Strategy

- **grid 計算 (分割アルゴリズム)**: 単体テストで網羅的にカバー
- **CLI 引数パース**: 単体テストでカバー
- **設定ファイルパーサー**: 単体テストでカバー
- **pane 分割の実行 (enigo)**: 実際の Ghostty 上での手動テスト中心。CI ではモック化が困難なためスキップ
