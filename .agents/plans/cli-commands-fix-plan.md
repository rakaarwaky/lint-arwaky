# CLI-Commands Fix Plan

> Generated from BA + QA + Backend report validation against actual codebase.
> 28 validated issues across 7 phases.
> P1.1 (global init) and P1.2 (MCP binary) already fixed in codebase — removed from plan.

## Decisions

| Decision | Choice |
|----------|--------|
| Plan structure | Single comprehensive plan, phases ordered by severity |
| Architecture refactor | Include IAnalysisPipelineAggregate contract + agent |
| Dead CLI commands | Remove Duplicates enum, skip tui (out of scope) |
| Global init | Remove `--global` flag — XDG installation is `install.local.sh` responsibility |
| Exit code convention | 0=success, 1=violations found, 2=system error, 3=tool missing |

## Severity Legend

- **CRITICAL**: Security vulnerability or data-correctness bug
- **HIGH**: False positive/negative, architectural violation, or functional bug
- **MEDIUM**: Improvement, missing check, or performance fix
- **LOW**: Documentation or minor optimization

---

## Phase 1: Security Hardening (CRITICAL)

### P1.1 — Remove orphaned `Duplicates` CLI definition

**Skill**: `create-surface-rust` — removing dead code from surface taxonomy.
**File**: `crates/shared/src/cli-commands/taxonomy_cli_vo.rs`
**Severity**: CRITICAL
**AES Code**: Correctness
**Problem**: `Duplicates` subcommand is defined but has no handler. Executing it causes runtime panic.

**Fix**: Remove the `Duplicates` variant from the `Commands` enum.

---

### P1.2 — Remove `--global` flag from `init` command

**Skill**: `create-surface-rust` — removing feature from surface. XDG installation is `install.local.sh` responsibility, not CLI.
**Files**: `crates/shared/src/cli-commands/taxonomy_cli_vo.rs`, `crates/cli-commands/src/surface_setup_command.rs`
**Severity**: HIGH
**AES Code**: Architecture
**Problem**: `init --global` duplicates what `install.local.sh` already does. CLI should only handle local init. Global installation belongs to the install script.

**Fix**:
1. Remove `--global` flag from `Init` subcommand in `taxonomy_cli_vo.rs`
2. Remove `handle_init_global()` function from `surface_setup_command.rs`
3. Remove `TRUSTED_DOCS` constant and `include_str!` imports (no longer needed)
4. Simplify `handle_init()` to only handle local init
5. Update FRD to remove `--global` reference

---

## Phase 2: Critical Bug Fixes (CRITICAL/HIGH)

### P2.1 — Fix file walker broken on non-Unix platforms

**Skill**: `create-utility-rust` — file walker is stateless, domain-agnostic, reusable.
**File**: `crates/shared/src/common/utility_file.rs`
**Severity**: CRITICAL
**AES Code**: Correctness
**Problem**: `get_inode()` returns `0` on non-Unix, causing walker to skip all directories after the first. Also, symlink check uses string prefix matching.

**Fix**: Replace inode-based walker with canonical-path-based walker. Use `HashSet<PathBuf>` for visited set. Fix symlink check to use workspace root, not parent directory.

Key changes:
- Remove `get_inode()` helper and Unix metadata import
- Change `visited` from `HashSet<u64>` (inodes) to `HashSet<PathBuf>` (canonical paths)
- Fix symlink check: `target.starts_with(root)` instead of `target.starts_with(dir)`

---

### P2.2 — Fix multi-workspace scan corrupting structured output

**Skill**: `create-surface-rust` — surface must not print human-readable text before machine-readable output.
**File**: `crates/cli-commands/src/surface_check_command.rs`
**Severity**: CRITICAL
**AES Code**: Correctness
**Problem**: `scan_with_discovery` prints banners before JSON/SARIF/JUnit output, corrupting structured output.

**Fix**: Guard all `println!` banners with `matches!(format, Format::Text)`.

```rust
if multi && matches!(format, Format::Text) {
    println!("Lint Arwaky v{} (Multi-Workspace Mode)", env!("CARGO_PKG_VERSION"));
    // ...
}
```

---

### P2.3 — Fix path filtering using string prefix

