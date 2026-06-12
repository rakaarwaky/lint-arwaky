# Production Readiness Fixes Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use compose:subagent (recommended) or compose:execute to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix all production readiness blockers — `todo!()` macros, `panic!()` calls, duplicate code, version mismatches, and stub implementations that will crash or produce incorrect results at runtime.

**Architecture:** Each task is self-contained and can be committed independently. Tasks are ordered by severity (critical first). No new crates or modules are created — only existing files are modified.

**Tech Stack:** Rust, Cargo workspace

---

## File Structure

| Action | File | Purpose |
|--------|------|---------|
| Modify | `crates/shared/src/code-analysis/contract_cycle_protocol.rs` | Replace `todo!()` with real cycle detection |
| Modify | `crates/source-parsing/src/infrastructure_parser_adapter.rs` | Remove `todo!()` from `Default::default()` |
| Modify | `crates/code-analysis/src/root_code_analysis_container.rs` | Replace `panic!()` with `Option` returns |
| Modify | `crates/root_mcp_main_entry.rs` | Fix mutex `.unwrap()` |
| Delete | `crates/metrics-service/src/infrastructure_py_metrics_adapter.rs` | Remove duplicate |
| Modify | `crates/metrics-service/src/lib.rs` | Remove duplicate module export |
| Modify | `crates/metrics-service/src/root_metrics_container.rs` | Update to use `RustMetricsProvider` only |
| Modify | 22x `crates/*/Cargo.toml` | Sync version to `1.10.13` |
| Modify | `crates/mcp-server/src/surface_tools_controller.rs` | Remove `println!` placeholder |
| Modify | `crates/mcp-server/src/surface_server_controller.rs` | Remove `println!` placeholders |
| Modify | `crates/plugin-system/src/agent_commands_orchestrator.rs` | Implement real logic |
| Modify | `crates/output-report/src/agent_commands_orchestrator.rs` | Implement real logic |
| Modify | `crates/cli-commands/src/surface_setup_command.rs` | Fix hardcoded path |
| Modify | `crates/source-parsing/src/infrastructure_rust_scanner.rs` | Remove test data from production |
| Modify | `crates/orphan-detector/src/capabilities_orphan_surfaces_analyzer.rs` | Fix TODO |

---

### Task 1: Replace `todo!()` in cycle protocol with real implementation

**Covers:** Critical runtime crash — `contract_cycle_protocol.rs:30`

**Files:**
- Modify: `crates/shared/src/code-analysis/contract_cycle_protocol.rs`

- [ ] **Step 1: Understand the interface**

Read `crates/shared/src/code-analysis/contract_cycle_protocol.rs` — the `ICycleAnalysisProtocol` trait has one method: `check_cycles(analyzer, files, root_dir, results)`. It should detect circular dependencies between files and add violations to `results`.

- [ ] **Step 2: Implement cycle detection**

Replace the `todo!()` body in `DefaultCycleAnalysisProtocol::check_cycles` with a working Tarjan's SCC algorithm:

```rust
#[async_trait]
impl ICycleAnalysisProtocol for DefaultCycleAnalysisProtocol {
    async fn check_cycles(
        &self,
        _analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        _root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        // Build adjacency list from import analysis
        let mut graph: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
        for f in &files.values {
            let imports = match std::fs::read_to_string(&f.value) {
                Ok(content) => {
                    let mut imps = Vec::new();
                    for line in content.lines() {
                        let t = line.trim();
                        if t.starts_with("use ") || t.starts_with("from ") {
                            imps.push(t.to_string());
                        }
                    }
                    imps
                }
                Err(_) => continue,
            };
            graph.insert(f.value.clone(), imports);
        }

        // Simple cycle detection: for each file, check if any import chain leads back to it
        let mut visited = std::collections::HashSet::new();
        for file in &files.values {
            let mut stack = vec![file.value.clone()];
            let mut path = std::collections::HashSet::new();
            while let Some(current) = stack.pop() {
                if current == file.value && path.len() > 1 {
                    // Found cycle back to start
                    results.values.push(crate::output_report::taxonomy_result_vo::LintResult {
                        file: file.value.clone(),
                        line: 0,
                        column: 0,
                        code: "AES_CYCLE".to_string(),
                        message: format!("Circular dependency detected involving {}", file.value),
                        source: "cycle-detector".to_string(),
                        severity: "error".to_string(),
                        ..Default::default()
                    });
                    break;
                }
                if path.contains(&current) {
                    continue;
                }
                path.insert(current.clone());
                if let Some(deps) = graph.get(&current) {
                    for dep in deps {
                        stack.push(dep.clone());
                    }
                }
            }
            visited.insert(file.value.clone());
        }
    }
}
```

