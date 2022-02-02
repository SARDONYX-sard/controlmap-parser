# ControlMap Parser Library

[English](readme.md) | 日本語

![controlmap sample](../../sample-code.png)

## 概要

Skyrim用のcontrolmap.txtをjsonに変換するライブラリです。

controlmap.txt => json file

## 要件

- git
- Rust

## はじめに

```bash
git clone https://github.com/SARDONYX-sard/controlmap-parser;
cd controlmap-parser;

make; # sample controlmap.txt => result.json
# or
make controlmap; # controlmap.txt => stdout (画面への表示用)

# or manual
cargo run --example controlmap-parser <your controlmap.txt file path>;
```

## 既知の問題

- controlmap.txtのイベント行の間のコメントをうまく解析できません。(配列としてまとめられてしまいます)
- json => controlmap.txtの未対応

## License

[MIT](https://opensource.org/licenses/MIT)
