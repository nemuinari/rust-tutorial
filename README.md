# Rust Tutorial

## セットアップ

#### Rust のインストール

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### インストール確認

```bash
rustc --version
cargo --version
```

## プロジェクト作成

#### 新規プロジェクト作成

```bash
cargo init my_project
cd my_project
```

#### ビルド

```bash
cargo build
```

#### 実行

```bash
cargo run
```

#### リリースビルド

```bash
cargo build --release
cargo run --release
```

## 参考

- [公式ドキュメント](https://www.rust-lang.org/ja/)
- [Rust Book](https://doc.rust-lang.org/book/)
