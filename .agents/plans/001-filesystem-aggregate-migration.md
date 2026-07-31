# Plan: Refactor Rule Crates to Filesystem Aggregate Pattern

## Status: DRAFT
## Date: 2026-08-01
## Branch: feat/filesystem-crate

---

## 1. Problem Statement

Currently, rule crates (orphan-detector, import-rules, code-analysis, role-rules, naming-rules) call `filesystem::utility_io` directly, bypassing the `IFilesystemAggregate` contract. This creates:

- **Tight coupling**: Rule crates depend on concrete implementations, not abstractions
- **No testability**: Cannot mock filesystem for unit tests
- **Violation of AES architecture**: Capabilities should not do I/O; only orchestrators should

## 2. Target Architecture

```
Surface CLI → Contract Agent → Rule Orchestrator → IFilesystemAggregate → FilesystemOrchestrator
                                         │                                        │
                                         ▼                                        ▼
                                Rule Capabilities                          Filesystem Capabilities
                              (business logic ONLY)                       (walk, parse, extract, graph)
                                         │                                        │
                                         ▼                                        ▼
                                    LintResult                            Filesystem Utility IO
```

### Key Principles:
1. **Rule Orchestrator** owns I/O via `IFilesystemAggregate`
2. **Rule Capabilities** receive data, return results — NO I/O
3. **Filesystem Aggregate** is the ONLY way rule crates access filesystem

## 3. Current State Analysis

### 3.1 Filesystem Calls per Crate

| Crate | Agent (Orchestrator) | Capabilities | Total |
|-------|---------------------|--------------|-------|
| orphan-detector | 12 | 9 | 21 |
| import-rules | 2 | 4 | 6 |
| code-analysis | 2 | 0 | 2 |
| role-rules | 1 | 2 | 3 |
| naming-rules | 1 | 0 | 1 |
| **Total** | **18** | **15** | **33** |

### 3.2 Which Capabilities Have I/O Calls (MUST migrate)

| File | Calls | Functions Used |
|------|-------|----------------|
| `orphan_detector::capabilities_orphan_agent_analyzer.rs` | 2 | read_file_safe |
| `orphan_detector::capabilities_orphan_utility_analyzer.rs` | 1 | read_file_safe |
| `orphan_detector::capabilities_orphan_contract_analyzer.rs` | 1 | read_file_safe |
| `orphan_detector::capabilities_orphan_graph_resolver.rs` | 5 | scan_directory, is_dir |
| `import_rules::capabilities_import_mandatory_checker.rs` | 1 | read_file |
| `import_rules::capabilities_import_unused_checker.rs` | 1 | read_file |
| `import_rules::capabilities_cycle_import_analyzer.rs` | 1 | read_file |
| `import_rules::capabilities_import_forbidden_checker.rs` | 1 | read_file |
| `role_rules::capabilities_surface_role_auditor.rs` | 2 | read_file |

### 3.3 Agent Orchestrators (already have I/O, keep as-is)

| File | Calls | Functions Used |
|------|-------|----------------|
| `orphan_detector::agent_orphan_orchestrator.rs` | 12 | scan, read_file, is_dir, should_ignore, workspace_root |
| `import_rules::agent_import_orchestrator.rs` | 2 | path_exists, read_file, walk_source_files, workspace_root |
| `code_analysis::agent_code_analysis_orchestrator.rs` | 2 | read_lintable_file |
| `role_rules::agent_role_orchestrator.rs` | 1 | read_file, walk_source_files |
| `naming_rules::agent_naming_orchestrator.rs` | 1 | walk_recursive (still using old shared path) |

## 4. Migration Strategy

### Phase 1: Create Data Transfer Objects (DTOs)

Create shared types for passing data from orchestrator to capabilities:

```rust
// In shared::filesystem::taxonomy_filesystem_vo

/// Data bundle passed from orchestrator to capabilities.
/// Capabilities use this instead of calling filesystem directly.
pub struct FilesystemContext {
    pub files: Vec<FileEntry>,
    pub file_cache: DashMap<PathBuf, String>,
    pub imports: Vec<ImportEntry>,
    pub graph: DependencyGraphSnapshot,
}

/// Read-only snapshot of dependency graph for capabilities.
pub struct DependencyGraphSnapshot {
    pub dependents: HashMap<PathBuf, Vec<PathBuf>>,
    pub dependencies: HashMap<PathBuf, Vec<PathBuf>>,
    pub cycles: Vec<Vec<PathBuf>>,
}
```

