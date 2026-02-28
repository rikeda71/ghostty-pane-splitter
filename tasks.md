# Tasks

## Next

次に着手するタスク: T-17

## Backlog

### リファクタリング

- [ ] T-17: main.rs からモジュールへの分離（layout.rs, config.rs に分離し、main.rs は CLI エントリーポイントのみにする）

### キーバインド文字列のパース

- [ ] T-10: `"super+ctrl+d"` → enigo のキー操作への変換
- [ ] T-11: 修飾キー (`super`, `ctrl`, `shift`, `alt`) の対応
- [ ] T-12: キーバインドパーサーの単体テスト

### pane 分割の実行

- [ ] T-13: enigo によるキーボード入力シミュレーション実装
- [ ] T-14: 分割フロー: split_right → pane 移動 → split_down → equalize
- [ ] T-15: 操作間のスリープ挿入
- [ ] T-16: Ghostty 上での手動テスト
