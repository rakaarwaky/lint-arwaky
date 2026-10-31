# AGENTS Exemplar

A filled AGENTS against the Ecommerce Application project, matching the Template shape exactly.

```markdown
# AGENTS — Ecommerce Application

## Agent Overview

AI agents working in this workspace must know the AES 7-layer naming convention, the pinned Rust edition 2024 toolchain, and the exact CI gates before generating code, docs, or suggestions. ARCHITECTURE.md wins on ambiguity.

## Runtime

| Aspect          | Detail                                          |
|-----------------|-------------------------------------------------|
| Language        | Rust (edition 2024 via rust-toolchain.toml)     |
| Environment     | cargo workspace under crates/                   |
| Artifacts       | target/ and $HOME/.local/bin/                   |
| Package Manager | cargo, with Cargo.lock                          |
| Version Probe   | `rustc --version`                               |
| Setup           | `CARGO_INCREMENTAL=0 cargo build --release`     |
| Activation      | None                                            |

## Project Structure

```text
ecommerce-app/
├── crates/
│   ├── shared/                              # shared kernel: domain contracts + VOs
│   │   ├── product/                         # shared subdomain: product
│   │   │   ├── contract_product_protocol.rs # IProductProtocol (Contract layer)
│   │   │   ├── taxonomy_product_vo.rs       # product value objects (Taxonomy layer)
│   │   │   ├── utility_product_helpers.rs   # shared stateless helpers (Utility layer)
│   │   │   ├── DATA.md                      # shared subdomain data shapes
│   │   │   └── BACKLOG.md                   # shared subdomain status
│   │   └── order/                           # shared subdomain: order
│   │       ├── contract_order_protocol.rs
│   │       ├── taxonomy_order_vo.rs
│   │       ├── utility_order_helpers.rs
│   │       ├── DATA.md
│   │       └── BACKLOG.md
│   ├── checkout/                            # feature: order processing
│   │   ├── capabilities_checkout.rs         # checkout invariants + mutations
│   │   ├── agent_checkout_orchestrator.rs   # checkout orchestration
│   │   ├── root.rs                          # wires contract -> capabilities
│   │   ├── FRD.md                           # feature spec
│   │   └── BACKLOG.md                       # feature status
│   ├── catalog/                             # feature: product catalog
│   │   ├── capabilities_catalog.rs
│   │   ├── agent_catalog_orchestrator.rs
│   │   ├── root.rs
│   │   ├── FRD.md
│   │   └── BACKLOG.md
│   └── web/                                 # surface: browser storefront + seller dashboard
│       ├── surface_web.rs                   # surface layer
│       ├── DESIGN.md                        # surface design system
│       └── BACKLOG.md                       # surface status
├── Cargo.toml                               # workspace manifest
├── AGENTS.md                                # this file
├── PRD.md                                   # product spec (root)
└── ROADMAP.md                               # root master (roadmap + status)
```

## Commands

| Category    | Command                              | Scope      | CI Gate              |
|-------------|--------------------------------------|------------|----------------------|
| Test (all)  | `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests -j 2`     | All        | ci.yml: Test         |
| Test (unit) | `CARGO_INCREMENTAL=0 cargo nextest run -p <crate>`                      | One unit   | ci.yml: Test         |
| Test (file) | `CARGO_INCREMENTAL=0 cargo nextest run -p <crate> --test <file>`        | One file   | ci.yml: Test         |
| Lint        | `CARGO_INCREMENTAL=0 cargo fmt --all -- --check`                         | All        | ci.yml: Format       |
| Type Check  | `CARGO_INCREMENTAL=0 cargo clippy --all-targets -- -D warnings`          | All        | ci.yml: Clippy       |
| Arch Scan   | `lint-arwaky-cli check .`                                                | All        | ci.yml: Self-Lint    |

## Reference

| Artifact     | Purpose                                    | Link |
|--------------|--------------------------------------------|------|
| README       | What the tool does, every command, install | [README.md](README.md) |
| CONTRIBUTING | Branch, commit, and PR conventions         | [CONTRIBUTING.md](CONTRIBUTING.md) |
| ARCHITECTURE | Layer contract for every crate             | [ARCHITECTURE.md](ARCHITECTURE.md) |
| ROADMAP      | What ships next, in order                  | [ROADMAP.md](ROADMAP.md) |
| PRD          | What the product does and why              | [PRD.md](PRD.md) |

```
