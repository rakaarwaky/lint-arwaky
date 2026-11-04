# HOW TO MAKE MIGRATION RUST

> **Purpose**: Guide phased migration of legacy Rust projects into AES layered architecture — taxonomy → contract → utility → capabilities → agent → surface → root.
>
> **Audience**: Agents and engineers executing a migration to AES.
>
> **Scope**: Phase-based migration workflow for Rust projects; references layer skills for execution.
>
> **Location**: Project root; each phase operates on a layer directory.
>
> **Length**: 9 phases (0–8); total duration depends on violation count.

---

## Rules

Nine rules. Each one governs one migration phase.

1. **Phase 0 first — audit before touching anything.** Run `lint-arwaky-cli scan .`; record baseline violations to choose strategy.
2. **Taxonomy before contract before capabilities.** VOs must exist before protocols can reference them.
3. **Protocol = one file per feature, one trait per capability seam, each trait rich.** One trait declares every method its capability owns, each with a concrete return type. Never a mega `execute(op, …)` that dispatches multiple features behind one name.
4. **Aggregate = exactly one method.** The single `execute(request)` entry point; the response is a taxonomy-defined enum of VOs. New consumer verbs are new variants on the request enum, not new aggregate methods.
5. **Utility is stateless free functions only.** No struct, no impl, no upward imports (AES404).
6. **Capabilities implement protocol; agent implements aggregate.** No cross-layer imports (AES201).
7. **Surface calls aggregate; never imports agent or capabilities.** Dependency arrow points down (AES201 purpose).
8. **Root wires everything; never contains business logic.** Container constructs; entry bootstraps.
9. **Verify at every phase.** `lint-arwaky-cli scan <layer-dir>` → 0 before moving to next phase.

## Workflow

1. **Phase 0 — Audit** — run `lint-arwaky-cli scan .`; record baseline.
2. **Phase 1 — Taxonomy** — extract VOs, errors, constants.
3. **Phase 2 — Contract** — create protocol (one file per feature, one trait per capability seam) + aggregate (exactly one `execute()` method).
4. **Phase 3 — Utility** — extract stateless helpers to shared.
5. **Phase 4 — Capabilities** — implement protocols with business logic.
6. **Phase 5 — Agent** — implement aggregate, delegate to capabilities.
7. **Phase 6 — Surface** — map I/O, call aggregate.
8. **Phase 7 — Root** — wire containers, bootstrap entry.
9. **Phase 8 — Verify** — full scan → 0 violations; compile clean.

---

## Template

Copy, fill, delete nothing. Migration proceeds phase-by-phase; each phase outputs files matching the layer HOW-TO.

### Phase 1 — Taxonomy

```rust
// crates/shared/src/<domain>/taxonomy_<domain>_vo.rs
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct <VOName>(pub String);

impl <VOName> {
    pub fn new(value: String) -> Self {
        Self(value)
    }
}
```

### Phase 2 — Contract

```rust
// crates/shared/src/<domain>/contract_<domain>_protocol.rs
use super::taxonomy_<domain>_request::RequestVO;
use super::taxonomy_<domain>_response::ResponseVO;
use super::taxonomy_<domain>_vo::*;

// One file per feature, one trait per capability seam, each trait rich.
pub trait I<Seam1>Protocol: Send + Sync {
    /// <What operation 1 does.> Return <what it returns>.
    fn <operation_1>(&self, arg: &<VOName>) -> ResultVO;
    /// <What operation 2 does.> Return <what it returns>.
    fn <operation_2>(&self, arg: &<VOName>) -> ResultVO;
}

pub trait I<Seam2>Protocol: Send + Sync {
    /// <What this seam's operation does.> Return <what it returns>.
    fn <operation_1>(&self, arg: &<VOName>) -> ResultVO;
}

// crates/shared/src/<domain>/contract_<domain>_aggregate.rs
// Exactly one method — the single entry point consumers call.
pub trait I<Domain>Aggregate: Send + Sync {
    fn execute(&self, request: RequestVO) -> ResponseVO;
}
```

### Phase 3 — Utility

```rust
// crates/shared/src/<domain>/utility_<domain>_<role>.rs
use super::taxonomy_<domain>_vo::*;

pub fn <helper>(arg: &<VOName>) -> <VOName> {
    // Stateless — no state, no struct, no impl
    <VOName>(arg.0.to_lowercase())
}
```

### Phase 4 — Capabilities

