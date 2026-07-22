# Orphan Detector Fix Plan

> Generated from BA, QA, and Backend reports. All findings validated against source code.
> User decisions applied: AES503=architecture correct, Agent-Utility=allowed (config YAML + ARCHITECTURE.md updated), Performance=hybrid, ILayerDetectionAggregate=remove, Path security=include, Severity=adjust.

---

## Summary

| Metric                          | Value |
| ------------------------------- | ----- |
| Total issues                    | 28    |
| Critical (security/correctness) | 8     |
| High (performance/architecture) | 10    |
| Medium (code quality)           | 6     |
| Low (documentation/severity)    | 4     |
| New files to create             | 4     |
| Files to modify                 | 10    |

---

## Phase 1: Security & Path Confinement (Critical)

### P1.1 — Add path confinement utility

**File:** `crates/shared/src/orphan-detector/utility_orphan_path.rs` (NEW)

**Skill:** `create-utility-rust` — stateless standalone functions only, no struct, no impl blocks, depends on Taxonomy only.

**Problem:** `#[path = "/etc/passwd"]` in source files can escape workspace root. No path confinement exists.

**Fix:** Create utility with `normalize_lexical`, `confine_under_root`, `resolve_module_path`.

```rust
use std::path::{Component, Path, PathBuf};

pub fn normalize_lexical(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => { normalized.pop(); }
            other => normalized.push(other.as_os_str()),
        }
    }
    normalized
}

pub fn confine_under_root(root: &Path, candidate: &Path) -> Option<PathBuf> {
    let root = normalize_lexical(root);
    let absolute = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        root.join(candidate)
    };
    let normalized = normalize_lexical(&absolute);
    if normalized.starts_with(&root) { Some(normalized) } else { None }
}

pub fn resolve_module_path(root: &Path, base_dir: &Path, module_path: &str) -> Option<PathBuf> {
    let candidate = if Path::new(module_path).is_absolute() {
        PathBuf::from(module_path)
    } else {
        base_dir.join(module_path)
    };
    confine_under_root(root, &candidate)
}
```

**Module registration:** Add `pub mod utility_orphan_path;` to `crates/shared/src/orphan-detector/mod.rs`.

### P1.2 — Use path confinement in graph resolver

**Skill:** `create-capabilities-rust` — adding utility call to capabilities, allowed (capabilities may depend on Utility).

**File:** `crates/orphan-detector/src/capabilities_orphan_graph_resolver.rs`

**Problem:** Lines 217-221 resolve `#[path = "..."]` without confinement check.

**Before:**

```rust
let resolved = if mod_path.starts_with('/') {
    mod_path.clone()
} else {
    format!("{}/{}", base_dir, mod_path)
};
```

**After:**

```rust
use shared::orphan_detector::utility_orphan_path::resolve_module_path;

let Some(resolved_path) = resolve_module_path(
    std::path::Path::new(root_dir),
    &std::path::Path::new(&base_dir),
    &mod_path,
) else {
    continue; // Reject paths outside workspace
};
let resolved = resolved_path.to_string_lossy().to_string();
```

### P1.3 — Fix SurfacesOrphanAnalyzer CWD dependency

**Skill:** `create-contract-rust` — protocol signature change must use VOs, remain object-safe.

**File:** `crates/orphan-detector/src/capabilities_orphan_surfaces_analyzer.rs`

**Problem:** Line 42 uses `Path::new(".")` instead of `root_dir`.

**Before:**

```rust
let root = std::path::Path::new(".");
if let Ok(workspace_root) =
    shared::orphan_detector::utility_workspace::find_workspace_root(root)
```

**After:** Pass `root_dir` through the protocol. Change `ISurfacesOrphanProtocol`:

```rust
// In contract_orphan_protocol.rs
pub trait ISurfacesOrphanProtocol: Send + Sync {
    fn is_surface_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,  // ADD THIS
        alive_files: &ReachabilityResult,
        definition: Option<&LayerDefinition>,
    ) -> OrphanIndicatorResult;
}
```

Then in `surfaces_analyzer.rs`:

```rust
let root = std::path::Path::new(root_dir.value());
if let Ok(workspace_root) =
    shared::orphan_detector::utility_workspace::find_workspace_root(root)
```

Update call site in `orchestrator.rs:301-304`:

```rust
if layer_str.contains(LAYER_SURFACES) {
    return self
        .surfaces_analyzer
        .is_surface_orphan(&fp, &root, &alive_set, None);
}
```

---

## Phase 2: Configuration & Ignored Paths (High)

### P2.1 — Wire ignored_paths into orchestrator

**Skill:** `create-root-rust` — container wires Capabilities to Contracts, may instantiate and wire components.

**File:** `crates/orphan-detector/src/root_orphan_detector_container.rs`

**Problem:** Line 19: `new_with_ignored(_ignored_paths)` ignores the parameter.

**Before:**

```rust
pub fn new_with_ignored(_ignored_paths: Vec<String>) -> Self {
```

