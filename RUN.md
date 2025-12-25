# How to Run Chip8-Sandbox

## Prerequisites
- Rust (latest stable)
- macOS (tested on latest versions)

### Installing Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Running the App
To start the native macOS application:

```bash
cargo run --release
```

## Running Tests
To run the core logic unit tests:

```bash
cargo test
```

## Troubleshooting
If you encounter issues with `cargo` not being found, ensure your Rust toolchain is correctly added to your PATH.