```rust
// crates/<domain>/src/capabilities_<domain>_<role>.rs
use shared::<domain>::contract_<domain>_protocol::I<Domain>Protocol;
use shared::<domain>::taxonomy_<domain>_vo::*;

pub struct <Capability> {
    // Dependencies are typed against a contract protocol, never a raw type.
    dep: Arc<dyn I<SeamDependency>Protocol>,
}

impl I<Seam1>Protocol for <Capability> {
    // Every declared method is implemented — a partial impl will not compile.
    fn <operation_1>(&self, arg: &<VOName>) -> ResultVO {
        // Business logic here
        ...
    }

    fn <operation_2>(&self, arg: &<VOName>) -> ResultVO {
        ...
    }
}

impl <Capability> {
    pub fn new(dep: Arc<dyn I<SeamDependency>Protocol>) -> Self { Self { dep } }
}
```

### Phase 5 — Agent

```rust
// crates/<domain>/src/agent_<domain>_orchestrator.rs
use shared::<domain>::contract_<domain>_aggregate::I<Domain>Aggregate;
use shared::<domain>::contract_<domain>_protocol::I<Domain>Protocol;

pub struct <Domain>Orchestrator {
    proto: Arc<dyn I<Domain>Protocol>,
}

impl I<Domain>Aggregate for <Domain>Orchestrator {
    // One method: match the request enum, dispatch to the right protocol trait.
    fn execute(&self, request: RequestVO) -> ResponseVO {
        match request {
            RequestVO::<Variant1> { .. } => ResponseVO::<Result1>(self.proto.<operation_1>(..)),
            RequestVO::<Variant2> { .. } => ResponseVO::<Result2>(self.proto.<operation_2>(..)),
        }
    }
}

impl <Domain>Orchestrator {
    pub fn new(proto: Arc<dyn I<Domain>Protocol>) -> Self { Self { proto } }
}
```

### Phase 6 — Surface

```rust
// crates/<domain>/src/surface_<domain>_command.rs
use shared::<domain>::contract_<domain>_aggregate::I<Domain>Aggregate;

pub fn main(aggregate: &dyn I<Domain>Aggregate, args: &[String]) -> ExitCode {
    // Parse input, call aggregate, render output
    ...
}
```

### Phase 7 — Root

```rust
// crates/<domain>/src/root_<domain>_container.rs
use shared::<domain>::contract_<domain>_aggregate::I<Domain>Aggregate;
use crate::capabilities_<domain>_<role>::<Capability>;
use crate::agent_<domain>_orchestrator::<Domain>Orchestrator;

pub struct <Domain>Container {
    orchestrator: Arc<dyn I<Domain>Aggregate>,
}

impl <Domain>Container {
    pub fn new(dep: Arc<dyn I<SeamDependency>Protocol>) -> Self {
        let proto: Arc<dyn I<Seam1>Protocol> = Arc::new(<Capability>::new(dep));
        let orch: Arc<dyn I<Domain>Aggregate> = Arc::new(<Domain>Orchestrator::new(proto));
        Self { orchestrator: orch }
    }

    pub fn aggregate(&self) -> Arc<dyn I<Domain>Aggregate> {
        self.orchestrator.clone()
    }
}
```

---

## Section Contract

| Section | Why it belongs here|
| ------- | -------------------|
| Phase number in heading | Makes the migration plan explicit; you know which layer you are on.|
| Rules as numbered list | Each rule prevents one failure mode; agents scan for the first violated rule.|
| Template per phase | Copy-paste-ready skeleton so the agent never guesses the structure.|
| Section Contract table | Justifies each required section; keeps the HOW-TO self-documenting.|
| Verify block | Machine check (lint) + manual check (reader) with exact commands.|

---

## Verify

```bash
# Phase 0 — baseline
lint-arwaky-cli scan .

# Per phase (replace <dir> with the layer directory being migrated)
lint-arwaky-cli scan crates/shared/src/<domain>  # taxonomy + contract + utility
lint-arwaky-cli scan crates/<domain>/src  # capabilities → agent → surface → root

# Final gate
lint-arwaky-cli scan .   # must be 0
cargo check --workspace
```

A pass means naming, layer imports, primitives, and roles are clean. Migration strategy
(phase order, skip conditions) is **manual** — see Rules §1–2.

---

## Rust Lint (Verify Pipeline)

> Shared commands, MCP tools, exit codes, and AES101 naming live in `aes-migration/SKILL.md` (Lint Rules — Global).
> This file only holds Rust deltas: adapters, scan session, Rust-specific fix routing, and role boundaries.

### Adapters & native tooling

`lint-arwaky-cli install` installs external adapters; `lint-arwaky-cli adapters` lists the active ones (Clippy, rustfmt, …); `lint-arwaky-cli external crates/` runs only clippy. `get_config` accepts `"language": "rust"`.

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo check -p <crate-name>
cargo test -p <crate-name>
cargo test --workspace
```

### Scan session

```bash
# Pre-flight — project must build
CARGO_INCREMENTAL=0 cargo build -p <crate> 2>&1 | tail -20