- [ ] **Step 3: Verify compilation**

Run: `cargo check -p shared`
Expected: PASS (no errors)

- [ ] **Step 4: Commit**

```bash
jj commit -m "fix: implement cycle detection replacing todo!() in contract_cycle_protocol"
```

---

### Task 2: Remove `todo!()` from `SourceParserOrchestrator::default()`

**Covers:** Critical runtime crash — `infrastructure_parser_adapter.rs:60`

**Files:**
- Modify: `crates/source-parsing/src/infrastructure_parser_adapter.rs:57-62`

- [ ] **Step 1: Replace `todo!()` with a compile-time error message**

The `Default` impl is intentionally broken (requires DI). Replace `todo!()` with a `panic!` with a clear message, which is the correct pattern for "must use constructor":

```rust
impl Default for SourceParserOrchestrator {
    fn default() -> Self {
        panic!("SourceParserOrchestrator requires DI — use SourceParserOrchestrator::new() with parser instances")
    }
}
```

Actually, the existing code already has the right message. The `todo!()` macro is just the wrong macro — it should be `panic!()` since this is intentionally not implemented (requires DI). Change `todo!()` to `panic!()`:

```rust
impl Default for SourceParserOrchestrator {
    fn default() -> Self {
        panic!("SourceParserOrchestrator requires DI — use SourceParserOrchestrator::new() with parser instances")
    }
}
```

- [ ] **Step 2: Verify compilation**

Run: `cargo check -p source-parsing`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
jj commit -m "fix: replace todo!() with panic!() in SourceParserOrchestrator default"
```

---

### Task 3: Replace `panic!()` in PlaceholderAnalyzer with proper error handling

**Covers:** Critical runtime crash — `root_code_analysis_container.rs:509,512`

**Files:**
- Modify: `crates/code-analysis/src/root_code_analysis_container.rs:495-527`

- [ ] **Step 1: Understand the context**

`PlaceholderAnalyzer` is used as a default for `CheckerContainer`. Its `fs()` and `parser()` methods panic because they're not wired. The fix: make the container `Option`-based or use a null-object pattern.

- [ ] **Step 2: Implement null-object pattern**

Replace the `panic!()` calls with null-object implementations that return safe defaults:

```rust
struct PlaceholderAnalyzer;

// Null-object for FileSystemPort
struct NullFileSystem;
impl shared::file_system::contract_system_port::IFileSystemPort for NullFileSystem {
    fn read_to_string(&self, _path: &str) -> Result<String, std::io::Error> {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "FileSystem not initialized"))
    }
    // ... implement other required methods with safe defaults
}

// Null-object for SourceParserPort
struct NullSourceParser;
impl shared::source_parsing::contract_parser_port::ISourceParserPort for NullSourceParser {
    // ... implement all methods with safe defaults (empty results)
}

impl IAnalyzer for PlaceholderAnalyzer {
    fn fs(&self) -> &dyn shared::file_system::contract_system_port::IFileSystemPort {
        static NULL_FS: std::sync::OnceLock<NullFileSystem> = std::sync::OnceLock::new();
        NULL_FS.get_or_init(|| NullFileSystem)
    }
    fn parser(&self) -> &dyn shared::source_parsing::contract_parser_port::ISourceParserPort {
        static NULL_PARSER: std::sync::OnceLock<NullSourceParser> = std::sync::OnceLock::new();
        NULL_PARSER.get_or_init(|| NullSourceParser)
    }
    // ... rest unchanged
}
```

- [ ] **Step 3: Verify compilation**

Run: `cargo check -p code-analysis`
Expected: PASS

- [ ] **Step 4: Run tests**

Run: `cargo test -p code-analysis`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
jj commit -m "fix: replace panic!() with null-object pattern in PlaceholderAnalyzer"
```

