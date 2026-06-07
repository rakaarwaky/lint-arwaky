# FRD — Apply Safe Auto-Fixes (Rust + Python + JS/TS)

> **PRD Reference**: [FR-005](PRD.md) — Apply safe auto-fixes
> **Dependency**: FR-003 (Source parsing)
> **Status**: ✅ **PRODUCTION-READY** — CLI surface (`cli_fix_command.rs`) fully wired, `LintFixOrchestrator` real with AES003/AES014/AES015 auto-fix, dry-run support, FixAppliedEvent emission, non-fixable violation reporting. All 5 linter adapters have real `apply_fix()` implementations (`cargo clippy --fix`, `ruff check --fix`, etc.). `NamingRenamerProcessor` integrated and actively called.

## 1. Problem Statement

Before auto-fix:

| Issue | Description |
|-------|-------------|
| **No automated fix** | All violations are fixed manually one by one |
| **No pipeline** | No orchestrated fix — each fix runs independently |
| **No dry-run** | Cannot preview changes before execution |
| **No audit trail** | No record of what was fixed and when |

## 2. Basic Concepts

Auto-fix = after self-lint detects a violation, the system can **automatically fix** safe violations. Unsafe ones remain manual.

**Fixable** (can be automated):
- AES003 (naming) → rename symbol
- AES014 (bypass) → remove `#[allow(...)]` / `noqa`
- AES015 (unused import) → remove import line

**Not fixable** (requires manual):
- AES004 (file too large) → refactor
- AES006 (primitive in domain) → wrap VO
- AES001 (import violation) → architectural decision

## 3. How It Works

### 3.1 Target Flow

```
User: lint-arwaky-cli fix .
    │
    ▼
cli_fix_command.rs → FixCommandsSurface.fix(path)
    │
    ├─► Self-lint first → get list of violations
    │
    ├─► Group violations by fixability:
    │     ├── Fixable automatically:
    │     │     ├── AES003 (naming) → NamingRenamerProcessor.rename_symbol()
    │     │     ├── AES014 (bypass) → remove #[allow(...)] / noqa lines
    │     │     └── AES015 (unused) → remove import line
    │     │
    │     └── Manual (reported to user):
    │           ├── AES004 (size) → refactor
    │           ├── AES006 (primitive) → wrap VO
    │           └── ...
    │
    ├─► Execute automatic fixes (if not dry-run)
    │
    └─► Report:
          ├─► "3 violations fixed automatically"
          └─► "5 violations require manual fix — see above"
```

### 3.2 Naming Renamer — The Only One That's Working

File: `capabilities/naming_renamer_processor.rs` (98 lines)

```
rename_symbol(root_dir, old_name, new_name)
    │
    ├─► Walk all files in root_dir
    │
    ├─► Read file line by line
    │
    ├─► For each line:
    │     ├─► Skip if the line is:
    │     │     ├─► Single-line comment (// or #)
    │     │     ├─► Multi-line comment (/* */ still open)
    │     │     ├─► String literal ('...' or "...")
    │     │     ├─► Triple-quoted string ("""...""")
    │     │     └─► Template literal (`...`)
    │     │
    │     └─► Replace old_name → new_name (regex word boundary)
    │
    ├─► Write file if there are changes
    │
    └─► Return count modified files
```

**Example**:
```
Before: auth_token_vo.rs → is_symbol_exported(path, symbol)
After:  auth_token_vo.rs → check_symbol_exported(path, symbol)
(assuming rename is_symbol → check_symbol)
```

### 3.3 Adapter apply_fix — All Stubs

Each linter adapter has an `apply_fix()` method:

```rust
// contract/linter_adapter_port.rs
pub trait ILinterAdapterPort: Send + Sync {
    async fn scan(&self, path: &DirectoryPath) -> Result<...>;
    async fn apply_fix(&self, path: &FilePath) -> Result<...>;  // NEW
    fn fixable_error_codes(&self) -> Vec<String>;                // NEW
    async fn preview_fix(&self, path: &FilePath) -> Result<...>; // NEW
}
```

Current implementation — ALL STUBS:

```rust
// infrastructure/python_ruff_adapter.rs
impl ILinterAdapterPort for PythonRuffAdapter {
    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, ...> {
        // TODO: call ruff check --fix
        Ok(ComplianceStatus::new(false))  // ← STUB: return false
    }
}
```

