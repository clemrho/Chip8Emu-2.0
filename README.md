# Chip8 Sandbox

**Native macOS Chip-8 Emulator & Debugger**

Built with **Rust** + **egui** + **wgpu** + **winit**.

## Overview

Chip8 Sandbox is a native macOS Chip‑8 emulator with a developer‑friendly UI. It combines a simple CPU core with a clean visual debugger so you can step through ROMs, inspect registers, and see the 64×32 display in real time.

## Features

* **Screen Renderer:** 64×32 monochrome display with adjustable scale.
* **Register Analyzer:** live V registers, I, PC, SP, DT, ST.
* **Memory Viewer:** hex dump around the current PC.
* **Disassembly Peek:** shows the next opcode at the current PC.
* **Run Controls:** Run/Pause/Step/Reset plus cycles-per-frame tuning.
* **Quick Test ROMs:** IBM Logo, Corax+, Flags (if you have `chip8-test-suite`).

## Tech Stack

* **Language:** Rust (Stable, 2021 edition)
* **UI Framework:** [egui](https://github.com/emilk/egui)
* **Graphics:** [wgpu](https://github.com/gfx-rs/wgpu)
* **Windowing:** [winit](https://github.com/rust-windowing/winit)

## Project Structure

* `core`: CPU, memory, timers, and opcode implementation.
* `frontend`: egui-based UI (screen, panels, timeline).
* `main.rs`: native app entry point.

## Quick Start

### Prerequisites

* Rust toolchain (cargo, rustc)
* macOS (tested on latest versions)

### Run

```bash
cargo run --release
```

### Optional: Test ROMs

The UI has quick buttons for three standard test ROMs. If you want those to work, clone the test suite into this repo root:

```bash
git clone https://github.com/Timendus/chip8-test-suite.git
```

Then use the **Quick tests** buttons:

* **IBM Logo**
* **Corax+**
* **Flags**

## How To Use

1. Launch the app with `cargo run --release`.
2. Load a ROM using:
   * The **ROM Path** field + **Load ROM**, or
   * The **Quick tests** buttons (if the test suite is present).
3. Use **Run**, **Pause**, **Step**, and **Reset** to control execution.
4. Tune **Cycles/frame** for speed and **Screen scale** for display size.

## Tests

```bash
cargo test
```