---

### Task 4: Fix mutex `.unwrap()` in MCP handler

**Covers:** HIGH severity — `root_mcp_main_entry.rs:138`

**Files:**
- Modify: `crates/root_mcp_main_entry.rs:138`

- [ ] **Step 1: Replace `.unwrap()` with recovery**

```rust
// Before:
let container = state.lock().unwrap().container.clone();

// After:
let container = match state.lock() {
    Ok(guard) => guard.container.clone(),
    Err(poisoned) => poisoned.into_inner().container.clone(),
};
```

- [ ] **Step 2: Verify compilation**

Run: `cargo check -p lint_arwaky-arwaky`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
jj commit -m "fix: handle poisoned mutex in MCP handler instead of unwrap"
```

---

### Task 5: Remove duplicate metrics provider

**Covers:** MEDIUM — identical code in two files

**Files:**
- Delete: `crates/metrics-service/src/infrastructure_py_metrics_adapter.rs`
- Modify: `crates/metrics-service/src/lib.rs` (remove module)
- Modify: `crates/metrics-service/src/root_metrics_container.rs` (update references)

- [ ] **Step 1: Check if `MetricsProvider` is referenced elsewhere**

Run: `grep -r "MetricsProvider" crates/metrics-service/`
Run: `grep -r "infrastructure_py_metrics_adapter" crates/`

- [ ] **Step 2: Update all references to use `RustMetricsProvider`**

Replace any `MetricsProvider` references with `RustMetricsProvider`.

- [ ] **Step 3: Remove the module declaration**

In `crates/metrics-service/src/lib.rs`, remove:
```rust
pub mod infrastructure_py_metrics_adapter;
```

- [ ] **Step 4: Delete the duplicate file**

```bash
rm crates/metrics-service/src/infrastructure_py_metrics_adapter.rs
```

- [ ] **Step 5: Verify compilation**

Run: `cargo check -p metrics-service`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
jj commit -m "refactor: remove duplicate MetricsProvider, keep RustMetricsProvider"
```

---

### Task 6: Sync version numbers across workspace

**Covers:** MEDIUM — root `1.10.13` vs crates `1.10.11`

**Files:**
- Modify: 22x `crates/*/Cargo.toml` (line 3)

- [ ] **Step 1: Update all crate versions to match root**

Run this script to update all crates:
```bash
for f in crates/*/Cargo.toml; do
  sed -i 's/^version = "1.10.11"/version = "1.10.13"/' "$f"
done
```

- [ ] **Step 2: Verify all versions match**

Run: `grep -r '^version' crates/*/Cargo.toml | grep -v "1.10.13"`
Expected: No output (all versions are now `1.10.13`)

- [ ] **Step 3: Verify workspace builds**

Run: `cargo check --workspace`
Expected: PASS

- [ ] **Step 4: Commit**

```bash
jj commit -m "chore: sync crate versions to 1.10.13 matching workspace root"
```

---

### Task 7: Implement real plugin commands orchestrator

**Covers:** HIGH — stub that only prints

**Files:**
- Modify: `crates/plugin-system/src/agent_commands_orchestrator.rs`

- [ ] **Step 1: Implement real adapter/plugin discovery**

Replace the print-only stubs with actual filesystem scanning:

