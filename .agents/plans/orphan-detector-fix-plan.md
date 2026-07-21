# Orphan Detector Fix Plan

> Generated from BA, QA, and Backend reports. All findings validated against source code.
> User decisions applied: AES503=architecture correct, Agent-Utility=violation, Performance=hybrid, ILayerDetectionAggregate=remove, Path security=include, Severity=adjust.

---

## Summary

| Metric | Value |
|---|---|
| Total issues | 28 |
| Critical (security/correctness) | 8 |
| High (performance/architecture) | 10 |
| Medium (code quality) | 6 |
| Low (documentation/severity) | 4 |
| New files to create | 2 |
| Files to modify | 10 |

---

## Phase 1: Security & Path Confinement (Critical)

### P1.1 — Add path confinement utility

**File:** `crates/shared/src/orphan-detector/utility_orphan_path.rs` (NEW)

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

### P1.2 — Use path confinement in graph resolver

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

**File:** `crates/orphan-detector/src/agent_orphan_orchestrator.rs`

**Add method:**
```rust
fn is_ignored(&self, file: &str) -> bool {
    let file = file.replace('\\', "/");
    self.config.ignored_paths.values.iter().any(|pattern| {
        let raw = pattern.value().replace('\\', "/");
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

**In `check_orphans`, add after config gate:**
```rust
let filtered_files: Vec<String> = files
    .iter()
    .filter(|f| !self.is_ignored(f))
    .cloned()
    .collect();