**Skill**: `create-utility-rust` — path containment check must be component-aware.
**File**: `crates/cli-commands/src/surface_check_command.rs`
**Severity**: HIGH
**AES Code**: Correctness
**Problem**: `abs_path.to_string_lossy().starts_with(&canonical_scan_path)` matches unrelated paths (e.g., `/repo/foo` matches `/repo/foobar`).

**Fix**: Use `Path::starts_with()` which is component-aware.

```rust
let in_scope = |file_value: &str| {
    let file_path = std::path::Path::new(file_value);
    let abs_path = if file_path.is_absolute() {
        file_path.to_path_buf()
    } else {
        cwd_canonical.join(file_value)
    };
    abs_path.starts_with(&scan_path)
        || file_path.starts_with(path)
};
```

---

### P2.4 — Fix workspace filtering fallback duplicating results

**Skill**: `create-surface-rust` — surface must handle canonicalization failure gracefully.
**File**: `crates/cli-commands/src/surface_check_command.rs`
**Severity**: HIGH
**AES Code**: Correctness
**Problem**: If workspace canonicalization fails, filter becomes `true`, duplicating all results across workspaces.

**Fix**: Build a fallback path from the raw workspace path when canonicalization fails. Never default to `true`.

```rust
let ws_fallback = if std::path::Path::new(&ws.path.value).is_absolute() {
    std::path::PathBuf::from(&ws.path.value)
} else {
    cwd_canonical.join(&ws.path.value)
};
let ws_fallback = std::fs::canonicalize(&ws_fallback).unwrap_or(ws_fallback);

let in_workspace = |file_value: &str| {
    // ... use ws_canonical or ws_fallback, never default to true
};
```

---

### P2.5 — Fix `check --git-diff` ignoring path and filter

**Skill**: `create-surface-rust` — surface must pass user-provided arguments to handlers.
**Files**: `crates/cli-commands/src/surface_check_action.rs`, `crates/cli-commands/src/surface_git_command.rs`
**Severity**: HIGH
**AES Code**: Correctness
**Problem**: `handle_git_diff` hardcodes `FilePath::new(".")` and ignores `--filter`.

**Fix**: Update `handle_git_diff` signature to accept `project_path: FilePath` and `filter: Option<String>`. Update `handle_check` to pass user-provided path and filter.

---

### P2.6 — Fix `adapters` command ignoring actual adapters

**Skill**: `create-surface-rust` — surface must use injected aggregate, not hardcoded list.
**File**: `crates/cli-commands/src/surface_plugin_command.rs`
**Severity**: HIGH
**AES Code**: Correctness
**Problem**: `handle_adapters` prints hardcoded list, ignores injected `IExternalLintAggregate`.

**Fix**: Call `external_lint.adapter_names()` and print actual adapters.

```rust
pub fn handle_adapters(external_lint: Arc<dyn IExternalLintAggregate>) -> ExitCode {
    println!("External lint adapters:");
    let adapters = external_lint.adapter_names();
    if adapters.is_empty() {
        println!("  (none enabled)");
    } else {
        for adapter in adapters {
            println!("  - {adapter}");
        }
    }
    ExitCode::SUCCESS
}
```

---

### P2.7 — Fix CI threshold comparison truncating score

**Skill**: `create-surface-rust` — surface must use precise numeric comparison.
**File**: `crates/cli-commands/src/surface_common_command.rs`
**Severity**: HIGH
**AES Code**: Correctness
**Problem**: `(score.value() as u32) < threshold.value()` truncates float score.

**Fix**: Compare as floats.

```rust
let below_threshold = score.value() < threshold.value() as f64;
```

---

### P2.8 — Fix `xml_escape` producing invalid XML

**Skill**: `create-surface-rust` — surface must produce valid XML for JUnit output.
**File**: `crates/cli-commands/src/surface_check_command.rs`
**Severity**: HIGH
**AES Code**: Correctness
**Problem**: `xml_escape` pushes same character back instead of XML entities.

**Fix**: Replace with proper XML escaping.

```rust
fn xml_escape(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len() + 16);
    for c in s.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            other => escaped.push(other),
        }
    }
    escaped
}
```

---

### P2.9 — Fix silent audit error swallowing

