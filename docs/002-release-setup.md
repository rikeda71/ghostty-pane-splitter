# Release Setup Design

## Status

Draft

## Background

ghostty-pane-splitter は現在ソースからのビルドでしかインストールできない。ユーザーが手軽にインストールできるよう、複数のインストール方法を提供する必要がある。

## Goals

- `curl` でバイナリをダウンロードしてインストールできる
- `cargo install ghostty-pane-splitter` でインストールできる
- `brew install` でインストールできる
- タグプッシュをトリガーにリリースプロセスを自動化する

## Non-Goals

- Windows 向けバイナリの提供（Ghostty が Windows 未サポートのため）
- Debian/RPM パッケージの提供
- 自動バージョンバンプ

## Design

### 1. curl インストール（GitHub Releases）

GitHub Actions の release workflow でタグプッシュ時にマルチプラットフォームのバイナリをビルドし、GitHub Releases にアップロードする。

#### ビルドマトリクス

| ターゲット | OS | アーキテクチャ | ランナー |
|-----------|-----|--------------|---------|
| `aarch64-apple-darwin` | macOS | aarch64 | `macos-latest` |
| `x86_64-apple-darwin` | macOS | x86_64 | `macos-13` |
| `x86_64-unknown-linux-gnu` | Linux | x86_64 | `ubuntu-latest` |

#### アーティファクト命名規則

```
ghostty-pane-splitter-{target}.tar.gz
```

例:
- `ghostty-pane-splitter-aarch64-apple-darwin.tar.gz`
- `ghostty-pane-splitter-x86_64-apple-darwin.tar.gz`
- `ghostty-pane-splitter-x86_64-unknown-linux-gnu.tar.gz`

#### ユーザーのインストール手順

```bash
# macOS (Apple Silicon)
curl -L https://github.com/rikeda71/ghostty-pane-splitter/releases/latest/download/ghostty-pane-splitter-aarch64-apple-darwin.tar.gz | tar xz
sudo mv ghostty-pane-splitter /usr/local/bin/

# macOS (Intel)
curl -L https://github.com/rikeda71/ghostty-pane-splitter/releases/latest/download/ghostty-pane-splitter-x86_64-apple-darwin.tar.gz | tar xz
sudo mv ghostty-pane-splitter /usr/local/bin/

# Linux (x86_64)
curl -L https://github.com/rikeda71/ghostty-pane-splitter/releases/latest/download/ghostty-pane-splitter-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv ghostty-pane-splitter /usr/local/bin/
```

### 2. cargo install（crates.io）

Cargo.toml に crates.io 公開用のメタデータを追加し、release workflow 内で `cargo publish` を実行する。

#### Cargo.toml の変更点

```toml
[package]
name = "ghostty-pane-splitter"
version = "0.1.0"
edition = "2021"
description = "CLI tool to split panes on Ghostty Terminal"
license = "MIT"
repository = "https://github.com/rikeda71/ghostty-pane-splitter"
keywords = ["ghostty", "terminal", "pane", "split"]
categories = ["command-line-utilities"]
```

追加するフィールド:
- `repository`: GitHub リポジトリの URL
- `keywords`: crates.io での検索用キーワード（最大5つ）
- `categories`: crates.io のカテゴリ

#### ユーザーのインストール手順

```bash
cargo install ghostty-pane-splitter
```

### 3. brew install（Homebrew tap）

別リポジトリ `rikeda71/homebrew-tap` を作成し、ソースからビルドする formula を配置する。release workflow で formula のバージョンを自動更新する。

#### Homebrew formula テンプレート

`homebrew-tap` リポジトリに `Formula/ghostty-pane-splitter.rb` を配置する:

```ruby
class GhosttyPaneSplitter < Formula
  desc "CLI tool to split panes on Ghostty Terminal"
  homepage "https://github.com/rikeda71/ghostty-pane-splitter"
  url "https://github.com/rikeda71/ghostty-pane-splitter/archive/refs/tags/v#{version}.tar.gz"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "Usage:", shell_output("#{bin}/ghostty-pane-splitter --help")
  end
end
```

#### formula の自動更新

release workflow 内で `homebrew-tap` リポジトリの formula ファイルを更新する:
1. `homebrew-tap` リポジトリをチェックアウト
2. formula 内の `url` と `sha256` をリリースタグに合わせて更新
3. コミット＆プッシュ

#### ユーザーのインストール手順

```bash
brew tap rikeda71/tap
brew install ghostty-pane-splitter
```

### Release Workflow 概要

#### トリガー

```yaml
on:
  push:
    tags:
      - "v*"
```

`v` プレフィックス付きタグ（例: `v0.1.0`）のプッシュで起動する。

#### ジョブ構成

```
┌─────────────────────────────────────────────┐
│  build (matrix: 3 targets)                  │
│  - checkout                                 │
│  - Rust ツールチェインセットアップ              │
│  - cargo build --release --target <target>  │
│  - tar.gz にアーカイブ                        │
│  - アーティファクトをアップロード                │
└──────────────────┬──────────────────────────┘
                   │ needs: build
┌──────────────────▼──────────────────────────┐
│  release                                    │
│  - アーティファクトをダウンロード                │
│  - GitHub Release を作成                     │
│  - tar.gz をリリースにアップロード              │
└──────────────────┬──────────────────────────┘
                   │ needs: release
┌──────────────────▼──────────────────────────┐
│  publish-crate                              │
│  - cargo publish                            │
└──────────────────┬──────────────────────────┘
                   │ needs: release
┌──────────────────▼──────────────────────────┐
│  update-homebrew                            │
│  - homebrew-tap リポジトリをチェックアウト      │
│  - formula を更新                            │
│  - コミット＆プッシュ                          │
└─────────────────────────────────────────────┘
```

## 必要な準備

### GitHub Secrets

| Secret 名 | 用途 |
|-----------|------|
| `CARGO_REGISTRY_TOKEN` | crates.io への publish 用トークン |
| `HOMEBREW_TAP_TOKEN` | `homebrew-tap` リポジトリへの push 用 Personal Access Token |

### リポジトリ作成

- `rikeda71/homebrew-tap` リポジトリを作成する

### リリース手順

1. `Cargo.toml` の `version` を更新
2. コミット＆プッシュ
3. タグを作成してプッシュ: `git tag v0.1.0 && git push origin v0.1.0`
4. release workflow が自動的に以下を実行:
   - マルチプラットフォームバイナリのビルド
   - GitHub Release の作成
   - crates.io への publish
   - Homebrew formula の更新

## Alternatives Considered

### バイナリ配布: GitHub Releases vs 独自サーバー

独自サーバーでバイナリをホストする案もあるが、運用コストが高い。GitHub Releases は無料で帯域制限も緩く、CI との統合も容易なため選択した。

### Homebrew: ボトル (pre-built binary) vs ソースビルド

pre-built binary をボトルとして配布する方式はインストールが高速だが、ビルド・配布の仕組みが複雑になる。本ツールはビルド時間が短いため、ソースビルド方式で十分と判断した。

### crates.io publish: 手動 vs CI 自動化

手動で `cargo publish` を実行する案もあるが、リリース忘れやバージョン不一致のリスクがある。CI で自動化することで確実にリリースできる。
