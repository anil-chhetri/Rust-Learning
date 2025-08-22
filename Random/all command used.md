```rust
cargo new --bin hello_rust
cd hello_rust
cargo run

```


add new project to  "rust-analyzer.linkedProjects" in settings.json file. [this is for rust analyzer to find your projects.]


```rust
cargo check        # fast type-check, no codegen
cargo build        # debug build
cargo build --release
cargo test
cargo clippy -- -D warnings
cargo fmt

```