**Skill**: `create-surface-rust` — surface must report subsystem failures, not swallow them.
**File**: `crates/cli-commands/src/surface_check_command.rs`
**Severity**: HIGH
**AES Code**: Error handling
**Problem**: `naming_results.unwrap_or_default()` silently discards failures.

**Fix**: Match on results and print warnings.

```rust
match naming_results {
    Ok(values) => all_results.extend(values),
    Err(e) => eprintln!("[warn] naming audit failed: {e}"),
}
match import_results {
    Ok(values) => all_results.extend(values),
    Err(e) => eprintln!("[warn] import audit failed: {e}"),
}
```

---

### P2.10 — Fix `scan --member` substring matching

**Skill**: `create-surface-rust` — surface must use exact matching for member selection.
**File**: `crates/cli-commands/src/surface_check_command.rs`
**Severity**: HIGH
**AES Code**: Correctness
**Problem**: `ws.path.value.contains(member_name)` matches unintended members (e.g., `sha` matches `shared`).

**Fix**: Use exact filename comparison.

```rust
ws_file == member_name || ws.path.value == member_name
```

---

### P2.11 — Fix `dependencies` returning success on failure

**Skill**: `create-surface-rust` — surface must return correct exit codes.
**File**: `crates/cli-commands/src/surface_maintenance_command.rs`
**Severity**: HIGH
**AES Code**: Error handling
**Problem**: `handle_dependencies` returns `ExitCode::SUCCESS` even when `Err(e)` occurs.

**Fix**: Return `ExitCode::from(1)` on error.

```rust
Err(e) => {
    println!("{e}");
    return ExitCode::from(1);
}
```

---

## Phase 3: Performance Fixes (HIGH)

### P3.1 — Fix orphan detection scanning ignored directories

**Skill**: `create-surface-rust` — surface must use ignore-aware file collection.
**File**: `crates/cli-commands/src/surface_check_command.rs`
**Severity**: HIGH
**AES Code**: Performance
**Problem**: `collect_all_source_files_raw` applies no ignore rules, scanning `node_modules`, `target`, `.git`.

**Fix**: Replace with `collect_all_source_files` (ignore-aware).

```rust
let all_source_files: Vec<String> =
    shared::common::collect_all_source_files(&scan_root)
        .iter()
        .map(|f| f.value.clone())
        .collect();
```

---

### P3.2 — Add missing directories to default ignore list

**Skill**: `create-utility-rust` — ignore list is domain-agnostic, reusable.
**File**: `crates/shared/src/common/utility_file.rs`
**Severity**: MEDIUM
**AES Code**: Performance
**Problem**: Default ignore list misses `.git`, `dist`, `build`, `coverage`, `.venv`.

**Fix**: Add missing directories to `default_ignored_paths()`.

```rust
let mut ignored: Vec<String> = vec![
    "target".into(), "test-workspaces".into(), ".mimocode".into(),
    ".agents".into(), "node_modules".into(), "build.rs".into(),
    ".git".into(), "dist".into(), "build".into(),
    "coverage".into(), ".venv".into(),
];
```

---

### P3.3 — Defer container construction for lightweight commands

**Skill**: `create-root-rust` — entry point should only construct what's needed.
**File**: `crates/cli-commands/src/root_cli_container.rs` (or binary entrypoint)
**Severity**: MEDIUM
**AES Code**: Performance
**Problem**: `CliContainer::new_default()` eagerly builds all subsystems even for `version`, `adapters`.

**Fix**: In binary entrypoint, handle lightweight commands before constructing full container.

```rust
match cli.command {
    Commands::Version => {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }
    Commands::Adapters => {
        // Only construct external_lint container
        let el = ExternalLintContainer::new_default();
        return handle_adapters(el.aggregate());
    }
    _ => {
        let container = CliContainer::new_default();
        // ... existing dispatch
    }
}
```

---

## Phase 4: Architecture Refactor — IAnalysisPipelineAggregate (HIGH)

### P4.1 — Create IAnalysisPipelineAggregate contract

**Skill**: `create-contract-rust` — pure trait definition, no implementation, VO-based signatures.
**File**: `crates/shared/src/cli-commands/contract_analysis_pipeline_aggregate.rs` (NEW)
**Severity**: HIGH
**AES Code**: Architecture (Surface doing orchestration)

