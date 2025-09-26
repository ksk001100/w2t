w2t is a command-line tool that converts white (or bright) areas in an image to transparent, saving the result as a PNG with transparency.

# w2t

**w2t**は、画像の白（または明るい）部分を透過PNGに変換するツールです。Rust製のモノレポ構成で、CLIツール・Webアプリ・コアライブラリを含みます。

## プロジェクト構成



## CLIツール

### インストール

Cargo（Rust公式パッケージマネージャ）がインストールされていれば、以下のコマンドでCLIツールをインストールできます。

```sh
cargo install --path cli
```

インストール後、`w2t`コマンドが利用可能になります。

### ビルド

Rustがインストールされている環境で、`cli/`ディレクトリに移動しビルドします。

```sh
cd cli
cargo build --release
```

### 使い方

```sh
./target/release/w2t <input_path> <output_path> [--threshold <value>]
```

- `<input_path>`: 入力画像ファイルのパス
- `<output_path>`: 出力PNGファイルのパス
- `--threshold`, `-t`, `-th`: 透過の閾値（0–255, デフォルト: 240）

#### 例

```sh
./target/release/w2t input.jpg output.png --threshold 250
```
この例では、RGB値が250以上のピクセルが透過されます。

```sh
for t in {0..255}; do ./target/release/w2t input.jpg threshold_${t}.png -t $t; done
```
このコマンドで、閾値0〜255の256通りの透過PNGを一括生成できます。

---

## Webアプリ

`web/`配下にWeb版（WASM + JS）があり、ローカルでビルド・動作確認できます。

### ビルド例

```sh
cd web
wasm-pack build --release --target web
# または
# trunk serve
```

`index.html`をブラウザで開くとGUIで画像変換が可能です。

---

## コアライブラリ

`core/`配下に画像処理ロジックが実装されており、CLI・Web両方から利用されています。

---

## ライセンス

MIT

## Author

ksk001100 (<hm.pudding0715@gmail.com>)