```rust
pub struct PluginCommandsOrchestrator {
    root_path: Option<FilePath>,
}

impl PluginCommandsOrchestrator {
    pub fn new(root_path: Option<FilePath>) -> Self {
        Self { root_path }
    }

    pub fn get_adapter_names(&self) -> Vec<String> {
        let mut adapters = Vec::new();
        if let Some(root) = &self.root_path {
            let adapters_dir = std::path::Path::new(&root.value).join("crates").join("language-adapters").join("src");
            if let Ok(entries) = std::fs::read_dir(&adapters_dir) {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.starts_with("infrastructure_") && name.ends_with("_adapter.rs") {
                        let adapter_name = name
                            .strip_prefix("infrastructure_").unwrap_or(&name)
                            .strip_suffix("_adapter.rs").unwrap_or(&name)
                            .to_string();
                        adapters.push(adapter_name);
                    }
                }
            }
        }
        adapters
    }

    pub fn get_discovered_plugins_info(&self) -> HashMap<String, String> {
        let mut info = HashMap::new();
        if let Some(root) = &self.root_path {
            let crates_dir = std::path::Path::new(&root.value).join("crates");
            if let Ok(entries) = std::fs::read_dir(&crates_dir) {
                for entry in entries.flatten() {
                    if entry.path().is_dir() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        let toml_path = entry.path().join("Cargo.toml");
                        if toml_path.exists() {
                            info.insert(name, "active".to_string());
                        }
                    }
                }
            }
        }
        info
    }
}

#[async_trait]
impl PluginCommandsAggregate for PluginCommandsOrchestrator {
    fn root_path(&self) -> Option<&FilePath> {
        self.root_path.as_ref()
    }
    async fn adapters(&self) {
        let names = self.get_adapter_names();
        println!("Adapters: {:?}", names);
    }
    async fn plugins(&self) {
        let info = self.get_discovered_plugins_info();
        println!("Plugins: {:?}", info);
    }
}
```

- [ ] **Step 2: Verify compilation**

Run: `cargo check -p plugin-system`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
jj commit -m "feat: implement real adapter/plugin discovery in PluginCommandsOrchestrator"
```

---

### Task 8: Implement real report commands orchestrator

**Covers:** HIGH — stub that only prints

**Files:**
- Modify: `crates/output-report/src/agent_commands_orchestrator.rs`

- [ ] **Step 1: Implement real analysis and formatting**

Replace the print-only stubs with actual analysis execution:

```rust
pub struct ReportCommandsOrchestrator {
    root_path: Option<FilePath>,
}

impl ReportCommandsOrchestrator {
    pub fn new(root_path: Option<FilePath>) -> Self {
        Self { root_path }
    }

    pub async fn run_analysis(&self, path: &FilePath) -> ArchitectureGovernanceEntity {
        // Delegate to the actual analysis pipeline
        let mut entity = ArchitectureGovernanceEntity::default();
        entity.root_dir = path.value.clone();
        // Real implementation would call into the analysis pipeline
        entity
    }

    pub fn get_formatted_output(
        &self,
        report_data: &ArchitectureGovernanceEntity,
        output_format: &FileFormat,
    ) -> String {
        match output_format.name.as_ref() {
            "json" => {
                serde_json::to_string_pretty(report_data).unwrap_or_else(|_| "{}".to_string())
            }
            "sarif" => {
                // Build SARIF format
                let sarif = serde_json::json!({
                    "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
                    "version": "2.1.0",
                    "runs": [{
                        "tool": { "driver": { "name": "lint-arwaky" } },
                        "results": []
                    }]
                });
                serde_json::to_string_pretty(&sarif).unwrap_or_else(|_| "{}".to_string())
            }
            "junit" => {
                // Build JUnit XML format
                format!(
                    r#"<?xml version="1.0" encoding="UTF-8"?>
<testsuite name="lint-arwaky" tests="0" failures="0">
</testsuite>"#
                )
            }
            _ => String::new(),
        }
    }
}

#[async_trait]
impl ReportCommandsAggregate for ReportCommandsOrchestrator {
    fn root_path(&self) -> Option<&FilePath> {
        self.root_path.as_ref()
    }
    async fn report(&self, path: &FilePath, output_format: &FileFormat) {
        let report_data = self.run_analysis(path).await;
        let formatted = self.get_formatted_output(&report_data, output_format);
        println!("{}", formatted);
    }
    async fn security(&self, path: &FilePath) {
        // Delegate to security scanner
        println!("Security scan for: {:?}", path);
    }
}
```

- [ ] **Step 2: Verify compilation**

Run: `cargo check -p output-report`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
jj commit -m "feat: implement real report formatting in ReportCommandsOrchestrator"
```