**Contract**:
```rust
#[async_trait]
pub trait IAnalysisPipelineAggregate: Send + Sync {
    async fn run(&self, request: ScanRequest) -> Result<ScanReport, PipelineError>;
}
```

**Supporting types** (same file or separate taxonomy files):
- `ScanRequest` — target, mode, filter, member, format
- `ScanMode` — Check, Scan, Ci { threshold }
- `ScanReport` — results, diagnostics, score
- `PipelineDiagnostic` — source, message, severity
- `PipelineError` — PathNotFound, InvalidPath, WorkspaceDiscovery, Analysis, Io

**Module registration**: Add to `crates/shared/src/cli-commands/mod.rs`.

---

### P4.2 — Create AnalysisPipelineOrchestrator agent

**Skill**: `create-agent-rust` — orchestration only, zero I/O, zero business logic, depends on Contract only. Agent must NOT format output, print to stdout, or do filesystem operations. File collection must be delegated to a capabilities layer.
**File**: `crates/cli-commands/src/agent_analysis_pipeline_orchestrator.rs` (NEW)
**Severity**: HIGH
**AES Code**: Architecture

**Agent**:
```rust
pub struct AnalysisPipelineOrchestrator {
    code_analysis: Arc<dyn ICodeAnalysisAggregate>,
    naming_rules: Arc<dyn INamingRulesAggregate>,
    import_rules: Arc<dyn IImportRulesAggregate>,
    role_rules: Arc<dyn IRoleRulesAggregate>,
    external_lint: Arc<dyn IExternalLintAggregate>,
    orphan_detector: Arc<dyn IOrphanAggregate>,
}

impl IAnalysisPipelineAggregate for AnalysisPipelineOrchestrator {
    async fn run(&self, request: ScanRequest) -> Result<ScanReport, PipelineError> {
        // Orchestrate: collect files → run all linters → merge results → score
        // No I/O, no formatting, no printing
    }
}
```

**Module registration**: Add `pub mod agent_analysis_pipeline_orchestrator;` to `crates/cli-commands/src/mod.rs`.

---

### P4.3 — Rename formatter files from agent_ to capabilities_ (AES compliance)

**Skill**: `create-capabilities-rust` — formatters are single-responsibility (formatting), not orchestration. Must be capabilities, not agents.
**Files**: Rename existing files in `crates/cli-commands/src/`
**Severity**: MEDIUM
**AES Code**: Architecture (Agent doing capabilities work)
**Problem**: `agent_text_formatter.rs`, `agent_json_formatter.rs`, `agent_sarif_formatter.rs`, `agent_junit_formatter.rs` are named `agent_` but implement `IReportFormatterProtocol` — single responsibility, not orchestration. Violates AES layer rules.

**Fix**: Rename files:
- `agent_text_formatter.rs` → `capabilities_text_formatter.rs`
- `agent_json_formatter.rs` → `capabilities_json_formatter.rs`
- `agent_sarif_formatter.rs` → `capabilities_sarif_formatter.rs`
- `agent_junit_formatter.rs` → `capabilities_junit_formatter.rs`

Update `mod.rs` declarations accordingly. No code changes needed inside files — they already implement the protocol correctly.

**Note**: `contract_report_formatter_protocol.rs` already exists in shared. No new contract file needed.

---

### P4.4 — Thin out CheckCommandsSurface

**Skill**: `create-surface-rust` — surface must be thin, delegate to aggregates.
**File**: `crates/cli-commands/src/surface_check_command.rs`
**Severity**: HIGH
**AES Code**: Architecture

**Before**: Surface contains orchestration, filtering, formatting, scoring.
**After**: Surface calls `IAnalysisPipelineAggregate::run()` and `IReportFormatterProtocol::format()`.

```rust
pub async fn handle_scan(
    pipeline: Arc<dyn IAnalysisPipelineAggregate>,
    formatter: Box<dyn IReportFormatterProtocol>,
    path: Option<String>,
    // ...
) -> ExitCode {
    let request = ScanRequest { target, mode: ScanMode::Scan, filter, member, format };
    match pipeline.run(request).await {
        Ok(report) => {
            println!("{}", formatter.format(&report.results.values, &context));
            if report.violation_count() > 0 { ExitCode::from(1) } else { ExitCode::SUCCESS }
        }
        Err(err) => { eprintln!("Error: {err}"); ExitCode::from(2) }
    }
}
```

