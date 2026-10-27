---
trigger: always
description: "Calculator workspace operational guide."
---
# Calculator Workspace

## Runtime

- Language: Rust (edition 2024 via rust-toolchain.toml), Python 3.10+, TypeScript ES2022
- Environment: cargo workspace under crates/, setuptools for Python, npm for TypeScript
- Artifacts: target/ and $HOME/.local/bin/ (install path). Never use /tmp for build output a reviewer must find
- Package manager: cargo, with Cargo.lock

```bash
rustc --version
CARGO_INCREMENTAL=0 cargo build --release
```

## Project Quick Facts

{project quick facts}

## Pipeline

scan → check → gates → publish

## Project Structure

{project structure}

## Commands

Every command must match the exact CI gate.

```bash
# Tests
CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests -j 2   # all tests
CARGO_INCREMENTAL=0 cargo nextest run -p <crate>                      # one unit
CARGO_INCREMENTAL=0 cargo nextest run -p <crate> --test <file>        # one file

# Lint / types / architecture
CARGO_INCREMENTAL=0 cargo fmt --all -- --check                         # matches ci.yml "Format" job
CARGO_INCREMENTAL=0 cargo clippy --all-targets -- -D warnings          # matches ci.yml "Clippy" job
lint-arwaky-cli check .                                                # architecture scanner (self-lint)
```

## Related Documents

- [ARCHITECTURE.md](ARCHITECTURE.md): layer contract for every crate and the rules that keep the boundaries
- [CONTRIBUTING.md](CONTRIBUTING.md): branch, commit, and PR conventions
- [PRD.md](PRD.md): what the product does and why — feature tiers, exit codes, non-functional goals
- [README.md](README.md): what the tool does, every command, and how to install it
- [ROADMAP.md](ROADMAP.md): what ships next, in order
---

## Reference
| Prd | {Link to PRD.md} | [{link}](PRD.md) |
| Frd | {Link to FRD.md} | [{link}](FRD.md) |
| Data | {Link to DATA.md} | [{link}](DATA.md) |
| Design | {Link to DESIGN.md} | [{link}](DESIGN.md) |
| Architecture | {Link to ARCHITECTURE.md} | [{link}](ARCHITECTURE.md) |
