# CCFinderSW Parser

CCFinderSWの出力ファイルをJSON形式に変換するツールです。

## 機能

- CCFinderSWの出力ファイルを読み込み
- JSON形式に変換
- 標準出力またはファイルに出力

## インストール

```bash
git clone https://github.com/your-username/ccfindersw-parser.git
cd ccfindersw-parser
cargo build --release
```

## 使用方法

基本的な使用方法：

```bash
ccfindersw-parser -i <入力ファイル>
```

オプション：

- `-i, --input <FILE>`: CCFinderSWの出力ファイルのパス（必須）
- `-o, --output <FILE>`: JSON出力ファイルのパス（オプション）
- `-p, --pretty`: JSONを整形して出力するかどうか（オプション）

使用例：

```bash
# 基本的な使用方法（標準出力に出力）
ccfindersw-parser -i input.txt

# 出力ファイルを指定
ccfindersw-parser -i input.txt -o output.json

# 整形されたJSONとして出力
ccfindersw-parser -i input.txt -p
```

## 出力形式

出力されるJSONの形式は以下の通りです：

```json
{
  "file_data": [
    {
      "file_id": 1,
      "file_path": "path/to/file",
      "loc": 100,
      "token_count": 200
    }
  ],
  "clone_sets": [
    {
      "clone_id": 1,
      "fragments": [
        {
          "file_id": 1,
          "start_line": 10,
          "start_col": 1,
          "end_line": 20,
          "end_col": 5
        }
      ]
    }
  ]
}
```

## ライセンス

MIT License
