# Practice WASM Rust

__[Enter The WebSite](https://neos21.github.io/practice-wasm-rust/)__


## Memo

```bash
# 事前の環境 : WSL2 Ubuntu
$ uname -r
5.10.102.1-microsoft-standard-WSL2
# Rust
$ cargo --version
cargo 1.56.0 (4ed5d137b 2021-10-04)

# npm install -g wasm-pack でも良いらしい https://rustwasm.github.io/wasm-pack/installer/
$ cargo install wasm-pack
$ wasm-pack -V
wasm-pack 0.10.2

# コレを参考にやってみる https://blog.logrocket.com/getting-started-with-webassembly-and-rust/
$ wasm-pack new practice-wasm-rust
$ tree practice-wasm-rust/
practice-wasm-rust/
├── Cargo.toml
├── LICENSE_APACHE
├── LICENSE_MIT
├── README.md
├── src
│   ├── lib.rs
│   └── utils.rs
└── tests
    └── web.rs
# テンプレート中の要らないファイルを消しておく
$ rm -rf ./tests ./.appveyor.yml ./.cargo-ok ./.travis.yml ./LICENSE_APACHE ./LICENSE_MIT
# コードを修正すれば ./src/lib.rs のみだけでもよくなる

# ビルドしてみる
$ wasm-pack build --target web
# ./pkg/ と ./target/ などができるが、使用する最終成果物は ./pkg/practice_wasm_rust.js と ./practice_wasm_rust_bg.wasm の2つ

# WASM を呼び出す HTML を作る
$ touch ./index.html
# ローカルサーバを立てる
$ npx live-server ./ --host
# 動かせた

# 最終成果物だけを取り出すためのスクリプトを書いてみた
$ bash ./build.bash
```

## その他参考資料

- https://qiita.com/poe_hoshi/items/ef685ed579de1a29fd3d
  - https://qiita.com/poe_hoshi/items/94caf704f212cd9f2da9
- https://dev.classmethod.jp/articles/rust-webassembly-javascript/
  - 上の例と同じようなことを日本語で解説している
- https://caddi.tech/archives/879
  - 内部ではココで紹介されているような `WebAssembly.instantiateStreaming()` をゴリゴリ生成してくれている
- https://tech.smartcamp.co.jp/entry/wasm-walkthrough
  - Go 言語など他の言語での例とサイズの比較
- https://rustwasm.github.io/docs/wasm-bindgen/
  - 内部で使われている wasm-bindgen のドキュメント
- https://zenn.dev/rithmety/articles/20200702-webasm-for-deno-30bab6720444f3ad2f3b
  - Deno・TS で簡単に確認している
- https://developer.mozilla.org/ja/docs/WebAssembly/Rust_to_wasm


## Links

- [Neo's World](https://neos21.net/)
- [GitHub - Neos21](https://github.com/Neos21/)
- [GitHub - practice-wasm-rust](https://github.com/Neos21/practice-wasm-rust)
- [GitHub Pages - Practice WASM Rust](https://neos21.github.io/practice-wasm-rust/)
