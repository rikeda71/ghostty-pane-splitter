# Tasks

## Next

次に着手するタスク: T-6, T-7, T-8, T-9

## Backlog

### 設定ファイル

- [ ] T-6: Ghostty 設定ファイルの読み込み（OS ごとのパス解決）
- [ ] T-7: `keybind = <trigger>=<action>` 形式のパーサー実装
- [ ] T-8: 必要なアクションのキーバインドが見つからない場合のエラー表示
- [ ] T-9: 設定ファイルパーサーの単体テスト

### キーバインド文字列のパース

- [ ] T-10: `"super+ctrl+d"` → enigo のキー操作への変換
- [ ] T-11: 修飾キー (`super`, `ctrl`, `shift`, `alt`) の対応
- [ ] T-12: キーバインドパーサーの単体テスト

### pane 分割の実行

- [ ] T-13: enigo によるキーボード入力シミュレーション実装
- [ ] T-14: 分割フロー: split_right → pane 移動 → split_down → equalize
- [ ] T-15: 操作間のスリープ挿入
- [ ] T-16: Ghostty 上での手動テスト

### モジュール分割

- [ ] T-17: main.rs からモジュールへの分離（grid, config, keybind, splitter 等）
