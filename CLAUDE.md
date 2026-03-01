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
- 各タスクには `T-N` のタスク番号を付与する
- `## Next` セクションに次に着手するタスク番号を記載する
- ユーザーはタスク番号で着手指示できる（例: 「T-3 をやって」）
- 新規タスク追加時は既存の最大番号 + 1 を割り当てる
- **対応の終了時に必ず `tasks.md` を見直すこと**:
  - 完了したタスクは行ごと削除する
  - Next セクションを次に着手すべきタスクに更新する
  - 対応中に新たに出てきたタスクがあれば追記する

## コーディング規約

- コード内のコメントはすべて英語で記述する

## テスト規約

- 可能な限り parameterized test（テストケースを配列/タプルで定義しループで実行）を使う
- タスク完了時に以下の品質チェックをすべて実行し、通過を確認すること:
  - `cargo fmt --check`
  - `cargo test`
  - `cargo clippy -- -D warnings`

## ドキュメント規約

- `docs/` ディレクトリに連番付きで配置: `NNN-title.md`
- Google Design Doc 形式を基本とする
