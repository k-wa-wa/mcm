
# Markdown Contents Manager (mcm)

Markdownのコンテンツを管理するツール。

## Install

```shell
cargo install mcm
```

## Usage

```shell
# ZennのコンテンツをHugo(theme: Robust)用のコンテンツに変換する
mcm export -s=zenn -t=hugo-robust

# それぞれのディレクトリを指定する
mcm export -s=zenn -t=hugo-robust --source_dir=../zenn-contents --target_dir=.
```

## Features
- [x] Zenn用のディレクトリ構成からHugo(theme: Robust)用のディレクトリ構成、Markdown構成に変換する
    - [ ] 画像
