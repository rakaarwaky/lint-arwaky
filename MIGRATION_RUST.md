# AES Migration Guide — Rust

> Skill-driven migration workflow for Rust projects to AES architecture.
> Each phase delegates to a dedicated skill in `.agents/skills/`.

See [ARCHITECTURE.md](ARCHITECTURE.md) for layer rules and [README.md](README.md) for project usage.

## Workspace Structure

```
project-root/
├── Cargo.toml              ← workspace manifest (members = ["crates/*"])
├── crates/
│   ├── shared/             ← shared types (subfolders per feature + common/)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs           ← re-exports all subfolders
│   │       ├── common/          ← truly shared across ALL features
│   │       └── <feature>/       ← shared types per feature domain
│   │
│   ├── <feature>/          ← feature crate
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── capabilities_<concept>_<role>.rs
│   │       ├── agent_<concept>_orchestrator.rs
│   │       ├── surface_<concept>_<role>.rs
│   │       ├── root_<concept>_container.rs
│   │       └── lib.rs
│   ├── root_<name>_entry.rs   ← binary entry point (file, NOT directory)
│   └── lib.rs
└── Cargo.lock
```

**Key rules:**

- All 7 layers coexist in each feature slice.
- Stable domain taxonomy, contracts, and utilities live under `crates/shared/<feature>/`.
- Orchestration, capabilities, and surfaces live in the feature crate.
- Entry points (`root_*_entry.rs`) are files inside `crates/`, not separate directories.
- `crates/shared/src/common/` holds types shared across ALL features.

---

## Prerequisites

```bash
cargo install lint-arwaky-cli
lint-arwaky-cli version
lint-arwaky-cli scan your-project/
```

---

## Phase 0: Audit

> **Skill:** `lint-arwaky-rust` — load for audit commands and violation analysis.

```bash
lint-arwaky-cli scan your-project/
find your-project/crates -name "*.rs" | wc -l
```

- Violations < 10 → full migration in one session
- Violations 10-50 → phased migration
- Violations > 50 → start with taxonomy only

---

## Phase 1: Taxonomy Layer

> **Skill:** `create-taxonomy-rust` — load for VOs, errors, constants, entities, events.

Define Value Objects, Errors, Events, and compile-time Constants under `crates/shared/<feature>/`.

### Steps

1. Identify domain types with `grep -rn "pub struct\|pub enum" crates/*/src/ | grep -v test | grep -v mod.rs`
2. Load `create-taxonomy-rust` skill
3. Create taxonomy files following skill templates and workflow
4. Register in domain `mod.rs`
5. Verify: `cargo check -p shared`

---

## Phase 2: Contract Layer

> **Skill:** `create-contract-rust` — load for protocol and aggregate traits.

Contracts define public interfaces (Protocols and Aggregates) without exposing implementation.

### Steps

1. Load `create-contract-rust` skill
2. Create protocol traits (inbound/outbound) under `crates/shared/<feature>/`
3. Create aggregate facade traits under `crates/shared/<feature>/`
4. Register in domain `mod.rs`
5. Verify: `cargo check -p shared`

---

## Phase 3: Utility Layer

> **Skill:** `create-utility-rust` — load for stateless standalone functions.

Utility contains low-level technical mechanics — **stateless standalone functions only**.

### Steps

1. Identify reusable stateless functions across modules
2. Load `create-utility-rust` skill
3. Create utility files under `crates/shared/<feature>/`
4. Register in domain `mod.rs`
5. Verify: `cargo check -p shared`

---

## Phase 4: Capabilities Layer

> **Skill:** `create-capabilities-rust` — load for business logic and external adaptation.

Capabilities contain concrete behavior implementations (business logic + external adapters).

### Steps

1. Load `create-capabilities-rust` skill
2. Create business logic capabilities (implement protocol traits)
3. Create external adaptation capabilities (repositories, clients)
4. Follow 3-Block Structure: Struct → Trait Impl → Constructors
5. Use `Arc<dyn Trait>` for DI
6. Verify: `cargo check -p <feature>`

---

## Phase 5: Agent Layer

> **Skill:** `create-agent-rust` — load for orchestration logic.

Orchestrates sequential execution, branching, looping, and error handling.

### Steps

1. Load `create-agent-rust` skill
2. Create orchestrator struct implementing aggregate trait
3. Inject protocol dependencies via `Arc<dyn Trait>`
4. Verify: `cargo check -p <feature>`

---

## Phase 6: Surface Layer

> **Skill:** `create-surface-rust` — load for user-facing input translation.

Translates user-facing inputs into actions, delegating to the Agent orchestrator.

### Steps

1. Load `create-surface-rust` skill
2. Create surface structs (commands, handlers, endpoints)
3. Inject aggregate trait via `Arc<dyn Trait>`
4. Verify: `cargo check -p <feature>`

---

## Phase 7: Root Layer

> **Skill:** `create-root-rust` — load for DI container and entry point wiring.

Wires concrete implementations to contracts and bootstraps the system.

### Steps

1. Load `create-root-rust` skill
2. Create DI container wiring all capabilities → orchestrator → surface
3. Create entry point file at `crates/root_<name>_entry.rs`
4. Verify: `cargo check -p <feature>`

---

## Phase 8: Verify

> **Skill:** `build-verify-all` — load for final build verification.

```bash
lint-arwaky-cli scan your-project/
cargo test --workspace
cargo fmt --all && cargo clippy --all-targets -- -D warnings
```

---

## Supplementary Skills (Post-Migration)

| Skill | When to Use |
|-------|-------------|
| `add-docs-rust` | Add doc comments, type annotations after migration |
| `fix-bypass-rust` | Remove `#[allow]`, `unwrap()`, `panic!` |
| `cleanup-consolidate-rust` | Remove dead code, merge duplicates |
| `create-test-rust` | Generate test suites |

---

## Reference: File Naming & Import Rules

See [ARCHITECTURE.md](ARCHITECTURE.md) §3 (Naming Convention) and §11 (Import Rules).

| Layer | Pattern |
|-------|---------|
| taxonomy | `taxonomy_<concept>_<suffix>.rs` |
| contract | `contract_<concept>_<suffix>.rs` |
| utility | `utility_<concept>_<suffix>.rs` |
| capabilities | `capabilities_<concept>_<suffix>.rs` |
| agent | `agent_<concept>_orchestrator.rs` |
| surface | `surface_<concept>_<suffix>.rs` |
| root | `root_<concept>_<suffix>.rs` |

---

## Troubleshooting

See [ARCHITECTURE.md](ARCHITECTURE.md) §12 (Troubleshooting) for violation codes and fixes.
