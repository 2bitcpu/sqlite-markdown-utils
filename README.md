# SQLite Markdown Utils

Markdown記法で書かれたテキストからMarkdown記法を削除するSQLite関数を追加するSQLite拡張ライブラリです。
`lindera-sqlite` のような形態解析器に渡す前に、Markdown記法を取り除いてテキストの本体だけを抽出するために使います。
そのため連続した空白(全角空白、改行、タブ、含む)を正規化(１つの空白に変換)します。

- `md_strip(value)`
  - Markdownの書式を取り除いて、空白を正規化したテキストを返します。
- `md_strip_n(value, n)`
  - 変換後の文字列から先頭から `n` 文字を返します(description用に作りました)。

## 使い方

### ビルド

```bash
cargo build --release
```

### SQLiteへのロード

生成された拡張ライブラリをSQLiteに読み込みます。

```sql
.load ./target/release/libmd_utils
```

> 環境によって拡張子は自動で判別されます。

### 使用例

```sql
SELECT md_strip('# Hello\n**World**');
-- => Hello World

SELECT md_strip_n('**Hello** world', 5);
-- => Hello
```