**After:**

```rust
pub fn new_with_ignored(ignored_paths: Vec<String>) -> Self {
    let mut config = shared::config_system::taxonomy_config_vo::ArchitectureConfig::default();
    config.ignored_paths = shared::common::taxonomy_paths_vo::FilePathList::new(
        ignored_paths
            .into_iter()
            .filter_map(|p| shared::common::taxonomy_path_vo::FilePath::new(p).ok())
            .collect(),
    );
    // ... pass config to ArchOrphanAnalyzer
}
```

### P2.2 — Store real config in ArchOrphanAnalyzer

**Skill:** `create-agent-rust` — config field uses shared VO (`ArchitectureConfig`), constructor in Block 3.

**File:** `crates/orphan-detector/src/agent_orphan_orchestrator.rs`

**Problem:** Lines 312-318: `config()` returns static default via `OnceLock`.

**Before:**

```rust
pub struct ArchOrphanAnalyzer {
    resolver: Arc<dyn IOrphanGraphResolverProtocol>,
    // ... 6 analyzers
}
```

**After:**

```rust
pub struct ArchOrphanAnalyzer {
    resolver: Arc<dyn IOrphanGraphResolverProtocol>,
    // ... 6 analyzers
    config: shared::config_system::taxonomy_config_vo::ArchitectureConfig,
}

impl ArchOrphanAnalyzer {
    pub fn new_with_config(
        resolver: Arc<dyn IOrphanGraphResolverProtocol>,
        // ... 6 analyzers
        config: shared::config_system::taxonomy_config_vo::ArchitectureConfig,
    ) -> Self {
        Self { resolver, /* ... */, config }
    }
}
```

Update `config()` method:

```rust
fn config(&self) -> &shared::config_system::taxonomy_config_vo::ArchitectureConfig {
    &self.config
}
```

### P2.3 — Add is_ignored helper and filter files early

**Skill:** `create-utility-rust` — path filtering is stateless, domain-agnostic, reusable. Must NOT live in Agent layer (agent skill: "zero business logic").

**Step A — Add utility function to `crates/shared/src/orphan-detector/utility_orphan_path.rs`:**

```rust
pub fn is_path_ignored(file: &str, patterns: &[String]) -> bool {
    let file = file.replace('\\', "/");
    patterns.iter().any(|pattern| {
        let raw = pattern.replace('\\', "/");
        if raw.is_empty() { return false; }
        if file == raw || file.ends_with(&raw) { return true; }
        let normalized = raw.trim_start_matches('/');
        if normalized.is_empty() { return false; }
        file.starts_with(&format!("{normalized}/"))
            || file.contains(&format!("/{normalized}/"))
            || file.contains(&format!("/{normalized}"))
    })
}
```

**Step B — In `agent_orphan_orchestrator.rs`, call utility in `check_orphans`:**

```rust
let ignored: Vec<String> = self.config.ignored_paths.values.iter()
    .map(|p| p.value().to_string())
    .collect();
let filtered_files: Vec<String> = files
    .iter()
    .filter(|f| !shared::orphan_detector::utility_orphan_path::is_path_ignored(f, &ignored))
    .cloned()
    .collect();
let files = filtered_files.as_slice();
```

---

## Phase 3: Remove ILayerDetectionAggregate from Orphan Detector (High)

### P3.1 — Remove ILayerDetectionAggregate impl from ArchOrphanAnalyzer

**File:** `crates/orphan-detector/src/agent_orphan_orchestrator.rs`

**Skill:** `create-agent-rust` — agent must not implement detection logic; layer detection belongs in Capabilities.

**Problem:** `ArchOrphanAnalyzer` implements `ILayerDetectionAggregate` with static defaults. Layer detection is just prefix matching — should use `utility_layer_detector` via a proper contract.

**Remove:** The entire `impl ILayerDetectionAggregate for ArchOrphanAnalyzer` block (lines 311-375).

**Replace with:** Inject `Arc<dyn ILayerDetectionProtocol>` via DI (see P3.4).

### P3.2 — Update OrphanContainer to use ILayerDetectionProtocol (not ILayerDetectionAggregate)

**File:** `crates/orphan-detector/src/root_orphan_detector_container.rs`

**Skill:** `create-root-rust` — container wires Capabilities to Contracts.

**Before:**

```rust
pub struct OrphanContainer {
    analyzer: Arc<dyn IOrphanAggregate>,
    layer_detector: Arc<dyn ILayerDetectionAggregate>,
}
```

**After:**

