# Backend Review Report — `import-rules` v1.10.106

Date: 2026-07-21
Reviewer: Expert Backend Review
Crate: `import_rules-lint-arwaky`
Version: `1.10.106`
Scope: Rust implementation of AES201–AES205 import-rule enforcement.

---

## 1. Executive Summary

The `import-rules` crate enforces AES201–AES205 import compliance across Rust, Python, and JavaScript/TypeScript codebases. The architecture follows a clear layered structure:

- `root_` wiring (`ImportContainer`)
- `agent_` orchestration (`ImportOrchestrator`)
- `capabilities_` rule execution (5 checkers)
- `contract_` protocols (in shared)
- `taxonomy_` value objects (in shared)
- `utility_` helpers (in shared)

The implementation is well-structured with clean protocol separation and proper delegation to utility functions. However, several backend-quality issues exist:

- **correctness**: `DummyImportChecker` creates default `LayerMapVO` in every method, ignoring actual configuration
- **reliability**: `run_audit` returns `Vec<LintResult>` instead of `Result`, hiding scan failures
- **maintainability**: `_check_aggregate_intent` is dead code (never called), `str_or` lives in the agent layer
- **performance**: blocking filesystem I/O inside async `run_audit`
- **security**: no path traversal, symlink, or max-depth protection in `walk_dir`

---

## 2. Findings

| Severity | Area            | Issue                                                                                      | Impact                                                                |
| -------- | --------------- | ------------------------------------------------------------------------------------------ | --------------------------------------------------------------------- |
| High     | Correctness     | `DummyImportChecker` creates `LayerMapVO::default()` in every protocol method              | Ignores actual layer config; dummy checks run against wrong layers    |
| High     | Reliability     | `run_audit` returns `Vec<LintResult>` instead of `Result`                                  | Scan failures (permission denied, missing path) are silently lost     |
| High     | Maintainability | `_check_aggregate_intent` is defined but never called                                      | Dead code increases maintenance burden and confuses readers           |
| Medium   | Performance     | Blocking `std::fs::read_dir` and `std::fs::read_to_string` in async context                | Can block the Tokio runtime during large scans                        |
| Medium   | Security        | `walk_dir` has no symlink, max-depth, or workspace confinement                             | Potential DoS or workspace escape via symlink chains                  |
| Medium   | Maintainability | `str_or` utility function lives in agent layer                                             | Violates layer boundaries; should be in utility layer                 |
| Medium   | Correctness     | `walk_dir` only checks `is_ignored` on directories, not files                              | Ignored file patterns are not respected                               |
| Medium   | Reliability     | `ImportOrchestrator::with_config` creates new empty checkers, ignoring config              | Config-driven checker selection is not supported                      |
| Low      | Correctness     | `walk_dir` does not skip `.git`, `node_modules`, `target`, `dist`                          | Unnecessary files are scanned, wasting time and producing noise       |
| Low      | Performance     | `collect_files` materializes all file paths into memory                                    | Poor scalability for very large repositories                          |
| Low      | Maintainability | `tokio::join!` parallelism is inconsistent (mandatory+forbidden parallel, rest sequential) | Unclear parallelism strategy; unused+cycle could also run in parallel |

---

## 3. Detailed Review

### 3.1 High: `DummyImportChecker` Ignores Layer Configuration

Every protocol method in `DummyImportChecker` creates a default layer map:

```rust
fn check_dummy_imports(&self, ...) {
    let layer_map = shared::taxonomy_definition_vo::LayerMapVO::default();
    self._check_dummy_imports(file.value(), content.value(), violations, &layer_map);
}
```

This means the dummy checker always runs against the hardcoded default layers, not the user's actual configuration. If a project customizes its layer definitions, AES204 checks will produce incorrect results.

**Recommended fix**: Accept `LayerMapVO` as a parameter in the protocol methods, or store it in the struct at construction time via the container.

---

### 3.2 High: `run_audit` Cannot Report Failures

Current contract:

```rust
async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>;
```

This cannot distinguish between:

- no violations found
- target path missing
- permission denied
- invalid configuration
- filesystem failure

Recommended contract:

```rust
async fn run_audit(
    &self,
    target: &FilePath,
) -> Result<Vec<LintResult>, ScanError>;
```

---

### 3.3 High: Dead Code — `_check_aggregate_intent`

The method `_check_aggregate_intent` (line 324) is fully implemented but never called from any protocol method. It detects phantom aggregate types but has no entry point.

**Recommended fix**: Either wire it into `check_dummy_imports` or `check_taxonomy_intent`, or remove it entirely.

---

### 3.4 Medium: Blocking Filesystem I/O in Async Context

`run_audit` is an `async fn` but calls:

- `std::fs::read_dir` in `walk_dir` (line 120)
- `std::fs::read_to_string` in the unused-import loop (line 186)

