# HOW TO USE LINT RUST

> **Purpose**: Scan, diagnose, and fix AES architecture violations in a Rust workspace with
> `lint-arwaky-cli`, then re-verify to zero findings.
>
> **Audience**: Agents and engineers running or remediating aes-lint-arwaky on Rust crates.
>
> **Scope**: Rust workspace paths (`crates/`, `workspaces-bad/crates/`), Rust adapters
> (Clippy, rustfmt), Rust-specific fix routing and verify pipeline.
>
> **Location**: Run from the repo root (or the lint target root); reports default under
> `${XDG_DATA_HOME:-~/.local/share}/aes-lint-arwaky/reports/`.
>
> **Length**: One scan session = pre-flight → scan → triage → fix → re-scan to 0. Shared
> commands, MCP tools, exit codes, and AES101 naming live in
> `HOW-TO-USE-LINT-COMMANDS.md`; the full AES fix table lives in
> `HOW-TO-USE-LINT-ROUTING.md`.

---

## Rules

Seven rules. Each one prevents a specific failure mode.

1. **Pre-flight first.** Violations on a non-building crate are unreliable — `cargo build`
   (or `cargo check`) must pass before you trust a scan.
2. **Shared command surface is not restated here.** Global flags, aliases (`lac`/`lat`/`lam`),
   MCP tools, exit codes, and AES101 naming live in `HOW-TO-USE-LINT-COMMANDS.md`;
   triage order and the fix table live in `HOW-TO-USE-LINT-ROUTING.md`. This file only holds Rust deltas.
3. **Path and `--member` follow the Rust layout.** Examples use `crates/`; `--member` targets a
   workspace **member / package** name. `scan`/`ci`/`orphan`/`quality`/`import`/`naming`/`role`/
   `external` accept paths; `--member` is accepted by `scan` and `orphan`.
4. **Triage CRITICAL before HIGH/MEDIUM.** 🔴 AES201, AES205, AES304 → 🟡 AES101–102, AES202,
   AES301–303, AES401–403, AES406, AES505–506 → 🟢 AES203–204, AES305, AES404–405, AES501–504.
5. **Auto-fix only what is auto-fixable.** AES101 rename, AES203 unused import, AES304 bypass
   → `fix` (try `--dry-run` first). AES205 cycle → manual; extract shared type downward via
   `aes-contract`.
6. **After AES101 rename, update `mod.rs`.** `git mv` then re-declare the module — an
   undeclared module is unreachable and reports as an orphan (AES501–506).
7. **Re-scan to 0 before declaring done.** Then run the Rust verify pipeline below
   (fmt / clippy / test); language checks are part of Verify, not a substitute for the AES scan.

---

## Template

Copy, fill, delete nothing.

### Adapters & native tooling

`lint-arwaky-cli install` installs external adapters; `lint-arwaky-cli adapters` lists the
active ones (Clippy, rustfmt, …); `lint-arwaky-cli external crates/` runs only clippy.
`get_config` accepts `"language": "rust"`.

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
lint-arwaky-cli ci crates/ --threshold 80 --format junit
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

# Self-lint for the aes-lint-arwaky project itself
lint-arwaky-cli scan .
```

### Rust-specific fix routing

- **AES101 rename** → after `git mv`, update `mod.rs` references (undeclared module = orphan
  AES501–506).
- **AES205 circular import** → extract the shared trait/type downward; both sides import it.
  Route the new interface through `aes-contract`.
- **AES303** → add the missing `struct` / `enum` / `trait`.
- **AES304** → remove `#[allow(...)]`; replace `unwrap()` / `expect()` / `panic!()` with real
  error handling (`fix-bypass`).
- **AES403 / AES405** → capability `impl`s target the `_protocol` trait; agent structs hold
  `Arc<dyn Trait>` aggregates and never `use` capabilities directly.
- **AES404** → utility modules: free functions only — no contract `impl` blocks, no state,
  no `use` of any layer but `taxonomy`.

### Role boundaries

| Layer | Can contain | Cannot contain |
| ----- | ----------- | -------------- |
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

## Section Contract

Every Rust lint session is required to carry the rows that apply. Each exists for one reason.

| Section | Why it belongs here |
| ------- | ------------------- |
| Pre-flight build | A scan on a broken crate is noise, not signal. |
| Scan with path + member + format | Findings must be machine-triageable and crate-scoped. |
| Triage order | CRITICAL (AES201/205/304) blocks merge; fix before HIGH/MEDIUM cosmetics. |
| Fix routing by AES code | Wrong fix layer recreates the violation; route via `HOW-TO-USE-LINT-ROUTING.md`. |
| `mod.rs` update after rename | AES101 without re-declaration becomes AES501–506. |
| Role-boundary check | Capabilities/utility/agent rows above must hold after fixes. |
| Re-scan to 0 | Done means zero AES findings, not "mostly clean". |
| Language verify | fmt / clippy / test still pass — AES clean ≠ crate healthy. |

---

## Verify

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

Exit codes (shared, `HOW-TO-USE-LINT-COMMANDS.md`): `0` pass · `1` violations · `2` config/parse error.

Checklist:

- [ ] `cargo fmt --all` clean.
- [ ] `cargo clippy --all-targets -- -D warnings` clean.
- [ ] `cargo test --workspace` (or `cargo nextest run -p <crate>`) passes.
- [ ] `lint-arwaky-cli scan .` reports 0 violations.
