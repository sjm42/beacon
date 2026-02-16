# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

- `cargo build` — debug build
- `cargo build --release` — optimized release build (opt-level 3, fat LTO, single codegen unit)
- `cargo run` — run debug build
- `cargo run --release` — run release build
- `cargo clippy` — lint
- `cargo test` — run tests (no test suite currently exists)

## Project Overview

Beacon is a minimal Rust CLI utility that periodically outputs a message (default: "ping" every 60 seconds). It supports configurable messages, intervals, optional timestamps, and multi-level structured logging via tracing.

## Architecture

- **`src/bin/beacon.rs`** — Binary entry point. Parses CLI args, initializes logging, runs the main output loop.
- **`src/lib.rs`** — Library root. Re-exports `tracing` macros and the `startup` module.
- **`src/startup.rs`** — `OptsCommon` struct (clap derive-based CLI options) and logging initialization via `tracing-subscriber`. Embeds build metadata (git branch/commit, source timestamp, compiler version) from `build.rs`.
- **`build.rs`** — Uses `build-data` crate to set compile-time env vars (`GIT_BRANCH`, `GIT_COMMIT`, `SOURCE_TIMESTAMP`, `RUSTC_VERSION`).

## Conventions

- Rust edition 2024, stable toolchain (configured in `rust-toolchain.toml`).
- All source files end with `// EOF` comment.
- Error handling uses `anyhow::Result`.
- Logging macros (`info!`, `debug!`, `trace!`, etc.) are re-exported from `tracing` via `lib.rs` and used throughout with `use beacon::*`.
