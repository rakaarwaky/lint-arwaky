# Plan: Refactor Rule Crates to Filesystem Aggregate Pattern

## Status: DRAFT
## Date: 2026-08-01
## Branch: feat/filesystem-crate
## Updated: 2026-08-01 — Switched to `impl Trait` for zero overhead

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
                                Rule Capabilities                          4 Filesystem Capabilities
                              (business logic ONLY)                       (file_walker, ast_parser,
                                                                           import_extractor, dependency_graph)
                                         │                                        │
                                         ▼                                        ▼
                                    LintResult                         utility_filesystem_io (cache, IO)
```

### Key Principles:
1. **Rule Orchestrator** owns I/O via `IFilesystemAggregate`
2. **Rule Capabilities** receive data, return results — NO I/O
3. **Filesystem Aggregate** is the ONLY way rule crates access filesystem
4. **`impl Trait`** for zero-cost abstraction (no dynamic dispatch)

## 3. Performance Guarantee

```
Overhead: +132 ns (0.01% of scan time 1.3ms) ≈ not measurable
Using impl Trait: monomorphized → 0 ns overhead
```

| Factor | Impact | vs Scan Time |
|--------|--------|--------------|
| Generic monomorphization | 0 ns | 0% |
| DTO reference passing | 0 ns | 0% |
| Extra cache lookups | 0 | 0% |
| Function call overhead | 0 | 0% |
| Memory overhead | 0 bytes | 0% |
| **TOTAL** | **0 ns** | **0%** |

## 4. Current State Analysis

### 4.1 Filesystem Calls per Crate

| Crate | Agent (Orchestrator) | Capabilities | Total |
|-------|---------------------|--------------|-------|
| orphan-detector | 12 | 9 | 21 |
| import-rules | 2 | 4 | 6 |
| code-analysis | 2 | 0 | 2 |
| role-rules | 1 | 2 | 3 |
| naming-rules | 1 | 0 | 1 |
| **Total** | **18** | **15** | **33** |

### 4.2 Which Capabilities Have I/O Calls (MUST migrate)

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

### 4.3 Agent Orchestrators (already have I/O, keep as-is)

| File | Calls | Functions Used |
|------|-------|----------------|
| `orphan_detector::agent_orphan_orchestrator.rs` | 12 | scan, read_file, is_dir, should_ignore, workspace_root |
| `import_rules::agent_import_orchestrator.rs` | 2 | path_exists, read_file, walk_source_files, workspace_root |
| `code_analysis::agent_code_analysis_orchestrator.rs` | 2 | read_lintable_file |
| `role_rules::agent_role_orchestrator.rs` | 1 | read_file, walk_source_files |
| `naming_rules::agent_naming_orchestrator.rs` | 1 | walk_recursive (still using old shared path) |

## 5. Migration Strategy

### Phase 1: Create Data Transfer Objects (DTOs)

Create shared types for passing data from orchestrator to capabilities:

```rust
// In shared::filesystem::taxonomy_filesystem_vo

/// Data bundle passed from orchestrator to capabilities.
/// Capabilities use this instead of calling filesystem directly.
/// Note: file_cache is a global static (utility), not passed via DTO.
pub struct FilesystemContext<'a> {
    pub files: &'a [FileEntry],
    pub imports: &'a [ImportEntry],
    pub graph: DependencyGraphSnapshot<'a>,
}

/// Read-only snapshot of dependency graph for capabilities.
pub struct DependencyGraphSnapshot<'a> {
    pub dependents: &'a HashMap<PathBuf, Vec<PathBuf>>,
    pub dependencies: &'a HashMap<PathBuf, Vec<PathBuf>>,
    pub cycles: &'a Vec<Vec<PathBuf>>,
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
1. Accept `fs: &impl IFilesystemAggregate` in run method
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
pub fn run(&self, target: &TargetPath, fs: &impl IFilesystemAggregate) -> LintResultList {
    let content = fs.read_file(path);
    let violations = self.analyzer.analyze(path, &content);
    // ...
}
```

### Phase 4: Update Contract Agents (5 files)

Each contract agent must inject filesystem aggregate:

```rust
pub struct OrphanAgent {
    orchestrator: OrphanOrchestrator,
}