### Phase 2: Migrate Capabilities (9 files)

Each capability function that currently calls filesystem must be changed to:

**Before:**
```rust
pub fn analyze_orphan(&self, path: &Path) -> Violation {
    let content = filesystem::utility_io::read_file_safe(path);
    // ... business logic ...
}
```

**After:**
```rust
pub fn analyze_orphan(&self, path: &Path, content: &str) -> Violation {
    // ... business logic only (no I/O) ...
}
```

### Phase 3: Update Orchestrators (5 files)

Each orchestrator must:
1. Accept `&dyn IFilesystemAggregate` in constructor or run method
2. Call aggregate methods to get data
3. Pass data to capabilities

**Before:**
```rust
pub fn run(&self, target: &TargetPath) -> LintResultList {
    let content = filesystem::utility_io::read_file_safe(path);
    let violations = self.analyzer.analyze(path);
    // ...
}
```

**After:**
```rust
pub fn run(&self, target: &TargetPath, fs: &dyn IFilesystemAggregate) -> LintResultList {
    let content = fs.read_file(path);
    let violations = self.analyzer.analyze(path, &content);
    // ...
}
```

### Phase 4: Update Contract Agents (5 files)

Each contract agent must inject filesystem aggregate:

```rust
pub struct OrphanAgent {
    fs: Arc<dyn IFilesystemAggregate>,
    orchestrator: OrphanOrchestrator,
}
```

## 5. Detailed Migration Plan

### 5.1 orphan-detector (21 calls → 0 in capabilities)

**Files to change:**

| File | Change | Complexity |
|------|--------|-----------|
| `agent_orphan_orchestrator.rs` | Inject `&dyn IFilesystemAggregate`, use `fs.read_file()`, `fs.discover_files()`, etc. | Medium |
| `capabilities_orphan_agent_analyzer.rs` | Remove `filesystem::utility_io::read_file_safe`, accept `content: &str` parameter | Low |
| `capabilities_orphan_utility_analyzer.rs` | Remove `filesystem::utility_io::read_file_safe`, accept `content: &str` parameter | Low |
| `capabilities_orphan_contract_analyzer.rs` | Remove `filesystem::utility_io::read_file_safe`, accept `content: &str` parameter | Low |
| `capabilities_orphan_graph_resolver.rs` | Remove `filesystem::utility_io::scan_directory`, `is_dir`, accept data from orchestrator | Medium |

**New aggregate methods needed:**
- `read_file_safe(path) → String` (already exists as `read_file`)
- `discover_files(root, ignored) → Vec<FileEntry>` (already exists)
- `is_dir(path) → bool` (already exists)

### 5.2 import-rules (6 calls → 0 in capabilities)

**Files to change:**

| File | Change | Complexity |
|------|--------|-----------|
| `agent_import_orchestrator.rs` | Inject `&dyn IFilesystemAggregate`, use `fs.read_file()`, `fs.path_exists()`, etc. | Low |
| `capabilities_import_mandatory_checker.rs` | Remove `filesystem::utility_io::read_file`, accept `content: &str` parameter | Low |
| `capabilities_import_unused_checker.rs` | Remove `filesystem::utility_io::read_file`, accept `content: &str` parameter | Low |
| `capabilities_cycle_import_analyzer.rs` | Remove `filesystem::utility_io::read_file`, accept `content: &str` parameter | Low |
| `capabilities_import_forbidden_checker.rs` | Remove `filesystem::utility_io::read_file`, accept `content: &str` parameter | Low |

**New aggregate methods needed:**
- `read_file(path) → Option<String>` (already exists)
- `path_exists(path) → bool` (already exists)
- `discover_source_files(root, ignored) → Vec<FilePath>` (already exists)

### 5.3 code-analysis (2 calls → 0 in capabilities)

**Files to change:**

