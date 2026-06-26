# Data Flow Reference

> **How data moves through Lint Arwaky from input to output.**
> This document traces the exact code path for each major operation.

---

## Table of Contents

1. [Single File Check (`check`)](#1-single-file-check-check)
2. [Multi-Project Scan (`scan`)](#2-multi-project-scan-scan)
3. [Auto-Fix (`fix`)](#3-auto-fix-fix)
4. [CI Mode (`ci`)](#4-ci-mode-ci)
5. [MCP Tool Execution](#5-mcp-tool-execution)
6. [File Watch → Lint Pipeline](#6-file-watch--lint-pipeline)

---

## 1. Single File Check (`check`)

**Command:** `lint-arwaky-cli check .`

```
┌──────────────┐     ┌──────────────────┐     ┌─────────────────┐
│ CLI main()   │────▶│ Create containers │────▶│ Parse args      │
│ root_cli_*   │     │ ImportContainer  │     │ clap::Parser    │
│              │     │ NamingContainer  │     │                 │
│              │     │ RoleContainer    │     │                 │
│              │     │ ...              │     │                 │
└──────────────┘     └──────────────────┘     └────────┬────────┘
                                                        │
                                                        ▼
┌──────────────────────────────────────────────────────────────────┐
│ handle_check(target, context)                                   │
│ surface_check_command.rs                                        │
│                                                                 │
│  1. context.code_analysis.run_code_analysis(target)             │
│     └─▶ CodeAnalysisOrchestrator.run_audit()                   │
│         ├─▶ FileLineChecker.check()           → AES301 violations
│         ├─▶ FunctionLineChecker.check()       → (sub-rule)
│         ├─▶ BypassChecker.check()             → AES304 violations
│         ├─▶ MandatoryDefChecker.check()       → AES305 violations
│         └─▶ TodoChecker.check()               → (sub-rule)
│                                                                 │
│  2. context.naming.run_audit(target)                            │
│     └─▶ NamingOrchestrator.run_audit()                         │
│         ├─▶ NamingConventionChecker.check()   → AES102 violations
│         └─▶ SuffixPrefixChecker.check()       → AES101 violations
│                                                                 │
│  3. context.imports.run_audit(target)                           │
│     └─▶ ImportOrchestrator.run_audit()                         │
│         ├─▶ MandatoryChecker.check()          → AES202 violations
│         ├─▶ ForbiddenChecker.check()          → AES201 violations
│         ├─▶ DummyChecker.check()              → AES204 violations
│         ├─▶ UnusedChecker.check()             → AES203 violations
│         └─▶ CycleAnalyzer.check()             → AES205 violations
│                                                                 │
│  4. external_lint.run(target)                                   │
│     └─▶ ExternalLintOrchestrator.run()                         │
│         ├─▶ ClippyAdapter.run()              → Clippy violations
│         ├─▶ RustfmtAdapter.check()           → Format violations
│         └─▶ AuditAdapter.check()             → Security advisory  │
│                                                                 │
│  5. context.roles.run_audit(target)                             │
│     └─▶ RoleOrchestrator.run_audit()                           │
│         └─▶ RoleAggregateImpl.check()        → AES401-406        │
│                                                                 │
│  6. context.orphans.run_audit(target)                           │
│     └─▶ OrphanAnalyzer.run_audit()                             │
│         └─▶ ArchOrphanAnalyzer.check()       → AES501-506        │
│                                                                 │
│  7. Filter results by --code and --path prefix                  │
│  8. Format output (colored terminal)                            │
│  9. Print summary with score                                    │
└──────────────────────────────────────────────────────────────────┘
```

### Key data structures

```rust
// Input
FilePath::new(".")              // validated, normalized path

// Intermediate
Vec<FilePath>                  // collected source files

// Output
Vec<LintResult>                // all violations found
LintResult {
    file: String,              // "crates/shared/src/lib.rs"
    line: usize,               // 42
    code: String,              // "AES202"
    severity: Severity,        // Warning | Error | Info
    message: String,           // "Missing mandatory import: taxonomy_file_path_vo"
}
```

### File collection flow

```
FilePath(".")
    │
    ▼
walk_dir_recursive()
    │
    ├── Skip: target/, .git/, node_modules/, __pycache__/
    ├── Skip: paths in config.ignore[]
    ├── Skip: non-source files (.json, .yaml, .lock, ...)
    │
    ▼
Vec<FilePath>                  // filtered source files
    │
    ├── .rs files  → Rust checks
    ├── .py files  → Python checks
    └── .js/.ts files → JavaScript checks
```

---

## 2. Multi-Project Scan (`scan`)

**Command:** `lint-arwaky-cli scan test-workspaces`

```
┌──────────────┐     ┌──────────────────┐
│ CLI main()   │────▶│ Create           │
│              │     │ OrchestratorFactory│
└──────────────┘     └────────┬─────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ handle_scan(factory, target)                                    │
│ surface_scan_command.rs                                         │
│                                                                 │
│  1. Find workspace root                                         │
│     └─▶ Walk up looking for Cargo.toml / pyproject.toml /      │
│         package.json                                            │
│                                                                 │
│  2. Discover workspace members                                  │
│     └─▶ ConfigOrchestrator.discover_members()                  │
│         ├── Read Cargo.toml → [workspace.members]               │
│         ├── Read pyproject.toml → [tool.black] / src/          │
│         └── Read package.json → [workspaces.packages]           │
│                                                                 │
│  3. For each workspace member:                                  │
│     ├── factory(member_path)  → creates per-project containers  │
│     ├── run_audit(member_path) → runs all linters               │
│     ├── collect LintResult[]                                    │
│     └── aggregate into shared results                           │
│                                                                 │
│  4. Merge all results                                           │
│  5. Format & display                                            │
└─────────────────────────────────────────────────────────────────┘
```

### Workspace discovery

```
test-workspaces/
├── Cargo.toml          → members: ["crates/*"]
├── pyproject.toml      → [tool.black] / src/ = modules/
├── package.json        → workspaces: ["packages/*"]
│
├── crates/             → 7 Rust test crates
├── modules/            → 8 Python test modules
└── packages/           → 27 JS/TS test packages
```

---

## 3. Auto-Fix (`fix`)

**Command:** `lint-arwaky-cli fix .`

```
┌─────────────────────────────────────────────────────────────────┐
│ handle_fix(target, dry_run)                                     │
│                                                                 │
│  1. Run all linters → collect violations                        │
│     (same pipeline as check)                                    │
│                                                                 │
│  2. Filter fixable violations                                   │
│     └─▶ AutoFixOrchestrator.filter_fixable(violations)          │
│         ├── AES201 (forbidden import) → can remove import       │
│         ├── AES203 (unused import)    → can remove import       │
│         └── others → not fixable                                │
│                                                                 │
│  3. For each fixable violation:                                 │
│     ├── IF dry_run:                                             │
│     │   └── Print what would be fixed (no file changes)        │
│     └── ELSE:                                                   │
│         └── FixProcessor.apply_fix(violation, file_content)     │
│             ├── Parse file content                              │
│             ├── Locate violation line                           │
│             ├── Apply safe transformation                       │
│             └── Write back to file                              │
│                                                                 │
│  4. Print summary: N fixes applied / M fixes previewed         │
└─────────────────────────────────────────────────────────────────┘
```

### Fix safety rules

| Violation | Safe to fix? | Action |
|-----------|-------------|--------|
| AES201 (forbidden import) | Yes | Remove the import line |
| AES202 (missing mandatory) | No | Requires manual intervention |
| AES203 (unused import) | Yes | Remove the import line |
| AES204 (dummy import) | Yes | Remove the import line |
| AES205 (cycle) | No | Requires architectural change |
| AES301 (file too long) | No | Requires code splitting |
| AES304 (bypass) | No | Requires manual review |
| AES501-506 (orphan) | No | Requires manual deletion |

---

## 4. CI Mode (`ci`)

**Command:** `lint-arwaky-cli ci . --threshold 80`

```
┌─────────────────────────────────────────────────────────────────┐
│ handle_ci(target, threshold)                                    │
│                                                                 │
│  1. Run full lint pipeline (same as check)                      │
│                                                                 │
│  2. Calculate score                                             │
│     └─▶ CodeAnalysisOrchestrator.calc_score(violations)         │
│         score = (total_files - files_with_violations)           │
│                 / total_files * 100                             │
│                                                                 │
│  3. Generate reports                                            │
│     ├── JSON report     → ci-report.json                        │
│     ├── SARIF 2.1.0     → ci-report.sarif (GitHub compatible)  │
│     └── JUnit XML       → ci-report.xml (CI dashboards)        │
│                                                                 │
│  4. Exit code                                                   │
│     ├── score >= threshold → exit(0) ✅                         │
│     └── score < threshold  → exit(1) ❌                        │
└─────────────────────────────────────────────────────────────────┘
```

### Report formats

```json
// ci-report.json (simplified)
{
  "score": 92.5,
  "total_files": 295,
  "violations_count": 47,
  "violations": [
    {
      "file": "crates/shared/src/lib.rs",
      "line": 12,
      "code": "AES304",
      "severity": "error",
      "message": "Bypass suppression detected"
    }
  ]
}
```

---

## 5. MCP Tool Execution

**Tool:** `execute_command` via JSON-RPC 2.0

```
┌─────────────────────────────────────────────────────────────────┐
│ AI Agent sends JSON-RPC request                                 │
│ { "method": "tools/call", "params": { "name": "execute_command" }}│
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│ LintArwakyMcpServer.execute_command(params)                     │
│ mcp_server_orchestrator.rs                                      │
│                                                                 │
│  1. Parse action from params                                    │
│     └─▶ "check" | "scan" | "fix" | "ci" | "doctor" | ...       │
│                                                                 │
│  2. Dispatch to appropriate pipeline                            │
│     └─▶ tokio::task::spawn_blocking(|| {                        │
│           let rt = Runtime::new()?;                             │
│           rt.block_on(match action {                            │
│               "check" => run_check_pipeline(target),            │
│               "scan"  => run_scan_pipeline(target),             │
│               ...                                               │
│           })                                                    │
│         })                                                      │
│                                                                 │
│  3. Collect results → serialize to JSON                         │
│                                                                 │
│  4. Return JSON-RPC response                                    │
│     { "result": { "content": [{ "text": "{...json...}" }] }}   │
└─────────────────────────────────────────────────────────────────┘
```

### Why `spawn_blocking`?

The lint pipeline is **synchronous** (file I/O, regex matching). The MCP server is **async** (tokio). `spawn_blocking` bridges the two worlds without blocking the async event loop.

---

## 6. File Watch → Lint Pipeline

**Command:** `lint-arwaky-cli watch .`

```
┌─────────────────────────────────────────────────────────────────┐
│ FileWatchOrchestrator.run(target)                               │
│ file_watch_orchestrator.rs                                      │
│                                                                 │
│  1. Initialize inotify watcher                                  │
│     └─▶ notify::Watcher::new()                                  │
│         ├── IN_CREATE | IN_MODIFY | IN_DELETE                   │
│         └── Recursive for target directory                      │
│                                                                 │
│  2. Event loop                                                  │
│     loop {                                                      │
│         event = rx.recv()  // blocks until file change          │
│                                                                 │
│         3. Debounce (500ms)                                     │
│             └─▶ Skip if same file changed within 500ms          │
│                                                                 │
│         4. Filter source files only                             │
│             └─▶ .rs, .py, .js, .ts, .jsx, .tsx                 │
│                                                                 │
│         5. Run lint pipeline on changed file                    │
│             └─▶ Same as check, but single file                  │
│                                                                 │
│         6. Print results immediately                            │
│             └─▶ Colored terminal output                         │
│     }                                                           │
└─────────────────────────────────────────────────────────────────┘
```

### Event flow

```
filesystem change
    │
    ▼
inotify event (IN_MODIFY)
    │
    ▼
notify::Watcher channel
    │
    ▼
debounce (500ms)
    │
    ▼
filter (source file?)
    │
    ├── NO  → ignore
    └── YES → run lint on single file
              │
              ▼
         LintResult[]
              │
              ▼
         print to terminal
```

---

## Summary: Data Flow Patterns

| Operation | Input | Processing | Output |
|-----------|-------|------------|--------|
| `check` | `FilePath` | All linters → aggregate | `Vec<LintResult>` |
| `scan` | `FilePath` | Per-project containers → merge | `Vec<LintResult>` |
| `fix` | `FilePath` | Lint → filter fixable → apply | Modified files |
| `ci` | `FilePath` | Lint → score → reports | Exit code + files |
| `watch` | `FilePath` | inotify → debounce → lint | Live terminal output |
| MCP | JSON-RPC | Parse → dispatch → lint | JSON response |

---

## Further Reading

| Topic | Document |
|-------|----------|
| Navigation hub (start here) | [DOCS.md](DOCS.md) |
| Developer guide (patterns, how-to) | [DEVELOPER_GUIDE.md](DEVELOPER_GUIDE.md) |
| Complete crate reference | [CRATE_INDEX.md](CRATE_INDEX.md) |
| AES 7-layer architecture | [ARCHITECTURE.md](../ARCHITECTURE.md) |
| 24 AES rules catalog | [rules/RULES_AES.md](rules/RULES_AES.md) |
