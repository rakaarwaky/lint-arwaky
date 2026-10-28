# README — Calculator Workspace

## Project Overview

A small multi-language calculator demo workspace with Rust features under `crates/`, Python features under `modules/`, and TypeScript features under `packages/`, all conforming to the AES layer model.

## Prerequisites

- Rust toolchain (edition 2024 via rust-toolchain.toml)
- Python 3.10+
- Node.js / npm

## Quick Start

| Step | Action | Command  |
|------|--------|----------|
| 1 | Build | `CARGO_INCREMENTAL=0 cargo build --release` |
| 2 | Lint | `lint-arwaky-cli check .` |
| 3 | Test | `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests -j 2` |

## Reference

| Artifact | Purpose | Link |
|----------|---------|------|