```rust
pub struct OrphanContainer {
    analyzer: Arc<dyn IOrphanAggregate>,
}

impl OrphanContainer {
    pub fn new_with_config(
        config: shared::config_system::taxonomy_config_vo::ArchitectureConfig,
    ) -> Self {
        let resolver: Arc<dyn IOrphanGraphResolverProtocol> = Arc::new(OrphanGraphResolver::new());
        let layer_detector: Arc<dyn ILayerDetectionProtocol> = Arc::new(crate::capabilities_layer_detector::CapabilitiesLayerDetector);
        let arch = Arc::new(ArchOrphanAnalyzer::new_with_config(
            resolver,
            layer_detector,
            Arc::new(crate::capabilities_orphan_taxonomy_analyzer::TaxonomyOrphanAnalyzer::new()),
            Arc::new(crate::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer::new()),
            Arc::new(crate::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer::new()),
            Arc::new(crate::capabilities_orphan_utility_analyzer::UtilityOrphanAnalyzer::new()),
            Arc::new(crate::capabilities_orphan_agent_analyzer::AgentOrphanAnalyzer::new()),
            Arc::new(crate::capabilities_orphan_surfaces_analyzer::SurfacesOrphanAnalyzer::new()),
            config,
        ));
        Self {
            analyzer: arch.clone() as Arc<dyn IOrphanAggregate>,
        }
    }
}
```

### P3.3 — Update IOrphanAggregate to not require ILayerDetectionAggregate

**Skill:** `create-contract-rust` — removing parameter from trait, must remain object-safe.

**File:** `crates/shared/src/orphan-detector/contract_orphan_aggregate.rs`

**Before:**

```rust
pub trait IOrphanAggregate: Send + Sync {
    fn check_orphans(
        &self,
        layer_detector: &dyn ILayerDetectionAggregate,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult>;
}
```

**After:**

```rust
pub trait IOrphanAggregate: Send + Sync {
    fn check_orphans(&self, files: &[String], root_dir: &str) -> Vec<LintResult>;
}
```

### P3.4 — Update check_orphans to call utility_layer_detector directly

**Architecture update:** Agent layer may now import Utility directly (ARCHITECTURE.md §9 updated, 3 config YAMLs updated). No need for ILayerDetectionProtocol wrapper.

**Approach:** Call `shared::common::utility_layer_detector` functions directly in `check_orphans`.

**Updated `check_orphans`:**

```rust
fn check_orphans(&self, files: &[String], root_dir: &str) -> Vec<LintResult> {
    if !self.config.enabled.value { return Vec::new(); }

    let ignored: Vec<String> = self.config.ignored_paths.values.iter()
        .map(|p| p.value().to_string())
        .collect();
    let filtered_files: Vec<String> = files
        .iter()
        .filter(|f| !shared::orphan_detector::utility_orphan_path::is_path_ignored(f, &ignored))
        .cloned()
        .collect();
    let files = filtered_files.as_slice();

    let file_vo = shared::orphan_detector::OrphanFileListVO::new(files.to_vec());
    let context = self.resolver.build_graph_context(&[file_vo], root_dir);

    let configured = self.get_orphan_entry_points();
    let configured_vo = shared::orphan_detector::OrphanEntryPatternListVO::new(configured);
    let entry_points = self.resolver.identify_entry_points(&[file_vo], &[configured_vo]);
    let alive_files_set = self._trace_reachability(&entry_points.values, &context.import_graph);

    let mut results = Vec::new();
    for f in files {
        let file_fp = match FilePath::new(f.clone()) { Ok(fp) => fp, Err(_) => continue };

        let filename = shared::common::utility_layer_detector::extract_filename(file_fp.value());
        let base_layer = match shared::common::utility_layer_detector::detect_layer_from_prefix(filename) {
            Some(l) => l,
            None => continue,
        };
        let layer_keys: Vec<String> = self.config.layers.keys().map(|k| k.value.to_string()).collect();
        let layer_str = shared::common::utility_layer_detector::resolve_specialized_layer(
            &base_layer, file_fp.value(), &layer_keys,
        );
        let definition = match shared::common::utility_layer_detector::get_layer_def(&layer_str, &self.config.layers) {
            Some(d) => d.clone(),
            None => continue,
        };

        let basename = file_fp.basename();
        if definition.exceptions.values.contains(&basename) { continue; }
        if !definition.orphan.check_orphan.value { continue; }

        let layer_vo = LayerNameVO::new(&layer_str);
        let res = self._evaluate_layer(f, &context, &alive_files_set, &layer_vo, files, root_dir);

        if res.is_orphan {
            let code = match layer_str.to_lowercase() {
                s if s.contains(LAYER_TAXONOMY) => "AES501",
                s if s.contains(LAYER_CONTRACT) => "AES502",
                s if s.contains(LAYER_CAPABILITIES) => "AES503",
                s if s.contains(LAYER_UTILITY) => "AES504",
                s if s.contains(LAYER_AGENT) => "AES505",
                s if s.contains(LAYER_SURFACES) => "AES506",
                _ => continue,
            };
            results.push(self._make_result(f, &res.reason, res.severity, code));
        }
    }
    results
}
```

### P3.5 — Update all callers of check_orphans

**Skill:** `create-surface-rust` — surface layer callers must update to match new aggregate signature.

**Files to update:**

