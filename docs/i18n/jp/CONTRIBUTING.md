# 貢献についての指針

## 目次

- [貢献についての指針](#貢献についての指針)
  - [目次](#目次)
  - [言語](#言語)
  - [Git ブランチフロー](#gitブランチフロー)
  - [Git コミットメッセージ規約](#gitコミットメッセージ規約)
    - [`<type>`](#type)
    - [`[optional scope]`](#optional-scope)
    - [`<description>`](#description)
    - [`[optional body]`](#optional-body)
    - [`[optional footer]`](#optional-footer)
    - [破壊的変更について](#破壊的変更について)

## 言語

Git のブランチ名やコミットメッセージ、GitHub のプルリクエストは、世界中の開発者が読めるように、英語で書かれていなければなりません。

## Git ブランチフロー

- このプロジェクトの開発には、[GitHub Flow](http://scottchacon.com/2011/08/31/github-flow.html) を採用しています。

- `main`ブランチにあるものはすべてデプロイ可能です。

- 何か新しいことに取り組むにはブランチを作成し、その名前にはプレフィックス(接頭辞)として`feature/` を追加します。

- ブランチ名は動詞で始まり、できるだけ簡潔でわかりやすいものにします。

```bash
# 例
feature/implement-xxx
feature/support-xxx-for-xxx
feature/fix-xxx-bugs
```

詳しくはこちらをご参照ください。
[GitHub Flow – Scott Chacon](http://scottchacon.com/2011/08/31/github-flow.html)

## Git コミットメッセージ規約

コミットフォーマットは[Conventional Commits](https://www.conventionalcommits.org/)に従います。
以下はその概要です。

```bash
# Format
<type>[optional scope]: <description>

[optional body]

[optional footer]
```

### `<type>`

1 つのコミットの目的は 1 つだけで、コミットメッセージの 1 行目の先頭に以下のようなコミットタイプを追加します。 メッセージを追加します。

|  `<type>`  | 使い方                                                                               |
| :--------: | :----------------------------------------------------------------------------------- |
|   `feat`   | 新機能                                                                               |
|   `fix`    | バグ修正                                                                             |
|   `docs`   | ドキュメントのみの変更                                                               |
|  `style`   | コードの意味に影響を与えない変更（空白、書式設定、セミコロンの欠落など）             |
| `refactor` | バグを修正せず、機能を追加しないコード変更                                           |
|   `perf`   | パフォーマンスを向上させるコード変更                                                 |
|   `test`   | 不足しているテストの追加、または既存のテストを修正                                   |
|  `build`   | ビルドシステムまたは外部の依存関係に影響する変更 (スコープの例: gulp、brocooli、npm) |
|    `ci`    | CI 構成ファイルとスクリプトの変更 (スコープの例: gulp、brocooli、npm)                |
|  `chore`   | src またはテストファイルを変更しないその他の変更                                     |
|  `revert`  | 以前のコミットをもとに戻します                                                       |

```bash
# 例
feat: allow provided config object to extend other configs


# 悪い例
feat:Allow provided config object to extend other configs # `<type>`と`<description>`の間にスペースがない

<Feat> allow provided config object to extend other configs # `<`と`>`が不要

Feat: allow provided config object to extend other configs # `<type>`が大文字

chore allow provided config object to extend other configs # `<type>`と`<description>`の間にコロン`:`がない
```

### `[optional scope]`

`[optional scope]`

```bash
feat(lang): add polish language
```

### `<description>`

- `<description>`（概要）は変更点をまとめたものです。

- コミットタイプを含めて 50 文字以内とします。

- ピリオド `.` は含まないでください。

- 小文字で始めます。

```bash
# 例
feat: allow provided config object to extend other configs


# 悪い例
FEAT: feat: allow provided config object to extend other configs # `type`が大文字

feat: Implement sign up system. Because ... # 概要が長すぎる
```

### `[optional body]`

- `[optional body]` は、そのコミットで変更された内容の説明です。

- `<description>`との間に一行空行を作ってから記述します。

- ピリオド `.`は含めないでください。

- 大文字から始めて、1 行ずつ記述します。

```bash
# 例
fix: correct minor typos in code

typos fixed


# 悪い例1: 1行目と2行目の間に空行がない
fix: correct minor typos in code
see the issue for details on the typos fixed


# 悪い例2: ピリオドが付いている
fix: correct minor typos in code

see the issue for details on the typos fixed.


# 悪い例3: 小文字で始めていない
fix: correct minor typos in code

See the issue for details on the typos fixed
```

### `[optional footer]`

- `[optional footer]`は、破壊的変更またはコード変更による issue 番号です。

- `[optional body]`との間に一行空行を作ってから記述します。

- ピリオド `.`は含めないでください。

- 大文字から始めて、1 行ずつ記述します。

```bash
# 例
fix: prevent racing of requests

Introduce a request id and a reference to latest request. Dismiss
incoming responses other than from latest request.

Remove timeouts which were used to mitigate the racing issue but are
obsolete now.

Reviewed-by: Z
Refs: #123


# 例2
feat: allow provided config object to extend other configs

BREAKING CHANGE: `extends` key in config file is now used for extending other config files
```

### 破壊的変更について

- 破壊的変更(BreakingChange)の場合は、以下のようなことができます。

```bash
# 破壊的な変化に注意を引くため、`<type>`に`!`をつける
feat!: send an email to the customer when a product is shipped


# スコープを持ってメッセージをコミットし、`!`を使って破壊的変更に注意を引かせる
feat(api)!: send an email to the customer when a product is shipped


# `!`と`BREAKINGCHANGE`フッターの両方でメッセージをコミットします
chore!: drop support for Node 6

BREAKING CHANGE: use JavaScript features not available in Node 6.
```
