# Tasks

## CLI 引数パース

- [ ] `<LAYOUT>` 引数の解析（数値 `4` / グリッド `2x3` の判定と変換）
- [ ] 不正な引数に対するエラーメッセージ表示

## 分割アルゴリズム

- [ ] 数値 → cols x rows 変換（正方形に近い因数分解）
- [ ] グリッド指定 (`CxR`) のパース
- [ ] 単体テスト（2, 3, 4, 5, 6, 9 等のケース）

## 設定ファイル

- [ ] `~/.config/ghostty-pane-splitter/config` の読み込み
- [ ] `key = "value"` 形式の簡易パーサー実装
- [ ] 設定ファイル未存在時のエラーメッセージと設定例の表示
- [ ] 単体テスト

## キーバインド文字列のパース

- [ ] `"super+ctrl+d"` → enigo のキー操作への変換
- [ ] 修飾キー (`super`, `ctrl`, `shift`, `alt`) の対応
- [ ] 単体テスト

## pane 分割の実行

- [ ] enigo によるキーボード入力シミュレーション実装
- [ ] 分割フロー: split_right → pane 移動 → split_down → equalize
- [ ] 操作間のスリープ挿入
- [ ] Ghostty 上での手動テスト

## モジュール分割

- [ ] main.rs からモジュールへの分離（grid, config, splitter 等）
