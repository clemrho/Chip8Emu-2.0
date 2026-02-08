# How to Run Chip8 Sandbox

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

## Optional: Test ROMs
If you want the built-in quick test buttons (IBM Logo, Corax+, Flags), clone the test suite into the repo root:

```bash
git clone https://github.com/Timendus/chip8-test-suite.git
```

Then use the **Quick tests** buttons in the top bar.

## Basic Usage
- **Run / Pause / Step / Reset** control execution.
- **ROM Path** + **Load ROM** lets you load any local `.ch8` file.
- **Cycles/frame** changes emulator speed.
- **Screen scale** changes the display size.

## Running Tests
To run the core logic unit tests:

```bash
cargo test
```

## Troubleshooting
If you encounter issues with `cargo` not being found, ensure your Rust toolchain is correctly added to your PATH.