---

## Phase 5: Error Handling Improvements (MEDIUM)

### P5.1 — Fix maintenance commands returning success on failure

**Skill**: `create-surface-rust` — surface must return correct exit codes.
**File**: `crates/cli-commands/src/surface_maintenance_command.rs`
**Severity**: MEDIUM
**AES Code**: Error handling
**Problem**: `handle_security` returns success when tool is missing.

**Fix**: Return `ExitCode::from(3)` when tool is missing.

```rust
if !report.tool_installed {
    eprintln!("Error: {} is not installed.", report.tool_name);
    return ExitCode::from(3);
}
```

---

### P5.2 — Add config-show secret redaction

**Skill**: `create-surface-rust` — surface must not leak secrets.
**File**: `crates/cli-commands/src/surface_setup_command.rs`
**Severity**: MEDIUM
**AES Code**: Security
**Problem**: `handle_config_show()` prints raw config content, may leak tokens/API keys.

**Fix**: Add redaction for known secret fields.

```rust
fn redact_config(raw: &str) -> String {
    raw.lines().map(|line| {
        let lower = line.to_lowercase();
        if lower.contains("token") || lower.contains("secret")
            || lower.contains("password") || lower.contains("api_key") {
            if let Some((key, _)) = line.split_once(':') {
                return format!("{key}: [REDACTED]");
            }
        }
        line.to_string()
    }).collect::<Vec<_>>().join("\n")
}
```

---

### P5.3 — Standardize exit codes across all commands

**Skill**: `create-surface-rust` — surface must follow POSIX exit code convention.
**File**: Multiple surface files
**Severity**: MEDIUM
**AES Code**: Error handling

**Convention**:
- `0` = success, no violations
- `1` = violations/findings found
- `2` = system/operational error
- `3` = required tool missing

Update all surface handlers to follow this convention.

---

## Phase 6: Command Catalog Consolidation (MEDIUM)

### P6.1 — Unify command catalog to single source of truth

**Skill**: `create-taxonomy-rust` — taxonomy defines constants, no behavior.
**File**: `crates/shared/src/cli-commands/taxonomy_catalog_constant.rs`
**Severity**: MEDIUM
**AES Code**: Architecture
**Problem**: `COMMAND_CATALOG` and `CommandCatalogVO::command_catalog()` are duplicated and inconsistent.

**Fix**: Make `CommandCatalogVO` derive from `COMMAND_CATALOG`.

```rust
impl CommandCatalogVO {
    pub fn command_catalog() -> HashMap<ActionName, CommandMetadataVO> {
        COMMAND_CATALOG.iter().map(|spec| {
            (ActionName::from(spec.name), CommandMetadataVO::new(
                DescriptionVO::new(spec.description),
                Suggestion::new(spec.example),
            ))
        }).collect()
    }
}
```

---

## Phase 7: Documentation (LOW)

### P7.1 — Rewrite FRD to match actual product scope

**Skill**: `add-docs-rust` — FRD must reflect actual implementation.
**File**: FRD document
**Severity**: LOW

**Action**: Update FRD with:
- Actual command list (check, scan, fix, ci, orphan, security, dependencies, doctor, init, install, mcp-config, config-show, watch)
- Remove `tui` and `duplicates` from FRD
- Add `check` vs `scan` semantic definitions
- Add `fix` safety boundaries (which rules are auto-fixable)
- Add NFRs: SARIF support, exit codes, cross-platform

---

### P7.2 — Document config resolution algorithm

**Skill**: `add-docs-rust` — documenting implicit behavior.
**File**: FRD document
**Severity**: LOW

**Action**: Document priority chain:
1. Project-root YAML
2. Parent dir (depth ≤ 3)
3. XDG user config
4. XDG system dirs
5. Embedded defaults

---

## Execution Order

1. **Phase 1** (P1.1-P1.2): Security — remove dead CLI, remove --global init.
   - **Verify:** `cargo check -p shared && cargo check -p cli-commands`