- `crates/cli-commands/src/surface_check_command.rs` — remove `layer_detector` parameter from `check_orphans` calls
- `crates/mcp-server/src/agent_mcp_server_orchestrator.rs` — same
- `crates/tui/src/capabilities_lint_executor.rs` — same

---

## Phase 4: Correctness Fixes (High)

### P4.1 — Fix graph resolver entry-point detection (missing _container.*)

**Skill:** `create-capabilities-rust` — protocol implementation fix, allowed.

**File:** `crates/orphan-detector/src/capabilities_orphan_graph_resolver.rs`

**Problem:** Lines 55-73: Default entry-point detection omits `_container.*` when no configured patterns are supplied.

**Before (lines 55-73):**

```rust
let matched: Vec<String> = if configured_strs.is_empty() {
    file_strs.iter().filter(|f| {
        let basename = f.rsplit('/').next().unwrap_or(f);
        basename.ends_with("_entry.rs")
            || basename.ends_with("_entry.py")
            || basename.ends_with("_entry.ts")
            || basename.ends_with("_entry.js")
            || basename.starts_with("root_")
            || basename == "main.rs"
            || basename == "lib.rs"
            || basename == "main.py"
            || basename == "__main__.py"
            || basename == "index.ts"
            || basename == "index.js"
    }).cloned().collect()
```

**After:**

```rust
let matched: Vec<String> = if configured_strs.is_empty() {
    file_strs.iter().filter(|f| {
        let basename = f.rsplit('/').next().unwrap_or(f);
        basename.ends_with("_container.rs")
            || basename.ends_with("_container.py")
            || basename.ends_with("_container.ts")
            || basename.ends_with("_container.js")
            || basename.ends_with("_entry.rs")
            || basename.ends_with("_entry.py")
            || basename.ends_with("_entry.ts")
            || basename.ends_with("_entry.js")
            || basename.starts_with("root_")
            || basename == "main.rs"
            || basename == "lib.rs"
            || basename == "main.py"
            || basename == "__main__.py"
            || basename == "main.ts"
            || basename == "main.js"
            || basename == "index.ts"
            || basename == "index.js"
    }).cloned().collect()
```

### P4.2 — Fix workspace import resolution (hyphenated dirs)

**Skill:** `create-capabilities-rust` — adding helper methods to capabilities, allowed.

**File:** `crates/orphan-detector/src/capabilities_orphan_graph_resolver.rs`

**Problem:** Lines 414-443: Uses `stem == module_name` without normalizing hyphens. `orphan-detector` directory won't match `orphan_detector` module name.

**Before (lines 421-441):**

```rust
if let Some(src_dir) = crate_src_dirs.get(crate_name) {
    let entries = shared::orphan_detector::utility_orphan_io::scan_directory(src_dir);
    for (_name, path_str, _is_dir) in entries {
        let path = std::path::PathBuf::from(&path_str);
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or_default();
        if stem == module_name && path_str != *f {
            // ... add edge
        }
    }
}
```

**After:** Use the pre-built `crate_module_index` from QA report approach:

```rust
// Build crate module index once (add to build_graph_context_inner)
let crate_module_index = Self::build_crate_module_index(&crate_src_dirs);

// Then resolve using:
if let Some(resolved) = Self::resolve_workspace_module(
    &crate_module_index, crate_name, &segments, f,
) {
    Self::add_edge(&mut import_graph, &mut inbound_links, f, &resolved);
    continue;
}
```

Add helper methods:

```rust
fn build_crate_module_index(
    crate_src_dirs: &HashMap<String, std::path::PathBuf>,
) -> HashMap<String, HashMap<String, String>> {
    // Build module_path -> file_path map per crate
    // Normalize hyphens to underscores
    // Handle mod.rs, __init__.py, index.ts
}

fn resolve_workspace_module(
    index: &HashMap<String, HashMap<String, String>>,
    crate_name: &str,
    segments: &[&str],
    current_file: &str,
) -> Option<String> {
    let map = index.get(crate_name)?;
    for i in (1..=segments.len()).rev() {
        let candidate = segments[..i].join("/");
        let normalized = shared::orphan_detector::utility_orphan::normalize_module_path(&candidate);
        if let Some(path) = map.get(&normalized) {
            let path = Self::normalize_file_path(path);
            if path != current_file { return Some(path); }
        }
    }
    None
}
```

### P4.3 — Fix utility orphan detection (naive substring matching)

**Skill:** `create-utility-rust` — new functions in `utility_orphan.rs` must be stateless, standalone, domain-agnostic (or documented as language-specific parsers).

**File:** `crates/orphan-detector/src/capabilities_orphan_utility_analyzer.rs`

**Problem:** Lines 115-118: Uses `content.contains(&format!("use {}", module_name))` which misses `shared::orphan_detector::utility_orphan_io`.

**Before:**

```rust
fn check_import_pattern(&self, content: &str, module_name: &str) -> bool {
    if content.contains(&format!("use {}", module_name))
        || content.contains(&format!("use {}::", module_name))
        || content.contains(&format!("use crate::{}", module_name))
        || content.contains(&format!("use shared::{}", module_name))
    { return true; }
    // ...
}
```