Both are blocking operations that can stall the Tokio runtime.

**Recommended fix**: Use `tokio::fs::read_dir` and `tokio::fs::read_to_string`, or wrap blocking calls in `tokio::task::spawn_blocking`.

---

### 3.5 Medium: No Path Safety in `walk_dir`

The `walk_dir` method (line 119) has no protection against:

- symbolic links (can escape the target directory)
- deeply nested directories (can cause stack overflow or excessive memory)
- workspace confinement (can scan outside the intended project)

**Recommended fix**: Use `walkdir` crate with symlink protection and max-depth, or add manual guards.

---

### 3.6 Medium: `str_or` Lives in Agent Layer

```rust
pub fn str_or<'a>(opt: Option<&'a str>, fallback: &'a str) -> &'a str {
    opt.map_or(fallback, |s| s)
}
```

This is a stateless utility function placed in the agent orchestrator file. It should be in the utility layer.

**Recommended fix**: Move to `shared/src/common/utility_string.rs` or similar.

---

### 3.7 Medium: `walk_dir` Ignores Files, Not Directories

The `is_ignored` check only applies to directories:

```rust
if path.is_dir() {
    if is_subdir && self.is_ignored(&path) {
        continue;
    }
```

Individual files matching ignored patterns are still scanned.

**Recommended fix**: Also check `is_ignored` for files before processing.

---

### 3.8 Medium: `with_config` Creates Empty Checkers

```rust
pub fn with_config(config: ArchitectureConfig) -> Self {
    // ...
    let empty_mandatory: Arc<dyn IImportMandatoryProtocol> = Arc::new(
        crate::capabilities_import_mandatory_checker::ArchImportMandatoryChecker::new(),
    );
    // ... same for forbidden, unused, cycle
```

The `with_config` constructor always creates fresh default checkers regardless of the config. There is no mechanism to select different checker implementations based on configuration.

---

### 3.9 Low: `walk_dir` Does Not Skip Build Directories

Common directories like `.git`, `node_modules`, `target`, `dist`, `build`, `.venv` are not explicitly skipped. While `is_ignored` may catch some via config, the defaults do not exclude them.

---

### 3.10 Low: Inconsistent Parallelism in `run_audit`

`mandatory` and `forbidden` run in parallel via `tokio::join!`, but `unused` and `cycle` run sequentially. The `unused` check reads files one-by-one in a loop, and `cycle` could also run in parallel with `unused`.

**Recommended future improvement**: Consider running all four checks concurrently or using a structured pipeline.

---

## 4. SOLID Assessment

### Single Responsibility Principle

Each checker handles one AES rule (AES201–AES205). The orchestrator coordinates without containing business logic. This is well-applied.

### Open/Closed Principle

Adding a new import rule requires: a new protocol, a new capability, and wiring in the container. The architecture is open for extension via protocol addition.

### Liskov Substitution Principle

`DummyImportChecker` creates default `LayerMapVO` in every method, which means it behaves differently from other checkers that use the actual config. This is a Liskov smell — it cannot be substituted transparently.

### Interface Segregation Principle

Protocols are split per-rule (`IImportMandatoryProtocol`, `IImportForbiddenProtocol`, etc.). This is well-applied.

### Dependency Inversion Principle

The orchestrator depends on protocol abstractions, not concrete implementations. The container wires concrete to abstract. This is well-applied.

---

## 5. Patch Scope

Recommended fixes for this round:

- fix `DummyImportChecker` to accept `LayerMapVO` from config
- make `run_audit` return `Result<Vec<LintResult>, ScanError>`
- remove or wire `_check_aggregate_intent`
- move `str_or` to utility layer
- add `is_ignored` check for files in `walk_dir`
- skip common build directories by default

Follow-up work (separate PRs):

- async-safe filesystem traversal (`spawn_blocking` or `tokio::fs`)
- secure filesystem walker with symlink protection
- streaming file iteration
- config-driven checker selection in `with_config`

---

## 6. Validation Plan

Run:

```bash
cargo check --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

Self-lint:

```bash
cargo run --bin lint-arwaky-cli -- check .
```

Expected: `Total violations: 0`

Scan test projects:

```bash
cargo run --bin lint-arwaky-cli -- scan test-workspaces/crates for rust
cargo run --bin lint-arwaky-cli -- scan test-workspaces/modules for python
cargo run --bin lint-arwaky-cli -- scan test-workspaces/packages for typescript
```

---

## 7. Final Recommendation

Merge fixes for the high-severity correctness and reliability issues. The `DummyImportChecker` layer-map issue and the `run_audit` error propagation are the most impactful. Dead code removal (`_check_aggregate_intent`) and the `str_or` relocation are low-risk cleanups that should be included.

Defer async filesystem traversal and secure walker to follow-up PRs since they affect dependencies and runtime behavior.