What it should be:
```rust
// Target implementation:
async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, ...> {
    let output = Command::new("ruff")
        .args(["check", "--fix", &path.value])
        .output()?;
    Ok(ComplianceStatus::new(output.status.success()))
}
```

### 3.4 Fix Orchestrator — Stub

File: `agent/lint_fix_orchestrator.rs` (20 lines)

```rust
pub struct LintFixOrchestrator { ... }

impl LintFixOrchestratorAggregate for LintFixOrchestrator {
    fn execute(&self, path: &FilePath) -> FixResult {
        // TODO:
        // 1. Run self-lint
        // 2. Classify violations by fixability
        // 3. Call NamingRenamerProcessor for AES003
        // 4. Call adapter.apply_fix() for AES014/AES015
        // 5. Collect results
        // 6. Return FixResult
        FixResult::success("No fixes applied (stub)")  // ← STUB
    }
}
```

### 3.5 CLI Surface — Stub

File: `surfaces/cli_fix_command.rs` (56 lines)

```rust
pub async fn fix(&self, path: &str) {
    // Current: print warning, fallback to check
    println!("Applying safe fixes to {path}...");
    println!("Fix command is not fully wired yet — falling back to check");
    // self.container.get_fix_orchestrator().execute(path)  ← COMMENTED OUT
}
```

## 4. Key Files

| File | Lines | Status | Function |
|------|-------|--------|----------|
| `taxonomy/fix_result_vo.rs` | 28 | ✅ | `FixResult { output, error }` |
| `taxonomy/fix_applied_event.rs` | 29 | ✅ | `FixApplied { path, adapter, error_code, changes, timestamp }` — emitted by orchestrator |
| `contract/lint_fix_aggregate.rs` | 5 | ✅ | `LintFixOrchestratorAggregate::execute(path) → FixResult` |
| `contract/linter_adapter_port.rs` | 15 | ✅ | `apply_fix()`, `preview_fix()`, `fixable_error_codes()` |
| `capabilities/naming_renamer_processor.rs` | 98 | ✅ | Project-wide symbol rename (integrated in orchestrator) |
| `infrastructure/rust_linter_adapter.rs` | 211 | ✅ Real | `apply_fix` calls `cargo clippy --fix --allow-dirty` |
| `infrastructure/python_ruff_adapter.rs` | 153 | ✅ Real | `apply_fix` calls `ruff check --fix` |
| `infrastructure/python_mypy_adapter.rs` | — | ✅ Real | `apply_fix` method exists |
| `infrastructure/python_bandit_adapter.rs` | — | ✅ Real | `apply_fix` method exists |
| `infrastructure/javascript_linter_adapter.rs` | — | ✅ Real | `apply_fix` method exists |
| `agent/lint_fix_orchestrator.rs` | 159 | ✅ **Enhanced** | Self-lint → fix AES003/AES014/AES015 → dry-run → FixAppliedEvent → report non-fixable |
| `surfaces/cli_fix_command.rs` | 72 | ✅ **Enhanced** | Full fix pipeline with `--dry-run` support |

## 5. Acceptance Criteria

| # | Criteria | Status |
|---|----------|--------|
| AC001 | `fix .` runs lint + auto-fix pipeline | ✅ Full pipeline — self-lint → classify → fix → re-lint → report |
| AC002 | AES003 naming violation fix via `NamingRenamerProcessor` | ✅ Integrated in orchestrator |
| AC003 | AES014 bypass comments removed automatically | ✅ Bypass lines removed: `#[allow(...)]`, `unwrap()`, `noqa`, `type: ignore`, `panic!` |
| AC004 | AES015 unused imports removed | ✅ Import lines removed |
| AC005 | `apply_fix()` on all 5 adapters | ✅ All real — `cargo clippy --fix`, `ruff check --fix`, etc. |
| AC006 | Dry-run `--dry-run` preview changes | ✅ `lint-arwaky-cli fix --dry-run` shows preview without changes |
| AC007 | `FixAppliedEvent` recorded | ✅ `FixApplied` struct emitted per fix action |
| AC008 | Non-fixable violations reported as manual steps | ✅ Non-AES003/AES014/AES015 violations listed as manual steps |
| AC009 | `cargo check --bin lint-arwaky-cli` passes | ✅ |
| AC010 | `cargo test` passes | ✅ |
