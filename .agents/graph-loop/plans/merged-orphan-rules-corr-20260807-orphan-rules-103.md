# Plan: orphan-rules — Architect (Merged Plan)

## Summary

The orphan-rules crate (AES501–AES506) is architecturally sound in structure — clean 3-block patterns, correct naming, proper BFS reachability, and good FRD traceability. However, two **CRITICAL** findings dominate: (1) `eprintln!` debug statements in production code (`utility_orphan_graph.rs`) violate the project-wide tracing migration, and (2) two capability analyzers (`ContractOrphanAnalyzer`, `CapabilitiesOrphanAnalyzer`) hold `Arc<dyn IFilesystemAggregate>` references and perform direct I/O (file discovery, directory checks, cached reads), violating the FRD's "zero I/O" invariant and the AES Capabilities role boundary. These must be fixed before merge.

---

## Merged Findings

### Layer Boundaries

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| LB-01 | 🔴 CRITICAL | `ContractOrphanAnalyzer` holds `filesystem: Arc<dyn IFilesystemAggregate>` and performs I/O in `cached_search_files()` — calls `find_workspace_root_from_path()`, `is_dir()`, `discover_source_files()` | `capabilities_orphan_contract_analyzer.rs` lines 6, 148–171 | Remove `filesystem` dependency. The orchestrator already pre-reads all files into `content_map` — pass needed data as parameters instead of performing I/O |
| LB-02 | 🔴 CRITICAL | `CapabilitiesOrphanAnalyzer` holds `filesystem: Arc<dyn IFilesystemAggregate>` and performs I/O — calls `read_cached()`, `find_workspace_root_from_path()`, `check_wired_in_container()` | `capabilities_orphan_capabilities_analyzer.rs` lines 6, 68–74 | Same as LB-01: the orchestrator should pre-compute wiring info or the container check should be delegated upward |

### Naming

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| — | — | All 11 files pass AES101–102 | All files | No issues found |

### Orphan

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| O-01 | 🔴 CRITICAL | `eprintln!` debug statements in production BFS code — 4 occurrences printing debug graph info to stderr. Violates project-wide tracing migration (all eprintln replaced with tracing). Will spam terminal output | `utility_orphan_graph.rs` lines 25–33, 45–50, 53–56 | Replace all `eprintln!` with `tracing::debug!` or `tracing::trace!` with structured fields |
| O-02 | 🟡 WARNING | Contract analyzer `cached_search_files()` calls `discover_source_files()` — this duplicates filesystem crate's file discovery, causing potential inconsistency if the filesystem crate changes discovery logic | `capabilities_orphan_contract_analyzer.rs` lines 148–171 | The orchestrator already discovers files via `GraphAnalysisContext.all_workspace_files`. Use that data instead of re-discovering |

### Scalability

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| S-01 | 🟡 WARNING | `TaxonomyOrphanAnalyzer::is_taxonomy_orphan()` content-based fallback scans ALL files in `all_files` linearly for each taxonomy orphan check. For a workspace with 10K files × 500 taxonomy files, this is O(N×M) | `capabilities_orphan_taxonomy_analyzer.rs` lines 82–110 | Pre-build a reverse index (module_name → higher-layer consumers) in the orchestrator and pass it to the analyzer |
| S-02 | 🟡 WARNING | `UtilityOrphanAnalyzer::is_utility_orphan()` also has a linear fallback scan across all consumer files (lines 72–85). Same O(N×M) concern | `capabilities_orphan_utility_analyzer.rs` lines 72–85 | Same fix as S-01: pre-compute consumer lookup in orchestrator |

### Data Flow

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| D-01 | 🟡 WARNING | `ContractOrphanAnalyzer` uses `Mutex<Option<SearchFilesCache>>` for caching — but the orchestrator already runs analyzers via `rayon::par_iter`. A `Mutex` cache inside a parallel iterator is a contention point and the cache key (root + file_count) may not be unique across concurrent tasks | `capabilities_orphan_contract_analyzer.rs` lines 14, 25–35 | Since the orchestrator pre-reads everything into `content_map`, remove the cache and the filesystem calls entirely |

---

## Validation

- [x] FRD compliance checked — FR-001 through FR-010 mapped to code; gaps identified in LB-01/LB-02 (orchestrator does I/O instead of zero-I/O contract)
- [x] AES compliance checked — AES101/102 pass; AES403 (capabilities role) violated by filesystem dependency; AES304 (bypass) not violated
- [x] Skip Report validated — N/A (no nodes skipped)
- [x] Assumptions validated — FRD assumes zero I/O, but 2 analyzers do I/O; FRD assumes rayon parallelism but Mutex cache is a contention point
- [x] Timestamp + Correlation ID signed — corr-20260807-orphan-rules-103, 2026-08-07T05:30:00+07:00

---

## Action Items

