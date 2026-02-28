# ghostty-pane-splitter

Ghostty ターミナルの pane 分割を CLI コマンドで自動化する Rust ツール。

## 技術スタック

- Rust (edition 2021)
- enigo: クロスプラットフォームのキーボード入力シミュレーション
- CLI 引数パース: `std::env::args` (clap 不使用)

## ビルド・テストコマンド

```bash
cargo build              # ビルド
cargo test               # テスト実行
cargo clippy -- -D warnings  # lint
cargo fmt --check        # フォーマットチェック
cargo run -- --help      # ヘルプ表示
cargo run -- 4           # 2x2 グリッドで pane 分割
```

## ファイル構成

- `src/main.rs` - エントリーポイント
- `docs/NNN-title.md` - 設計ドキュメント（連番管理）
- `.github/workflows/ci.yml` - CI 設定

## タスク管理

- `tasks.md` で TODO を管理する
- `- [ ]` 形式の箇条書きで記載し、完了したタスクは行ごと削除する
- タスクを実装する際は、対応するタスクの削除を忘れないこと

## ドキュメント規約

- `docs/` ディレクトリに連番付きで配置: `NNN-title.md`
- Google Design Doc 形式を基本とする