# Scan (text / json / by member / by rule)
lint-arwaky-cli scan workspaces-bad/crates
lint-arwaky-cli scan workspaces-bad/crates --format json
lint-arwaky-cli scan workspaces-bad/crates --member shared
lint-arwaky-cli scan workspaces-bad/crates --filter AES401
lint-arwaky-cli ci crates/ --threshold 80
lint-arwaky-cli scan crates/ --format junit -o reports/
lint-arwaky-cli naming crates/
lint-arwaky-cli orphan crates/ --member shared_common --format json
lint-arwaky-cli import crates/code_analysis
lint-arwaky-cli role crates/
lint-arwaky-cli security crates/
lint-arwaky-cli dependencies crates/
lint-arwaky-cli watch crates/

# Reports → XDG data dir
lint-arwaky-cli scan crates/ --format json \
  > ~/.local/share/aes-lint-arwaky/reports/scan_rust.json
lint-arwaky-cli scan crates/ --format sarif \
  > ~/.local/share/aes-lint-arwaky/reports/scan_rust.sarif

# Self-lint — run on the project root after migration completes
lint-arwaky-cli scan .
```

### Rust-specific fix routing

- **AES101 rename** → after `git mv`, update `mod.rs` references (undeclared module = orphan AES501–506).
- **AES205 circular import** → extract the shared trait/type downward; both sides import it. Route the new interface through `aes-contract`.
- **AES303** → add the missing `struct` / `enum` / `trait` (skipped for `utility`/`surface` layers).
- **AES304** → remove `#[allow(...)]`; replace `unwrap()` / `expect()` / `panic!()` with real error handling (`lint-arwaky-cli fix <path> --filter AES304`).
- **AES403 / AES405** → capability `impl`s target the `_protocol` trait; agent structs hold `Arc<dyn Trait>` aggregates and never `use` capabilities directly.
- **AES404** → utility modules: free functions only — no contract `impl` blocks, no state, no `use` of any layer but `taxonomy`.

### Role boundaries

| Layer | Can contain | Cannot contain |
| :----- | :----- | :----- |
| capabilities | Pure computation, validation | I/O, network, database |
| utility | Stateless `pub fn` helpers | State, trait impls, non-taxonomy imports |
| agent | Orchestration flow | Computation, I/O, business |

### Quick fix recipes

```bash
# AES101/102 rename
git mv src/capabilities_scanner.rs src/capabilities_file_scanner.rs
# then update mod.rs

# AES203 unused import
lint-arwaky-cli fix src/file.rs --filter AES203

# AES304 bypass
grep -rn 'unwrap\|expect\|panic!\|#\[allow' src/
# unwrap → ?; expect → ? with context; panic! → Result::Err; allow → fix warning

# AES501–506 orphan
lint-arwaky-cli orphan crates/ --format json
# truly dead → delete; should be wired → container / mod chain

# AES205 cycle
lint-arwaky-cli import <target-path> --filter AES205
# extract shared types/traits to taxonomy or contract_<concern>_protocol.rs;
# both sides import from contract, never each other
```

---

## Rust Section Contract

| Section | Why it belongs here |
| :----- | :----- |
| Pre-flight build | A scan on a broken crate is noise, not signal. |
| Scan with path + member + format | Findings must be machine-triageable and crate-scoped. |
| Triage order | CRITICAL (AES201/205/304) blocks merge; fix before HIGH/MEDIUM cosmetics. |
| Fix routing by AES code | Wrong fix layer recreates the violation; route via lint-routing in `aes-migration/SKILL.md`. |
| `mod.rs` update after rename | AES101 without re-declaration becomes AES501–506. |
| Role-boundary check | Capabilities/utility/agent rows above must hold after fixes. |
| Re-scan to 0 | Done means zero AES findings, not "mostly clean". |
| Language verify | fmt / clippy / test still pass — AES clean ≠ crate healthy. |

---

## Rust Verify

```bash
# 1. Re-scan — must be 0 AES violations
lint-arwaky-cli scan <target-path>

# 2. Build
CARGO_INCREMENTAL=0 cargo check -p <crate>

# 3. Tests
cargo nextest run -p <crate> --lib --tests

# 4. Format + clippy
cargo fmt --all
CARGO_INCREMENTAL=0 cargo clippy --all-targets -- -D warnings
```

Exit codes (shared, `aes-migration/SKILL.md`): `0` pass · `1` violations · `2` config/parse error.

Manual:

- [ ] `cargo fmt --all` clean.
- [ ] `cargo clippy --all-targets -- -D warnings` clean.
- [ ] `cargo test --workspace` (or `cargo nextest run -p <crate>`) passes.
- [ ] `lint-arwaky-cli scan .` reports 0 violations.