- [ ] 🔴 **CRITICAL** Replace all `eprintln!` in `utility_orphan_graph.rs` with `tracing::debug!` (O-01)
- [ ] 🔴 **CRITICAL** Remove `filesystem` dependency from `ContractOrphanAnalyzer` — eliminate `cached_search_files()` I/O calls (LB-01, O-02)
- [ ] 🔴 **CRITICAL** Remove `filesystem` dependency from `CapabilitiesOrphanAnalyzer` — eliminate `read_cached()` and `check_wired_in_container()` I/O (LB-02)
- [ ] 🟡 Update `OrphanContainer` wiring to pass pre-computed data instead of filesystem references (LB-01, LB-02)
- [ ] 🟡 Update `ArchOrphanDeps` to remove filesystem from contract and capabilities analyzer constructors
- [ ] 🟡 Update `Cargo.toml` if filesystem dependency can be removed from direct deps after analyzer cleanup (verify shared workspace dep still needed)

---

## Fixed Code

### File: `utility_orphan_graph.rs`

Replace all `eprintln!` with `tracing::debug!`:

```rust
// BEFORE (lines 25-33):
eprintln!(
    "[debug bfs] eps={} graph_keys={} edges_from_first_3={}",
    entry_points.len(),
    graph.mapping.len(),
    edges_found
);

// AFTER:
debug!(
    entry_points = entry_points.len(),
    graph_keys = graph.mapping.len(),
    edges_from_first_3 = edges_found,
    "BFS reachability start"
);
```

```rust
// BEFORE (lines 45-50):
eprintln!(
    "[debug bfs] reachable={} not_reachable_sample={:?}",
    reachable.len(),
    graph
        .mapping
        .keys()
        .filter(|k| !reachable.contains(*k))
        .take(5)
        .collect::<Vec<_>>()
);

// AFTER:
debug!(
    reachable = reachable.len(),
    not_reachable_sample = ?graph
        .mapping
        .keys()
        .filter(|k| !reachable.contains(*k))
        .take(5)
        .collect::<Vec<_>>(),
    "BFS reachability complete"
);
```

```rust
// BEFORE (lines 53-56):
let sample_target = "crates/shared/src/common/taxonomy_definition_vo.rs";
eprintln!(
    "[debug inbound] {} has_key={}",
    sample_target,
    graph.mapping.contains_key(sample_target)
);

// AFTER — remove entirely, this is a hardcoded debug probe not suitable for production
```

Add `use tracing::debug;` to imports.

### File: `capabilities_orphan_contract_analyzer.rs`

Remove `filesystem` field and `cached_search_files()`. The orchestrator should provide the search file list via the `all_files` parameter:

```rust
// BEFORE:
pub struct ContractOrphanAnalyzer {
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    search_cache: Mutex<Option<SearchFilesCache>>,
}

impl ContractOrphanAnalyzer {
    pub fn new(filesystem: Arc<dyn IFilesystemAggregate>) -> Self {
        Self {
            search_cache: Mutex::new(None),
            filesystem,
        }
    }
    // ... cached_search_files() method that calls filesystem I/O
}

// AFTER:
pub struct ContractOrphanAnalyzer;

impl ContractOrphanAnalyzer {
    pub fn new() -> Self {
        Self
    }
}

// Remove SearchFilesCache struct entirely
// Remove cached_search_files() method entirely
// In is_contract_orphan(): replace `self.cached_search_files(root_dir, all_files)` with `Arc::new(all_files.to_vec())`
```

### File: `capabilities_orphan_capabilities_analyzer.rs`

Remove `filesystem` field. The orchestrator already provides `content_map`:

```rust
// BEFORE:
pub struct CapabilitiesOrphanAnalyzer {
    pub filesystem: Arc<dyn IFilesystemAggregate>,
}

impl CapabilitiesOrphanAnalyzer {
    pub fn new(filesystem: Arc<dyn IFilesystemAggregate>) -> Self {
        Self { filesystem }
    }
}

// AFTER:
pub struct CapabilitiesOrphanAnalyzer;

impl CapabilitiesOrphanAnalyzer {
    pub fn new() -> Self {
        Self
    }
}
```

In `is_capabilities_orphan()`, replace `self.filesystem.read_cached(&path)` with a `content_map` parameter (add `content_map: &HashMap<String, String>` to the trait signature). Replace `self.filesystem.find_workspace_root_from_path()` and `self.filesystem.check_wired_in_container()` with data passed from the orchestrator.

### File: `root_orphan_detector_container.rs`

Update wiring to match new analyzer constructors:

```rust
// BEFORE:
capabilities_analyzer: Arc::new(
    crate::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer::new(
        filesystem.clone(),
    ),
),

// AFTER:
capabilities_analyzer: Arc::new(
    crate::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer::new(),
),
```

Same for contract_analyzer.

### File: `agent_orphan_orchestrator.rs`

Update `_check_orphans_inner` to pre-compute the container wiring check result and pass it to the capabilities analyzer (requires updating the `ICapabilitiesOrphanProtocol` trait in shared).