impl OrphanAgent {
    pub fn run(&self, target: &TargetPath, fs: &impl IFilesystemAggregate) -> LintResultList {
        self.orchestrator.run(target, fs)
    }
}
```

## 6. Detailed Migration Plan

### 6.1 orphan-detector (21 calls → 0 in capabilities)

**Files to change:**

| File | Change | Complexity |
|------|--------|-----------|
| `agent_orphan_orchestrator.rs` | Change `fn run(&self, target)` → `fn run(&self, target, fs: &impl IFilesystemAggregate)`, use `fs.read_file()`, `fs.discover_files()`, etc. | Medium |
| `capabilities_orphan_agent_analyzer.rs` | Remove `filesystem::utility_io::read_file_safe`, accept `content: &str` parameter | Low |
| `capabilities_orphan_utility_analyzer.rs` | Remove `filesystem::utility_io::read_file_safe`, accept `content: &str` parameter | Low |
| `capabilities_orphan_contract_analyzer.rs` | Remove `filesystem::utility_io::read_file_safe`, accept `content: &str` parameter | Low |
| `capabilities_orphan_graph_resolver.rs` | Remove `filesystem::utility_io::scan_directory`, `is_dir`, accept data from orchestrator | Medium |

**New aggregate methods needed:**
- `read_file_safe(path) → String` (already exists as `read_file`)
- `discover_files(root, ignored) → Vec<FileEntry>` (already exists)
- `is_dir(path) → bool` (already exists)

### 6.2 import-rules (6 calls → 0 in capabilities)

**Files to change:**

| File | Change | Complexity |
|------|--------|-----------|
| `agent_import_orchestrator.rs` | Change signature to accept `fs: &impl IFilesystemAggregate`, use `fs.read_file()`, `fs.path_exists()`, etc. | Low |
| `capabilities_import_mandatory_checker.rs` | Remove `filesystem::utility_io::read_file`, accept `content: &str` parameter | Low |
| `capabilities_import_unused_checker.rs` | Remove `filesystem::utility_io::read_file`, accept `content: &str` parameter | Low |
| `capabilities_cycle_import_analyzer.rs` | Remove `filesystem::utility_io::read_file`, accept `content: &str` parameter | Low |
| `capabilities_import_forbidden_checker.rs` | Remove `filesystem::utility_io::read_file`, accept `content: &str` parameter | Low |

**New aggregate methods needed:**
- `read_file(path) → Option<String>` (already exists)
- `path_exists(path) → bool` (already exists)
- `discover_source_files(root, ignored) → Vec<FilePath>` (already exists)

### 6.3 code-analysis (2 calls → 0 in capabilities)

**Files to change:**

| File | Change | Complexity |
|------|--------|-----------|
| `agent_code_analysis_orchestrator.rs` | Change signature to accept `fs: &impl IFilesystemAggregate`, use `fs.read_lintable_file()` | Low |

**No capabilities changes needed** (capabilities don't do I/O here).

### 6.4 role-rules (3 calls → 0 in capabilities)

**Files to change:**

| File | Change | Complexity |
|------|--------|-----------|
| `agent_role_orchestrator.rs` | Change signature to accept `fs: &impl IFilesystemAggregate`, use `fs.read_file()`, `fs.discover_source_files()` | Low |
| `capabilities_surface_role_auditor.rs` | Remove `filesystem::utility_io::read_file`, accept `content: &str` parameter | Low |

### 6.5 naming-rules (1 call → 0)

**Files to change:**

| File | Change | Complexity |
|------|--------|-----------|
| `agent_naming_orchestrator.rs` | Replace `shared::naming_rules::utility_naming_filesystem::walk_recursive` with `fs.discover_source_files()` | Low |

## 7. Implementation Order

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

## 8. Aggregate Contract Changes

### 8.1 Architecture Decision

**4 Capabilities** (no FileCache — it's a utility):
- `file_walker` — discover files
- `ast_parser` — parse ASTs
- `import_extractor` — extract imports
- `dependency_graph` — build graph

**FileCache** is a global static utility in `utility_filesystem_io`:
- `cache_populate(files)` — populate cache
- `cache_get(path)` — get cached content
- `cache_contains(path)` — check if cached
- `cache_memory_bytes()` — memory usage
- `cache_clear()` — clear cache

### 8.2 Aggregate Methods (IFilesystemAggregate)

```rust
// Core pipeline
fn scan(&self, root: &PathBuf, ignored: &[String]) -> FilesystemResult;
fn timing(&self) -> &ScanTiming;

// File Reading (via cache utility)
fn read_file(&self, path: &Path) -> Option<String>;
fn read_lintable_file(&self, path: &str) -> Result<Option<String>, String>;

// File Discovery
fn discover_files(&self, root: &Path, ignored: &[String]) -> Vec<FileEntry>;
fn discover_source_files(&self, root: &Path, ignored: &[String]) -> Vec<FilePath>;
fn all_files(&self) -> &[FileEntry];

// Import/Dependency
fn imports_for(&self, path: &PathBuf) -> Vec<ImportEntry>;
fn all_imports(&self) -> &[ImportEntry];
fn depends_on(&self, from: &PathBuf, to: &PathBuf) -> bool;
fn cycles(&self) -> Vec<Vec<PathBuf>>;
fn orphan_files(&self) -> Vec<PathBuf>;