let files = filtered_files.as_slice();
```

---

## Phase 3: Remove ILayerDetectionAggregate from Orphan Detector (High)

### P3.1 — Remove ILayerDetectionAggregate impl from ArchOrphanAnalyzer

**File:** `crates/orphan-detector/src/agent_orphan_orchestrator.rs`

**Problem:** `ArchOrphanAnalyzer` implements `ILayerDetectionAggregate` with static defaults. Layer detection is just prefix matching — should use `utility_layer_detector` directly.

**Remove:** The entire `impl ILayerDetectionAggregate for ArchOrphanAnalyzer` block (lines 311-375).

### P3.2 — Update OrphanContainer to not use ILayerDetectionAggregate

**File:** `crates/orphan-detector/src/root_orphan_detector_container.rs`

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
        let arch = Arc::new(ArchOrphanAnalyzer::new_with_config(
            resolver,
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

### P3.4 — Update check_orphans to use utility_layer_detector directly

**File:** `crates/orphan-detector/src/agent_orphan_orchestrator.rs`

Replace the `check_orphans` implementation to use `utility_layer_detector` functions:

```rust
fn check_orphans(&self, files: &[String], root_dir: &str) -> Vec<LintResult> {
    if !self.config.enabled.value { return Vec::new(); }

    let filtered_files: Vec<String> = files
        .iter()
        .filter(|f| !self.is_ignored(f))
        .cloned()
        .collect();
    let files = filtered_files.as_slice();

    let file_vo = shared::orphan_detector::OrphanFileListVO::new(files.to_vec());
    let context = self.resolver.build_graph_context(&[file_vo], root_dir);

    let configured = self.get_orphan_entry_points();
    let configured_vo = shared::orphan_detector::OrphanEntryPatternListVO::new(configured);
    let entry_points = self.resolver.identify_entry_points(&[file_vo], &[configured_vo]);
    let alive_files_set = self._trace_reachability(&entry_points.values, &context.import_graph);

    let layer_keys: Vec<String> = self.config.layers.keys().map(|k| k.value.to_string()).collect();

    let mut results = Vec::new();
    for f in files {
        let file_fp = match FilePath::new(f.clone()) { Ok(fp) => fp, Err(_) => continue };

        // Use utility_layer_detector directly
        let filename = shared::common::utility_layer_detector::extract_filename(f);
        let base_layer = match shared::common::utility_layer_detector::detect_layer_from_prefix(filename) {
            Some(l) => l,
            None => continue,
        };
        let layer_str = shared::common::utility_layer_detector::resolve_specialized_layer(
            &base_layer, f, &layer_keys,
        );

        let definition = match shared::common::utility_layer_detector::get_layer_def(
            &layer_str, &self.config.layers
        ) {
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

fn get_orphan_entry_points(&self) -> Vec<String> {
    let mut entry_points = vec![
        "_container.rs".into(), "_container.py".into(),
        "_container.ts".into(), "_container.js".into(),
        "_entry.rs".into(), "_entry.py".into(),
        "_entry.ts".into(), "_entry.js".into(),
        "main.rs".into(), "lib.rs".into(),
        "main.py".into(), "__main__.py".into(),
        "main.ts".into(), "main.js".into(),
        "index.ts".into(), "index.js".into(),
    ];
    for layer_def in self.config.layers.values() {
        entry_points.extend(layer_def.orphan.orphan_entry_points.values.iter().cloned());
    }
    entry_points.sort();
    entry_points.dedup();
    entry_points
}
```

### P3.5 — Update all callers of check_orphans

**Files to update:**
- `crates/cli-commands/src/surface_check_command.rs` — remove `layer_detector` parameter from `check_orphans` calls
- `crates/mcp-server/src/agent_mcp_server_orchestrator.rs` — same
- `crates/tui/src/capabilities_lint_executor.rs` — same

---

## Phase 4: Correctness Fixes (High)

### P4.1 — Fix graph resolver entry-point detection (missing _container.*)

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

## Phase 6: Agent Layer Violation (Medium)

### P6.1 — Remove Utility imports from Agent orchestrator

**File:** `crates/orphan-detector/src/agent_orphan_orchestrator.rs`

**Problem:** Agent layer imports `utility_orphan_io`, `utility_orphan_filename`, `utility_workspace`. ARCHITECTURE.md §9 says Agent may depend only on Taxonomy and Contract.

**This is blocked by P3.4** — once `check_orphans` uses `utility_layer_detector` (which is in shared/common, not orphan-detector utility), the direct utility imports from the orchestrator are reduced. The remaining utility calls (in `_trace_reachability`, `_evaluate_layer`) are for graph operations which should move to Capabilities layer.

**Deferred:** Full refactoring of utility calls out of Agent layer requires moving graph traversal logic into a new `CapabilitiesOrphanGraphTraversal` capability. Track as follow-up task.

---

## Phase 7: Severity Adjustments (Low)

### P7.1 — Update severity assignments

**File:** `crates/orphan-detector/src/capabilities_orphan_contract_analyzer.rs`

**Before:** Contract orphan severity = `Severity::LOW`

**After:** Contract orphan severity = `Severity::MEDIUM`

**File:** `crates/orphan-detector/src/capabilities_orphan_utility_analyzer.rs`

**Before:** Utility orphan severity = `Severity::HIGH`

**After:** Utility orphan severity = `Severity::MEDIUM`

**Severity mapping after fix:**
| Layer | Code | Severity |
|---|---|---|
| Taxonomy | AES501 | LOW |
| Contract | AES502 | MEDIUM |
| Capabilities | AES503 | MEDIUM |
| Utility | AES504 | MEDIUM |
| Agent | AES505 | HIGH |
| Surface | AES506 | HIGH |

---

## Phase 8: Error Handling (Medium)

### P8.1 — Return diagnostics for unreadable files

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
2. **Phase 2** (P2.1-P2.3): Config — wire ignored_paths, inject real config. No dependencies.
3. **Phase 3** (P3.1-P3.5): Remove ILayerDetectionAggregate. Depends on Phase 2 (real config).
4. **Phase 4** (P4.1-P4.8): Correctness fixes. Can run in parallel with Phase 3.
5. **Phase 5** (P5.1-P5.3): Performance. Can run in parallel with Phase 4.
6. **Phase 6** (P6.1): Agent layer violation. Depends on Phase 3 and 4.
7. **Phase 7** (P7.1): Severity. Independent, can run anytime.
8. **Phase 8** (P8.1): Error handling. Independent, can run anytime.

---

## Files Summary

### New files (2)
- `crates/shared/src/orphan-detector/utility_orphan_path.rs` — path confinement
- `crates/shared/src/orphan-detector/utility_orphan.rs` — add `normalize_module_component`, `normalize_module_path`, `contains_delimited`, `import_tokens`, `has_trait_implementation`

### Modified files (10)
- `crates/shared/src/orphan-detector/contract_orphan_protocol.rs` — update IUtilityOrphanProtocol, ISurfacesOrphanProtocol signatures
- `crates/shared/src/orphan-detector/contract_orphan_aggregate.rs` — remove layer_detector param from check_orphans
- `crates/shared/src/orphan-detector/mod.rs` — add utility_orphan_path module
- `crates/orphan-detector/src/agent_orphan_orchestrator.rs` — remove ILayerDetectionAggregate impl, add config, add is_ignored, use utility_layer_detector
- `crates/orphan-detector/src/root_orphan_detector_container.rs` — wire config, remove layer_detector field
- `crates/orphan-detector/src/capabilities_orphan_graph_resolver.rs` — path confinement, fix entry points, fix workspace import resolution
- `crates/orphan-detector/src/capabilities_orphan_utility_analyzer.rs` — use inbound_links, token-based matching
- `crates/orphan-detector/src/capabilities_orphan_contract_analyzer.rs` — extract all traits, fix impl detection, cache search_files, severity
- `crates/orphan-detector/src/capabilities_orphan_agent_analyzer.rs` — add entry point patterns
- `crates/orphan-detector/src/capabilities_orphan_surfaces_analyzer.rs` — fix CWD, add entry patterns, use HashSet
- `crates/orphan-detector/src/capabilities_orphan_taxonomy_analyzer.rs` — fix non-orphan message
- `crates/orphan-detector/src/capabilities_orphan_capabilities_analyzer.rs` — cache container_files