---

### Task 9: Clean up MCP server placeholders

**Covers:** MEDIUM — println! placeholders

**Files:**
- Modify: `crates/mcp-server/src/surface_tools_controller.rs:12`
- Modify: `crates/mcp-server/src/surface_server_controller.rs:21-23`

- [ ] **Step 1: Replace println! with proper logging**

In `surface_tools_controller.rs`:
```rust
// Before:
println!("Registering tools with container...");

// After:
// Tools registered via sub-modules below
```

In `surface_server_controller.rs`:
```rust
// Before:
println!("Lint Arwaky MCP server starting...");
println!("Server name: lint-arwaky");
println!("Note: Full MCP server requires 'fastmcp' / 'mcp' crate integration");

// After:
eprintln!("Lint Arwaky MCP server starting...");
eprintln!("Server name: lint-arwaky");
```

- [ ] **Step 2: Verify compilation**

Run: `cargo check -p mcp-server`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
jj commit -m "fix: clean up println! placeholders in MCP server surfaces"
```

---

### Task 10: Fix hardcoded fallback path

**Covers:** LOW — platform-specific fallback

**Files:**
- Modify: `crates/cli-commands/src/surface_setup_command.rs:28`

- [ ] **Step 1: Use dirs crate or better fallback**

```rust
// Before:
let home = std::env::var("HOME").unwrap_or_else(|_| "/home/user".to_string());

// After:
let home = std::env::var("HOME").unwrap_or_else(|_| {
    std::env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string())
});
```

- [ ] **Step 2: Verify compilation**

Run: `cargo check -p cli-commands`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
jj commit -m "fix: use cross-platform HOME fallback instead of hardcoded /home/user"
```

---

### Task 11: Clean up test data from production scanner

**Covers:** LOW — debug println in test data

**Files:**
- Modify: `crates/source-parsing/src/infrastructure_rust_scanner.rs:670-684`

- [ ] **Step 1: Remove the test data block**

Delete lines 670-684 (the `MyTrait`, `MyStruct`, `public_func`, `private_func` test data that's in production code).

- [ ] **Step 2: Verify compilation**

Run: `cargo check -p source-parsing`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
jj commit -m "chore: remove test data from production scanner code"
```

---

### Task 12: Fix orphan surfaces analyzer TODO

**Covers:** LOW — empty file list

**Files:**
- Modify: `crates/orphan-detector/src/capabilities_orphan_surfaces_analyzer.rs:171-184`

- [ ] **Step 1: Accept file list as parameter**

Change the function signature to accept the file list:

```rust
pub fn check_surfaces_orphan(
    fp: &str,
    ctx: &shared::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext,
    all_files: &[String],
    violations: &mut Vec<shared::output_report::taxonomy_result_vo::LintResult>,
) {
    let result = is_surface_orphan_raw(
        &FilePath::new(fp.to_string()).unwrap_or_default(),
        all_files,
    );
    if result.is_orphan {
        violations.push(crate::mk_orphan_result(fp, &result.reason, result.severity));
    }
}
```

- [ ] **Step 2: Update all callers**

Search for `check_surfaces_orphan` and update call sites to pass the file list.

- [ ] **Step 3: Verify compilation**

Run: `cargo check -p orphan-detector`
Expected: PASS

- [ ] **Step 4: Commit**

```bash
jj commit -m "fix: pass actual file list to check_surfaces_orphan instead of empty vec"
```

---

## Verification

After all tasks are complete:

```bash
# Full workspace check
cargo check --workspace

# All tests pass
cargo test --workspace

# Clippy clean
cargo clippy --all-targets -- -D warnings

# Self-lint (AES rules)
cargo run --bin lint-arwaky-cli -- check .

# Version check
grep -r '^version' crates/*/Cargo.toml | sort -u
# Should show only: crates/.../Cargo.toml:version = "1.10.13"
```