**After:** Add shared utility functions to `utility_orphan.rs`:

```rust
// In crates/shared/src/orphan-detector/utility_orphan.rs

pub fn normalize_module_component(value: &str) -> String {
    value.replace('-', "_").replace('.', "_")
}

pub fn normalize_module_path(value: &str) -> String {
    value.split('/')
        .map(normalize_module_component)
        .collect::<Vec<_>>()
        .join("/")
}

pub fn contains_delimited(content: &str, token: &str) -> bool {
    // Check if token appears as a delimited path-like token
    // Avoids substring false positives like "utility_orphan" matching "utility_orphan_filename"
}

pub fn import_tokens(path: &str) -> Vec<String> {
    // Generate likely import tokens for a source file
    // e.g., "crates/shared/src/orphan-detector/utility_orphan_io.rs"
    // produces: ["utility_orphan_io", "orphan_detector/utility_orphan_io",
    //            "orphan_detector::utility_orphan_io", "shared::orphan_detector::utility_orphan_io", ...]
}
```

Then update utility analyzer to use graph first, fallback to token matching:

```rust
impl IUtilityOrphanProtocol for UtilityOrphanAnalyzer {
    fn is_utility_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        all_files: &[String],
        inbound_links: &InboundLinkMap,  // ADD THIS PARAMETER
    ) -> OrphanIndicatorResult {
        // Fast path: use already-built import graph
        if let Some(importers) = inbound_links.mapping.get(f.value()) {
            if importers.iter().any(|importer| importer != f.value()) {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
            }
        }

        // Fallback: token-based matching
        let tokens = import_tokens(f.value());
        for other_file in all_files {
            if other_file == f.value() { continue; }
            let content = read_file_safe(other_file);
            if tokens.iter().any(|token| contains_delimited(&content, token)) {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
            }
        }

        OrphanIndicatorResult::new(true, /* ... */, Severity::HIGH)
    }
}
```

**Update protocol in `contract_orphan_protocol.rs`:**

```rust
pub trait IUtilityOrphanProtocol: Send + Sync {
    fn is_utility_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        all_files: &[String],
        inbound_links: &InboundLinkMap,  // ADD
    ) -> OrphanIndicatorResult;
}
```

**Update call site in `orchestrator.rs:291-294`:**

```rust
if layer_str.contains(LAYER_UTILITY) {
    return self.utility_analyzer.is_utility_orphan(
        &fp, &root, all_files, &context.inbound_links,
    );
}
```

### P4.4 — Fix contract analyzer (only extracts first trait)

**Skill:** `create-capabilities-rust` — protocol implementation fix, allowed.

**File:** `crates/orphan-detector/src/capabilities_orphan_contract_analyzer.rs`

**Problem:** Lines 241-270: Uses `re.captures()` not `captures_iter()`, so only first trait is extracted.

**Before:**

```rust
fn extract_contract_trait_name(content: &str) -> Option<String> {
    // ...
    if let Some(re) = Self::re_contract_rust() {
        if let Some(caps) = re.captures(&code_lines) {
            return Some(caps[1].to_string());  // Only first match
        }
    }
    // ...
}
```

**After:**

```rust
fn extract_contract_trait_names(content: &str) -> Vec<String> {
    let code_lines: String = content.lines()
        .filter(|l| { let t = l.trim(); !t.starts_with("//") && !t.starts_with("/*") && !t.starts_with("*") })
        .collect::<Vec<_>>().join("\n");

    let mut traits = Vec::new();
    if let Some(re) = Self::re_contract_rust() {
        for caps in re.captures_iter(&code_lines) {
            traits.push(caps[1].to_string());
        }
    }
    if let Some(re) = Self::re_ts_interface_export() {
        for caps in re.captures_iter(&code_lines) {
            traits.push(caps[1].to_string());
        }
    }
    if let Some(re) = Self::re_interface() {
        for caps in re.captures_iter(&code_lines) {
            traits.push(caps[1].to_string());
        }
    }
    if let Some(re) = Self::re_contract_py() {
        for caps in re.captures_iter(&code_lines) {
            traits.push(caps[1].to_string());
        }
    }
    traits.sort();
    traits.dedup();
    traits
}
```

Then evaluate ALL traits:

```rust
let trait_names = Self::extract_contract_trait_names(&content);
if trait_names.is_empty() {
    return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
}

let mut unimplemented = Vec::new();
for trait_name in &trait_names {
    if !Self::has_implementation(&search_files, trait_name) {
        unimplemented.push(trait_name.clone());
    }
}
if unimplemented.is_empty() {
    return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
}
// Report unimplemented traits
```

### P4.5 — Fix contract analyzer implementation detection for Python/TS

**Skill:** `create-capabilities-rust` — adding utility function `has_trait_implementation` to shared, documented as language-specific parser.