| File | Change | Complexity |
|------|--------|-----------|
| `agent_code_analysis_orchestrator.rs` | Inject `&dyn IFilesystemAggregate`, use `fs.read_lintable_file()` | Low |

**No capabilities changes needed** (capabilities don't do I/O here).

### 5.4 role-rules (3 calls → 0 in capabilities)

**Files to change:**

| File | Change | Complexity |
|------|--------|-----------|
| `agent_role_orchestrator.rs` | Inject `&dyn IFilesystemAggregate`, use `fs.read_file()`, `fs.discover_source_files()` | Low |
| `capabilities_surface_role_auditor.rs` | Remove `filesystem::utility_io::read_file`, accept `content: &str` parameter | Low |

### 5.5 naming-rules (1 call → 0)

**Files to change:**

| File | Change | Complexity |
|------|--------|-----------|
| `agent_naming_orchestrator.rs` | Replace `shared::naming_rules::utility_naming_filesystem::walk_recursive` with `fs.discover_source_files()` | Low |

## 6. Implementation Order

```
Step 1: Create FilesystemContext DTO (shared crate)
Step 2: Migrate orphan-detector (largest, 21 calls)
  Step 2a: Update capabilities to accept data parameters
  Step 2b: Update orchestrator to use aggregate
Step 3: Migrate import-rules (6 calls)
  Step 3a: Update capabilities to accept data parameters
  Step 3b: Update orchestrator to use aggregate
Step 4: Migrate code-analysis (2 calls)
Step 5: Migrate role-rules (3 calls)
Step 6: Migrate naming-rules (1 call)
Step 7: Remove direct filesystem::utility_io imports from all capabilities
Step 8: Update all tests
Step 9: Verify 0 violations + full test pass
```

## 7. Aggregate Contract Changes

### 7.1 Existing Methods (no change needed)

```rust
fn scan(&self, root: &PathBuf, ignored: &[String]) -> FilesystemResult;
fn read_file(&self, path: &Path) -> Option<String>;
fn read_lintable_file(&self, path: &str) -> Result<Option<String>, String>;
fn get_file_content(&self, path: &PathBuf) -> Option<String>;
fn has_file(&self, path: &PathBuf) -> bool;
fn discover_files(&self, root: &Path, ignored: &[String]) -> Vec<FileEntry>;
fn discover_source_files(&self, root: &Path, ignored: &[String]) -> Vec<FilePath>;
fn all_files(&self) -> &[FileEntry];
fn path_exists(&self, path: &Path) -> bool;
fn is_dir(&self, path: &Path) -> bool;
fn should_ignore(&self, path: &str, ignored: &[String]) -> bool;
fn workspace_root(&self, start: &str) -> Option<PathBuf>;
```

### 7.2 New Methods Needed

```rust
/// Read file content, returning empty string on error (safe version).
fn read_file_safe(&self, path: &Path) -> String;

/// List directory entries as (name, path, is_dir) tuples.
fn list_directory(&self, path: &Path) -> Vec<(String, String, bool)>;
```

## 8. Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|-----------|
| Breaking existing tests | High | Run tests after each step |
| Performance regression (extra DashMap lookups) | Low | Aggregate uses cache-first approach |
| Circular dependency (shared ↔ filesystem) | High | Keep DTOs in shared, aggregate in shared |
| Large diff size | Medium | Migrate one crate at a time |

## 9. Success Criteria

- [ ] All capabilities have ZERO `filesystem::` imports
- [ ] All orchestrators use `&dyn IFilesystemAggregate`
- [ ] All tests pass
- [ ] 0 lint violations
- [ ] No circular dependencies

## 10. Estimated Effort

| Phase | Files | Complexity | Est. Time |
|-------|-------|-----------|-----------|
| DTOs | 2 | Low | 30 min |
| orphan-detector | 6 | Medium | 2 hours |
| import-rules | 5 | Low | 1 hour |
| code-analysis | 1 | Low | 15 min |
| role-rules | 2 | Low | 30 min |
| naming-rules | 1 | Low | 15 min |
| Tests | 5+ | Medium | 1 hour |
| **Total** | **22** | | **~5 hours** |
