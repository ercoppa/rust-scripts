# rust-scripts

Some *scripts* written in Rust. To run them, [`cargo-script`](https://github.com/DanielKeep/cargo-script) is required:
```
cargo install cargo-script
```
Run a script:
```
cargo script script-file.rs
```

**Note**: `cargo-script` does not work if the name of a script is equal to the name of a dependency.