# Chip8Emu-2.0

**Native macOS Chip-8 Emulator & Debugger**

Built with **Rust** + **egui** + **wgpu** + **winit**.

## Overview

This project is a high-performance, system-level Chip-8 emulator designed specifically for macOS. It focuses on providing a robust debugging environment and accurate GPU replay capabilities.

## Tech Stack

*   **Language:** Rust (Stable, 2021 edition)
*   **UI Framework:** [egui](https://github.com/emilk/egui) (Immediate mode GUI, perfect for debuggers)
*   **Graphics:** [wgpu](https://github.com/gfx-rs/wgpu) (WebGPU implementation for native, safe and fast)
*   **Windowing:** [winit](https://github.com/rust-windowing/winit) (Cross-platform window creation and management)

## Features (Planned)

*   **Native macOS Experience:** tailored for macOS desktop environment.
*   **Advanced Debugger:** Step-through execution, register inspection, memory view.
*   **GPU Replay:** leveraging wgpu for rendering.
*   **High Performance:** Core emulation logic written in optimized Rust.

## Project Structure

*   `chip8_core`: Core emulation logic (CPU, Memory, etc.)
*   `chip8_facade`: Interface layer
*   *(Web frontend removed in favor of native desktop app)*

## Getting Started

### Prerequisites

*   Rust toolchain (cargo, rustc)
*   macOS (tested on latest versions)

### Running

```bash
cargo run --release
```
