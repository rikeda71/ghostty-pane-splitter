# Tasks

## Next

T-1

## Backlog

- T-1: カスタム列レイアウト機能の実装（設計: docs/003-custom-column-layout.md）
  - T-1a: `Layout` 構造体を `columns: Vec<u32>` に変更し、カンマ区切りパーサー追加、既存パーサー・テスト更新
  - T-1b: `split.rs` の分割アルゴリズムを列ごとに可変行数に対応 + 処理終了後に左上 pane にフォーカスを戻す
  - T-1c: `main.rs` の出力メッセージ・ヘルプテキスト更新

## Done

- README.md 拡張: バッジ追加、インストール方法記載、Usage 充実、英語/日本語分離 (#13)