**File:** `crates/orphan-detector/src/capabilities_orphan_contract_analyzer.rs`

**Problem:** Lines 72-83: Uses `c.contains(&format!("impl {} for", trait_name))` which misses Python `class Foo(Trait):` and TS `implements`.

**Before:**

```rust
if c.contains(&format!("impl {} for", trait_name))
    || c.lines().any(|ln| {
        let t = ln.trim();
        t.starts_with("impl") && t.contains(&trait_name) && t.contains(" for")
    })
    || c.contains(&format!("class {}(\\(", trait_name))
    || c.contains(&format!("class {} ", trait_name))
    || c.contains(&format!("class {}:", trait_name))
```

**After:** Add `has_trait_implementation` to `utility_orphan.rs`:

**Note:** This function is language-specific (Rust/Python/TS pattern matching), not truly domain-agnostic. Acceptable as a technical parser (like `utility_orphan_io`), but document it as multi-language, not generic.

```rust
pub fn has_trait_implementation(content: &str, trait_name: &str) -> bool {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') || trimmed.starts_with('#') {
            continue;
        }
        // Rust: impl Trait for Type / impl<T> Trait for Type
        if trimmed.starts_with("impl") && trimmed.contains(" for ") {
            // ... parse trait name from impl block
        }
        // Python: class Foo(Trait): / class Foo(Base, Trait):
        if let Some(class_pos) = trimmed.find("class ") {
            // ... parse bases from parentheses
        }
        // TS: class Foo implements Trait
        if let Some(impl_pos) = trimmed.find(" implements ") {
            // ... parse implemented interfaces
        }
    }
    false
}
```

### P4.6 — Fix agent analyzer (missing entry point patterns)

**Skill:** `create-capabilities-rust` — protocol implementation fix, allowed.

**File:** `crates/orphan-detector/src/capabilities_orphan_agent_analyzer.rs`

**Problem:** Lines 46-53: Only checks `surface_*` and `*_container.*`, misses `main.rs`, `lib.rs`, `index.ts`, `_entry.*`.

**Before:**

```rust
let is_surface = cb.starts_with("surface_");
let is_container = cb.ends_with("_container.rs")
    || cb.ends_with("_container.py")
    || cb.ends_with("_container.ts")
    || cb.ends_with("_container.js");

if !is_surface && !is_container { continue; }
```

**After:**

```rust
fn is_caller_file(basename: &str) -> bool {
    basename.starts_with("surface_")
        || basename.starts_with("root_")
        || basename.ends_with("_container.rs")
        || basename.ends_with("_container.py")
        || basename.ends_with("_container.ts")
        || basename.ends_with("_container.js")
        || basename.ends_with("_entry.rs")
        || basename.ends_with("_entry.py")
        || basename.ends_with("_entry.ts")
        || basename.ends_with("_entry.js")
        || matches!(basename,
            "main.rs" | "lib.rs" | "main.py" | "__main__.py"
            | "main.ts" | "main.js" | "index.ts" | "index.js"
        )
}

// In is_agent_orphan:
if !Self::is_caller_file(cb) { continue; }
```

### P4.7 — Fix surfaces analyzer (missing entry patterns in fallback)

**Skill:** `create-capabilities-rust` — protocol implementation fix, allowed.

**File:** `crates/orphan-detector/src/capabilities_orphan_surfaces_analyzer.rs`

**Problem:** Lines 116-120: Fallback importer search misses `main.rs`, `lib.rs`, `index.ts`, `_container.*`.

**Before:**

```rust
let is_entry_or_router = name.starts_with("root_")
    || name.starts_with("cli_")
    || name.starts_with("mcp_")
    || name.contains("_entry")
    || name.contains("_router");
```

**After:**

```rust
let is_entry_or_router = name.starts_with("root_")
    || name.starts_with("cli_")
    || name.starts_with("mcp_")
    || name.starts_with("surface_")
    || name.contains("_entry")
    || name.contains("_router")
    || name.contains("_container")
    || matches!(name,
        "main.rs" | "lib.rs" | "main.py" | "__main__.py"
        | "main.ts" | "main.js" | "index.ts" | "index.js"
    );
```

### P4.8 — Fix taxonomy analyzer (non-empty message when not orphan)

**Skill:** `create-capabilities-rust` — protocol implementation fix, allowed.

**File:** `crates/orphan-detector/src/capabilities_orphan_taxonomy_analyzer.rs`

**Problem:** Lines 98-107: Always constructs `AesOrphanViolation` even when `is_orphan=false`.

**Before:**

```rust
OrphanIndicatorResult::new(
    is_orphan,
    AesOrphanViolation::TaxonomyOrphan {
        stem,
        category,
        reason: None,
    }.to_string(),
    Severity::LOW,
)
```

**After:**

```rust
if is_orphan {
    OrphanIndicatorResult::new(
        true,
        AesOrphanViolation::TaxonomyOrphan {
            stem,
            category,
            reason: Some(format!("Taxonomy '{}' is not imported by any file outside taxonomy.", stem).into()),
        }.to_string(),
        Severity::LOW,
    )
} else {
    OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
}
```