2. **Phase 2** (P2.1-P2.11): Critical bug fixes. Can run in parallel with Phase 1.
   - **Verify:** `cargo check -p shared && cargo check -p cli-commands`
3. **Phase 3** (P3.1-P3.3): Performance fixes.
   - **Verify:** `cargo check -p shared && cargo check -p cli-commands`
4. **Phase 4** (P4.1-P4.4): Architecture refactor. Depends on Phase 2 (bug fixes stable).
   - **Verify:** `cargo check -p shared && cargo check -p cli-commands`
5. **Phase 5** (P5.1-P5.3): Error handling. Independent.
   - **Verify:** `cargo check -p cli-commands`
6. **Phase 6** (P6.1): Command catalog. Independent.
   - **Verify:** `cargo check -p shared`
7. **Phase 7** (P7.1-P7.2): Documentation. Independent.

**Final verification (all phases complete):**
```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test --workspace
cargo run --bin lint-arwaky-cli -- check .
```

---

## Files Summary

### New files (2)
- `crates/shared/src/cli-commands/contract_analysis_pipeline_aggregate.rs` — pipeline contract (P4.1)
- `crates/cli-commands/src/agent_analysis_pipeline_orchestrator.rs` — pipeline agent (P4.2)

### Renamed files (4)
- `agent_text_formatter.rs` → `capabilities_text_formatter.rs` (P4.3)
- `agent_json_formatter.rs` → `capabilities_json_formatter.rs` (P4.3)
- `agent_sarif_formatter.rs` → `capabilities_sarif_formatter.rs` (P4.3)
- `agent_junit_formatter.rs` → `capabilities_junit_formatter.rs` (P4.3)

### Modified files (12)
- `crates/shared/src/common/utility_file.rs` — fix walker, add ignore dirs (P2.1, P3.2)
- `crates/shared/src/cli-commands/taxonomy_cli_vo.rs` — remove Duplicates, remove --global flag (P1.1, P1.2)
- `crates/shared/src/cli-commands/taxonomy_catalog_constant.rs` — unify catalog (P6.1)
- `crates/shared/src/cli-commands/mod.rs` — register contract_analysis_pipeline_aggregate (P4.1)
- `crates/cli-commands/src/surface_check_command.rs` — fix output, path filtering, errors (P2.2-P2.4, P2.8-P2.10, P3.1, P4.4)
- `crates/cli-commands/src/surface_check_action.rs` — fix git-diff path/filter (P2.5)
- `crates/cli-commands/src/surface_git_command.rs` — fix git-diff signature (P2.5)
- `crates/cli-commands/src/surface_plugin_command.rs` — fix adapter listing (P2.6)
- `crates/cli-commands/src/surface_common_command.rs` — fix CI threshold (P2.7)
- `crates/cli-commands/src/surface_maintenance_command.rs` — fix exit codes (P2.11, P5.1)
- `crates/cli-commands/src/surface_setup_command.rs` — remove --global, remove TRUSTED_DOCS, fix config-show (P1.2, P5.2)
- `crates/cli-commands/src/mod.rs` — register agent_analysis_pipeline_orchestrator, rename agent_*_formatter to capabilities_*_formatter (P4.2, P4.3)

### Deleted files (1)
- `crates/cli-commands/src/assets/` — bundled docs no longer needed (P1.2)

---

## Summary

| Phase | Items | Severity | Description |
|-------|-------|----------|-------------|
| 1 | P1.1-P1.2 | CRITICAL/HIGH | Security: remove dead CLI, remove --global init |
| 2 | P2.1-P2.11 | CRITICAL/HIGH | Bug fixes: walker, output, path filtering, git-diff, adapters, CI, XML, errors |
| 3 | P3.1-P3.3 | HIGH/MEDIUM | Performance: ignore rules, ignore list, lazy container |
| 4 | P4.1-P4.4 | HIGH/MEDIUM | Architecture: IAnalysisPipelineAggregate, rename formatters to capabilities, thin surface |
| 5 | P5.1-P5.3 | MEDIUM | Error handling: exit codes, secret redaction, standardization |
| 6 | P6.1 | MEDIUM | Command catalog consolidation |
| 7 | P7.1-P7.2 | LOW | FRD documentation updates |

**Total**: 29 items across 7 phases.