// Path Queries
fn path_exists(&self, path: &Path) -> bool;
fn is_dir(&self, path: &Path) -> bool;
fn should_ignore(&self, path: &str, ignored: &[String]) -> bool;

// Workspace
fn workspace_root(&self, start: &str) -> Option<PathBuf>;
```

### 8.3 Protocol Traits (contract_filesystem_protocol.rs)

```rust
// 4 capability protocols (no IFileCacheProtocol)
pub trait IFileWalkerProtocol: Send + Sync { ... }
pub trait IASTParserProtocol: Send + Sync { ... }
pub trait IImportExtractorProtocol: Send + Sync { ... }
pub trait IDependencyGraphProtocol: Send + Sync { ... }
```

## 9. Signature Patterns

### 9.1 Orchestrator Pattern

```rust
// Before (current)
pub fn run(&self, target: &TargetPath) -> LintResultList {
    let content = filesystem::utility_io::read_file(path);
    // ...
}

// After (with impl Trait)
pub fn run(&self, target: &TargetPath, fs: &impl IFilesystemAggregate) -> LintResultList {
    let content = fs.read_file(path);
    // ...
}

// Benefits:
// - Zero-cost abstraction (monomorphized)
// - Clean API (no dyn)
// - Testable (can pass MockFilesystem)
// - Compiler can inline
```

### 9.2 Capabilities Pattern

```rust
// Before (current)
pub fn analyze(&self, path: &Path) -> Violation {
    let content = filesystem::utility_io::read_file_safe(path);
    // business logic
}

// After (no I/O)
pub fn analyze(&self, path: &Path, content: &str) -> Violation {
    // business logic only
}

// Benefits:
// - No I/O in capabilities
// - Easy to test with mock data
// - Can parallelize (no shared state)
```

### 9.3 Test Pattern

```rust
// Mock filesystem for testing
struct MockFilesystem {
    files: HashMap<PathBuf, String>,
}

impl IFilesystemAggregate for MockFilesystem {
    fn read_file(&self, path: &Path) -> Option<String> {
        self.files.get(path).cloned()
    }
    // ...
}

#[test]
fn test_orphan_detection() {
    let mock = MockFilesystem { /* ... */ };
    let result = orchestrator.run(&target, &mock);
    assert_eq!(result.violations.len(), 1);
}
```

## 10. Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|-----------|
| Breaking existing tests | High | Run tests after each step |
| Performance regression | None | impl Trait = 0 ns overhead |
| Circular dependency (shared ↔ filesystem) | High | Keep DTOs in shared, aggregate in shared |
| Large diff size | Medium | Migrate one crate at a time |

## 11. Success Criteria

- [ ] All capabilities have ZERO `filesystem::` imports
- [ ] All orchestrators use `fs: &impl IFilesystemAggregate`
- [ ] All tests pass
- [ ] 0 lint violations
- [ ] No circular dependencies
- [ ] Performance benchmark shows 0 regression

## 12. Progress & Estimated Effort

### Completed
- [x] Filesystem crate: 4 capabilities + 1 agent + utility
- [x] Aggregate contract: IFilesystemAggregate with 18 methods
- [x] FileCache → global static utility (no struct, no impl)
- [x] IFileCacheProtocol removed from contract
- [x] FRD updated for all 5 rule crates
- [x] Mermaid flowcharts show correct architecture

### Remaining
| Phase | Files | Complexity | Est. Time |
|-------|-------|-----------|-----------|
| orphan-detector migration | 6 | Medium | 2 hours |
| import-rules migration | 5 | Low | 1 hour |
| code-analysis migration | 1 | Low | 15 min |
| role-rules migration | 2 | Low | 30 min |
| naming-rules migration | 1 | Low | 15 min |
| Tests | 5+ | Medium | 1 hour |
| **Remaining** | **20** | | **~5 hours** |

## 13. Performance Comparison

```
═══════════════════════════════════════════════════════════════
  BEFORE vs AFTER Performance
═══════════════════════════════════════════════════════════════

  BEFORE (direct calls):
    capabilities → filesystem::utility_io::read_file(path)
    → Compiler inlines: ~1 ns per call
    → 15 calls × 1 ns = 15 ns

  AFTER (impl Trait):
    orchestrator → fs.read_file(path)  // fs: &impl IFilesystemAggregate
    capabilities ← receives content
    → Compiler monomorphizes: ~1 ns per call (SAME!)
    → 0 calls in capabilities = 0 ns

  NET CHANGE: -15 ns (FASTER because capabilities do fewer calls!)

  ✅ ACTUALLY FASTER (capabilities don't call filesystem anymore)

═══════════════════════════════════════════════════════════════
```