---

## Phase 5: Performance Fixes (High)

### P5.1 — Cache contract analyzer search_files

**Skill:** `create-capabilities-rust` — caching is state ownership within execution scope, allowed in capabilities.

**File:** `crates/orphan-detector/src/capabilities_orphan_contract_analyzer.rs`

**Problem:** Lines 45-52: Recursively collects workspace source files for every contract file.

**Fix:** Add `Mutex<Option<SearchFilesCache>>` to struct:

```rust
struct SearchFilesCache {
    root: std::path::PathBuf,
    file_count: usize,
    files: Arc<Vec<String>>,
}

pub struct ContractOrphanAnalyzer {
    search_cache: Mutex<Option<SearchFilesCache>>,
}

fn search_files(&self, root_dir: &FilePath, all_files: &[String]) -> Arc<Vec<String>> {
    let root = std::path::Path::new(root_dir.value()).to_path_buf();
    if let Ok(guard) = self.search_cache.lock() {
        if let Some(cache) = guard.as_ref() {
            if cache.root == root && cache.file_count == all_files.len() {
                return cache.files.clone();
            }
        }
    }
    // ... build and cache
}
```

### P5.2 — Cache capabilities analyzer container_files

**Skill:** `create-capabilities-rust` — caching is state ownership within execution scope, allowed in capabilities.

**File:** `crates/orphan-detector/src/capabilities_orphan_capabilities_analyzer.rs`

**Problem:** Lines 57-63: Calls `find_workspace_root` + `check_wired_in_container` per file.

**Fix:** Add `Mutex<Option<(PathBuf, Vec<PathBuf>)>>` cache:

```rust
pub struct CapabilitiesOrphanAnalyzer {
    container_cache: Mutex<Option<(std::path::PathBuf, Vec<PathBuf>)>>,
}

fn container_files(&self, root_dir: &FilePath) -> Option<Vec<PathBuf>> {
    // Cache by root_dir, return cached if same root
}
```

### P5.3 — Use HashSet for alive set lookup in surfaces analyzer

**Skill:** `create-capabilities-rust` — performance optimization within capabilities, allowed.

**File:** `crates/orphan-detector/src/capabilities_orphan_surfaces_analyzer.rs`

**Problem:** Lines 24-29: Clones alive set to `Vec<String>` and uses linear `.contains()`.

**Before:**

```rust
let alive: Vec<String> = alive_files.paths.iter().map(|fp| fp.value().to_string()).collect();
let is_reachable = alive.contains(&f.value().to_string());
```

**After:**

```rust
let is_reachable = alive_files.paths.contains(f);
```

(`ReachabilityResult.paths` is already a `Vec<FilePath>` — `contains` on `FilePath` should work if `PartialEq` is implemented, otherwise convert to `HashSet` first.)

---

## Phase 6: Agent Layer — Utility imports now allowed (Resolved)

### P6.1 — Agent→Utility imports allowed by architecture update

**Decision:** ARCHITECTURE.md §9 updated to allow Agent to depend on Utility. Config YAMLs (rust/python/javascript) updated — `agent(orchestrator)` `allowed` includes `"utility"`.

**Rationale:** Agent orchestrator needs utility functions for graph traversal (`_trace_reachability`, `_evaluate_layer`) and layer detection. Wrapping these in a Capabilities layer just to satisfy the old Agent→Utility rule added unnecessary complexity (`ILayerDetectionProtocol` contract + `CapabilitiesLayerDetector`). Simpler to allow Agent→Utility directly.

**Files deleted:**

- `crates/shared/src/orphan-detector/contract_layer_detection_protocol.rs`
- `crates/shared/src/code-analysis/contract_layer_detection_protocol.rs`
- `crates/orphan-detector/src/capabilities_layer_detector.rs`

**Config changes:** `"utility"` added to agent's `allowed` and `allowed_imports` in all 3 config YAMLs.

**ARCHITECTURE.md §9 updated:** "Agent may depend only on Taxonomy, Contract, and Utility."

---

## Phase 7: Severity Adjustments (Low)

### P7.1 — Update severity assignments

**Skill:** `create-capabilities-rust` — config/severity change, no layer violation.

**File:** `crates/orphan-detector/src/capabilities_orphan_contract_analyzer.rs`

**Before:** Contract orphan severity = `Severity::LOW`

**After:** Contract orphan severity = `Severity::MEDIUM`

**File:** `crates/orphan-detector/src/capabilities_orphan_utility_analyzer.rs`

**Before:** Utility orphan severity = `Severity::HIGH`

**After:** Utility orphan severity = `Severity::MEDIUM`

**Severity mapping after fix:**

