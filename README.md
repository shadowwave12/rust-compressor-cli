# Rust Compressor CLI 🦀📦

A simple and efficient command-line tool written in Rust for compressing and decompressing files using GZIP format.

## 🚀 Features

- 🔒 Compress any text file into `.gz`
- 🔓 Decompress `.gz` files back to text
- 🧠 Built using idiomatic Rust (error handling, ownership, buffering)
- 🛠️ CLI powered by [`clap`](https://docs.rs/clap) crate

## 🧪 How to Use

### 📦 Compress a file

```bash
cargo run -- compress input.txt output.gz
```
### Decompress a file
```bash
cargo run -- decrypt output.gz output.txt
```

###Example
```bash
cargo run -- compress input.txt compressed.gz
cargo run -- decrypt compressed.gz restored.txt
```