| Layer        | Code   | Severity |
| ------------ | ------ | -------- |
| Taxonomy     | AES501 | LOW      |
| Contract     | AES502 | MEDIUM   |
| Capabilities | AES503 | MEDIUM   |
| Utility      | AES504 | MEDIUM   |
| Agent        | AES505 | HIGH     |
| Surface      | AES506 | HIGH     |

---

## Phase 8: Error Handling (Medium)

### P8.1 — Return diagnostics for unreadable files

**Skill:** `create-utility-rust` — stateless enum + free function, domain-agnostic.

**File:** `crates/shared/src/orphan-detector/utility_orphan_io.rs`

**Problem:** `read_file_safe` silently returns empty string on error.

**Add:**

```rust
pub enum FileReadOutcome {
    Content(String),
    Unreadable { path: String, reason: String },
}

pub fn read_file_with_diagnostic(path: &str) -> FileReadOutcome {
    match std::fs::read_to_string(path) {
        Ok(content) => FileReadOutcome::Content(content),
        Err(err) => FileReadOutcome::Unreadable {
            path: path.to_string(),
            reason: err.to_string(),
        },
    }
}
```

**Note:** Keep `read_file_safe` for backward compatibility. Add `read_file_with_diagnostic` for new code that wants diagnostics.

---

## Execution Order

1. **Phase 1** (P1.1-P1.3): Security — path confinement. No dependencies.
   - **Verify:** `cargo check -p shared && cargo check -p orphan-detector`
2. **Phase 2** (P2.1-P2.3): Config — wire ignored_paths, inject real config. No dependencies.
   - **Verify:** `cargo check -p orphan-detector`
3. **Phase 3** (P3.1-P3.5): Remove ILayerDetectionAggregate, call utility_layer_detector directly. Depends on Phase 2.
   - **Verify:** `cargo check -p shared && cargo check -p orphan-detector`
4. **Phase 4** (P4.1-P4.8): Correctness fixes. Can run in parallel with Phase 3.
   - **Verify:** `cargo check -p orphan-detector`
5. **Phase 5** (P5.1-P5.3): Performance. Can run in parallel with Phase 4.
   - **Verify:** `cargo check -p orphan-detector`
6. **Phase 6** (P6.1): Agent→Utility imports allowed — architecture configs and docs updated.
   - **Status:** Resolved. ARCHITECTURE.md §9 + 3 config YAMLs updated. No wrapper needed.
   - **Verify:** `cargo run --bin lint-arwaky-cli -- check .` — 0 agent-utility import violations
7. **Phase 7** (P7.1): Severity. Independent, can run anytime.
8. **Phase 8** (P8.1): Error handling. Independent, can run anytime.

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

- `crates/shared/src/orphan-detector/utility_orphan_path.rs` — path confinement + `is_path_ignored`
- `crates/shared/src/orphan-detector/utility_orphan.rs` — add `normalize_module_component`, `normalize_module_path`, `contains_delimited`, `import_tokens`, `has_trait_implementation`

### Deleted files (3)

- `crates/shared/src/orphan-detector/contract_layer_detection_protocol.rs` — removed (Agent→Utility direct, no wrapper needed)
- `crates/shared/src/code-analysis/contract_layer_detection_protocol.rs` — removed (dead code, duplicate)
- `crates/orphan-detector/src/capabilities_layer_detector.rs` — removed (no longer needed)

### Modified files (12)

- `crates/shared/src/orphan-detector/contract_orphan_protocol.rs` — update IUtilityOrphanProtocol, ISurfacesOrphanProtocol signatures
- `crates/shared/src/orphan-detector/contract_orphan_aggregate.rs` — remove layer_detector param from check_orphans
- `crates/shared/src/orphan-detector/mod.rs` — add utility_orphan_path module
- `crates/shared/src/code-analysis/mod.rs` — remove contract_layer_detection_protocol
- `crates/orphan-detector/src/agent_orphan_orchestrator.rs` — remove ILayerDetectionAggregate impl + ILayerDetectionProtocol DI, call utility_layer_detector directly
- `crates/orphan-detector/src/root_orphan_detector_container.rs` — wire config, remove CapabilitiesLayerDetector wiring
- `crates/orphan-detector/src/lib.rs` — remove capabilities_layer_detector module
- `crates/orphan-detector/src/capabilities_orphan_graph_resolver.rs` — path confinement, fix entry points, fix workspace import resolution
- `crates/orphan-detector/src/capabilities_orphan_utility_analyzer.rs` — use inbound_links, token-based matching
- `crates/orphan-detector/src/capabilities_orphan_contract_analyzer.rs` — extract all traits, fix impl detection, cache search_files, severity
- `crates/orphan-detector/src/capabilities_orphan_agent_analyzer.rs` — add entry point patterns
- `crates/orphan-detector/src/capabilities_orphan_surfaces_analyzer.rs` — fix CWD, add entry patterns, use HashSet
- `crates/orphan-detector/src/capabilities_orphan_taxonomy_analyzer.rs` — fix non-orphan message
- `crates/orphan-detector/src/capabilities_orphan_capabilities_analyzer.rs` — cache container_files
