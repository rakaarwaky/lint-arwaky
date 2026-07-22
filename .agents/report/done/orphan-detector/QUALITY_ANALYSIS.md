Below is a QA/debugging review of `orphan-detector v1.10.106`, focused only on functional correctness and performance. No new AES rules or features are introduced. The fixes only make AES501–AES506 detection behave as specified and remove false positives / excessive I/O.

---

# 1. Defect Summary

| ID  | Type        | Location                                                             | Problem                                                                                                                                                                                                                                                              | Impact                                                                                               |
| --- | ----------- | -------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| B1  | Functional  | `capabilities_orphan_graph_resolver.rs`                              | Workspace imports such as`shared::orphan_detector::utility_orphan_io` or `shared::cli_commands::taxonomy_result_vo` are not resolved because hyphenated directories (`orphan-detector`, `cli-commands`) are not normalized and nested module paths are not resolved. | Broken import graph, false orphan warnings for taxonomy/utility/contract/surface files.              |
| B2  | Functional  | `capabilities_orphan_graph_resolver.rs`                              | Default entry-point detection omits`_container.*` when no configured patterns are supplied.                                                                                                                                                                          | Containers may be treated as non-entry points, causing false positives.                              |
| B3  | Functional  | `capabilities_orphan_utility_analyzer.rs`                            | Utility orphan detection uses naive substring checks like`use utility_orphan`, which can miss `shared::orphan_detector::utility_orphan_io` and can false-match `utility_orphan_filename`.                                                                            | False positives/negatives for AES504.                                                                |
| B4  | Performance | `capabilities_orphan_utility_analyzer.rs`                            | Reads every other file for every utility file: O(N²) uncached disk I/O.                                                                                                                                                                                              | Very slow on multi-crate workspaces.                                                                 |
| B5  | Functional  | `capabilities_orphan_contract_analyzer.rs`                           | Contract implementation detection is wrong for Python/TS. Example:`class Foo(IFooProtocol):` is not detected correctly; TS `implements` is not checked properly.                                                                                                     | False AES502 contract orphan warnings.                                                               |
| B6  | Performance | `capabilities_orphan_contract_analyzer.rs`                           | Recursively collects workspace source files for every contract file.                                                                                                                                                                                                 | O(contract_files × workspace_files) directory traversal.                                             |
| B7  | Functional  | `capabilities_orphan_agent_analyzer.rs`                              | Agent orphan detection only checks`surface_*` and `*_container.*`, but FRD says agent orchestrators may be called from binary entry points too (`main.rs`, `lib.rs`, `index.ts`, `_entry.*`).                                                                        | False AES505 warnings when agents are wired from entries.                                            |
| B8  | Functional  | `capabilities_orphan_surfaces_analyzer.rs`                           | Uses current working directory`Path::new(".")` to locate workspace root instead of deriving it from the analyzed file.                                                                                                                                               | False AES506 warnings when CWD is not workspace root.                                                |
| B9  | Functional  | `capabilities_orphan_surfaces_analyzer.rs`                           | Fallback importer search misses common entry files such as`main.rs`, `lib.rs`, `index.ts`, `_entry.*`, `_container.*`.                                                                                                                                               | False AES506 warnings for surfaces registered through entries/containers.                            |
| B10 | Performance | `capabilities_orphan_surfaces_analyzer.rs`                           | Converts the whole alive set into`Vec<String>` for every surface file and uses linear `.contains()`.                                                                                                                                                                 | Unnecessary allocation and O(N) lookup per surface.                                                  |
| B11 | Functional  | `capabilities_orphan_capabilities_analyzer.rs`                       | Calls`find_workspace_root` and scans containers for every unreachable capability.                                                                                                                                                                                    | Repeated directory traversal.                                                                        |
| B12 | Functional  | `agent_orphan_orchestrator.rs` + `root_orphan_detector_container.rs` | `new_with_ignored(ignored_paths)` ignores the provided ignored paths. `ArchitectureConfig.ignored_paths` is not used by the orphan pipeline.                                                                                                                         | Configured ignored path patterns are silently discarded.                                             |
| B13 | Functional  | `agent_orphan_orchestrator.rs`                                       | `ILayerDetectionAggregate::config()` always returns a static default enabled config, so disabling architecture checks through the aggregate is ineffective.                                                                                                          | Config gate is unreliable.                                                                           |
| B14 | Functional  | `capabilities_orphan_taxonomy_analyzer.rs`                           | Returns a non-empty violation message even when`is_orphan == false`. Also lacks the same-crate fallback for non-utility taxonomy files.                                                                                                                              | Misleading result payload; possible false positives when graph resolution misses same-crate imports. |

---

# 2. Shared Utility Fixes

These helpers centralize repeated parsing/matching logic and keep capabilities free of duplicated technical code.

## File: `crates/shared/src/orphan-detector/utility_orphan.rs`

Add the following stateless utility functions after the existing `extract_struct_names` / `extract_trait_names` functions.

```rust
use std::path::Path;

/// Normalize a module/path component for import resolution.
///
/// Examples:
///   "orphan-detector" -> "orphan_detector"
///   "cli-commands"    -> "cli_commands"
pub fn normalize_module_component(value: &str) -> String {
    value.replace('-', "_").replace('.', "_")
}

/// Normalize a slash-separated module path.
///
/// Example:
///   "orphan-detector/utility_orphan_io" -> "orphan_detector/utility_orphan_io"
pub fn normalize_module_path(value: &str) -> String {
    value
        .split('/')
        .map(normalize_module_component)
        .collect::<Vec<_>>()
        .join("/")
}

/// Checks whether `token` appears in `content` as a delimited path-like token.
///
/// This is safer than `content.contains(token)` because it avoids substring
/// false positives such as:
///   token = "utility_orphan"
///   content contains "utility_orphan_filename"
pub fn contains_delimited(content: &str, token: &str) -> bool {
    if token.is_empty() {
        return false;
    }

    let bytes = content.as_bytes();
    let mut start = 0;

    while let Some(pos) = content[start..].find(token) {
        let abs = start + pos;
        let end = abs + token.len();

        let before_ok = abs == 0
            || {
                let b = bytes[abs - 1];
                !(b.is_ascii_alphanumeric() || b == b'_')
            };

        let after_ok = end >= bytes.len()
            || {
                let b = bytes[end];
                !(b.is_ascii_alphanumeric() || b == b'_')
            };

        if before_ok && after_ok {
            return true;
        }

        start = abs + 1;
    }

    false
}

/// Checks whether an identifier appears in code as a real identifier token.
/// Skips obvious comment lines to reduce false positives.
pub fn contains_identifier(content: &str, identifier: &str) -> bool {
    if identifier.is_empty() {
        return false;
    }

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("//")
            || trimmed.starts_with("/*")
            || trimmed.starts_with('*')
            || trimmed.starts_with('#')
        {
            continue;
        }

        if trimmed
            .split(|c: char| !(c.is_alphanumeric() || c == '_'))
            .any(|word| word == identifier)
        {
            return true;
        }
    }

    false
}

/// Detects whether a trait/interface/class is implemented by a concrete type.
///
/// Supports:
///   Rust:     impl Trait for Type
///             impl<T> Trait for Type
///             impl<'a> Trait<'a> for Type
///   Python:   class Foo(Trait):
///             class Foo(Base, Trait):
///   TS/JS:    class Foo implements Trait
///             export class Foo implements Trait, Other
pub fn has_trait_implementation(content: &str, trait_name: &str) -> bool {
    if trait_name.is_empty() {
        return false;
    }

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("//")
            || trimmed.starts_with("/*")
            || trimmed.starts_with('*')
            || trimmed.starts_with('#')
        {
            continue;
        }

        // Rust: impl Trait for Type
        if trimmed.starts_with("impl") && trimmed.contains(" for ") {
            if let Some(after_impl) = trimmed.strip_prefix("impl") {
                if let Some(before_for) = after_impl.split(" for ").next() {
                    let mut trait_part = before_for.trim();

                    // Remove leading generic parameters:
                    // impl<T> Trait for Type
                    if trait_part.starts_with('<') {
                        if let Some(gt) = trait_part.find('>') {
                            trait_part = trait_part[gt + 1..].trim();
                        }
                    }

                    // Remove trailing generic arguments:
                    // Trait<T>
                    let trait_part = trait_part.split('<').next().unwrap_or("").trim();

                    let last_segment = trait_part
                        .rsplit("::")
                        .next()
                        .unwrap_or(trait_part)
                        .trim();

                    if last_segment == trait_name || trait_part == trait_name {
                        return true;
                    }
                }
            }
        }

        // Python/TS/JS class inheritance or implementation.
        if let Some(class_pos) = trimmed.find("class ") {
            let class_line = &trimmed[class_pos..];

            // Python: class Foo(Trait), class Foo(Base, Trait)
            if let Some(paren_start) = class_line.find('(') {
                if let Some(paren_end) = class_line[paren_start..].find(')') {
                    let bases = &class_line[paren_start + 1..paren_start + paren_end];

                    if bases.split(',').any(|base| {
                        let base = base.trim().split('<').next().unwrap_or("").trim();
                        let base = base.rsplit('.').next().unwrap_or(base).trim();
                        let base = base.rsplit("::").next().unwrap_or(base).trim();
                        base == trait_name
                    }) {
                        return true;
                    }
                }
            }

            // TypeScript: class Foo implements Trait
            if let Some(impl_pos) = class_line.find(" implements ") {
                let after = &class_line[impl_pos + " implements ".len()..];
                let before_brace = after.split('{').next().unwrap_or(after);

                if before_brace.split(',').any(|base| {
                    let base = base.trim().split('<').next().unwrap_or("").trim();
                    let base = base.rsplit("::").next().unwrap_or(base).trim();
                    base == trait_name
                }) {
                    return true;
                }
            }
        }
    }

    false
}

/// Generate likely import tokens for a source file.
///
/// Example:
///   crates/shared/src/orphan-detector/utility_orphan_io.rs
///
/// Produces tokens such as:
///   utility_orphan_io
///   orphan_detector/utility_orphan_io
///   orphan_detector::utility_orphan_io
///   shared::orphan_detector::utility_orphan_io
///   shared.orphan_detector.utility_orphan_io
pub fn import_tokens(path: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let path_ref = Path::new(path);

    let comps: Vec<String> = path_ref
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .map(|s| s.to_string())
        .collect();

    if comps.is_empty() {
        return tokens;
    }

    let (crate_name, module_comps) = match comps.iter().position(|c| c == "src") {
        Some(pos) => {
            let crate_name = if pos > 0 {
                Some(comps[pos - 1].clone())
            } else {
                None
            };
            (crate_name, comps[pos + 1..].to_vec())
        }
        None => (None, comps.clone()),
    };

    if module_comps.is_empty() {
        return tokens;
    }

    let last = module_comps.last().unwrap();
    let stem = Path::new(last)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();

    if stem.is_empty() {
        return tokens;
    }

    let stem = normalize_module_component(stem);
    let dirs = &module_comps[..module_comps.len() - 1];

    tokens.push(stem.clone());

    for n in 0..=dirs.len() {
        let selected = &dirs[dirs.len() - n..];
        let mut parts: Vec<String> = selected
            .iter()
            .map(|s| normalize_module_component(s))
            .collect();

        parts.push(stem.clone());

        let slash = parts.join("/");
        tokens.push(slash.clone());
        tokens.push(slash.replace('/', "::"));
        tokens.push(slash.replace('/', "."));

        if let Some(crate_name) = &crate_name {
            let mut with_crate: Vec<String> = vec![normalize_module_component(crate_name)];
            with_crate.extend(parts.clone());

            let with_crate_slash = with_crate.join("/");
            tokens.push(with_crate_slash.clone());
            tokens.push(with_crate_slash.replace('/', "::"));
            tokens.push(with_crate_slash.replace('/', "."));
        }
    }

    tokens.sort();
    tokens.dedup();
    tokens
}
```

---

# 3. Contract Fix: Utility Orphan Protocol Needs Inbound Links

The utility analyzer should use the already-built import graph instead of repeatedly scanning all files.

## File: `crates/shared/src/orphan-detector/contract_orphan_protocol.rs`

Replace `IUtilityOrphanProtocol` with:

```rust
pub trait IUtilityOrphanProtocol: Send + Sync {
    fn is_utility_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        all_files: &[String],
        inbound_links: &InboundLinkMap,
    ) -> OrphanIndicatorResult;
}
```

No other contract changes are required.

---

# 4. Orchestrator Fixes

## File: `crates/orphan-detector/src/agent_orphan_orchestrator.rs`

Apply these changes:

1. Store `ArchitectureConfig` so `config()` is real.
2. Respect ignored paths.
3. Pass inbound links to utility analyzer.
4. Use configured layer definitions when present.

Add imports:

```rust
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::common::taxonomy_paths_vo::FilePathList;
```

Replace the struct and constructors:

```rust
pub struct ArchOrphanAnalyzer {
    resolver: Arc<dyn IOrphanGraphResolverProtocol>,
    taxonomy_analyzer: Arc<dyn ITaxonomyOrphanProtocol>,
    contract_analyzer: Arc<dyn IContractOrphanProtocol>,
    capabilities_analyzer: Arc<dyn ICapabilitiesOrphanProtocol>,
    utility_analyzer: Arc<dyn IUtilityOrphanProtocol>,
    agent_analyzer: Arc<dyn IAgentOrphanProtocol>,
    surfaces_analyzer: Arc<dyn ISurfacesOrphanProtocol>,
    config: ArchitectureConfig,
}

impl ArchOrphanAnalyzer {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        resolver: Arc<dyn IOrphanGraphResolverProtocol>,
        taxonomy_analyzer: Arc<dyn ITaxonomyOrphanProtocol>,
        contract_analyzer: Arc<dyn IContractOrphanProtocol>,
        capabilities_analyzer: Arc<dyn ICapabilitiesOrphanProtocol>,
        utility_analyzer: Arc<dyn IUtilityOrphanProtocol>,
        agent_analyzer: Arc<dyn IAgentOrphanProtocol>,
        surfaces_analyzer: Arc<dyn ISurfacesOrphanProtocol>,
    ) -> Self {
        Self::new_with_config(
            resolver,
            taxonomy_analyzer,
            contract_analyzer,
            capabilities_analyzer,
            utility_analyzer,
            agent_analyzer,
            surfaces_analyzer,
            ArchitectureConfig::default(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_with_config(
        resolver: Arc<dyn IOrphanGraphResolverProtocol>,
        taxonomy_analyzer: Arc<dyn ITaxonomyOrphanProtocol>,
        contract_analyzer: Arc<dyn IContractOrphanProtocol>,
        capabilities_analyzer: Arc<dyn ICapabilitiesOrphanProtocol>,
        utility_analyzer: Arc<dyn IUtilityOrphanProtocol>,
        agent_analyzer: Arc<dyn IAgentOrphanProtocol>,
        surfaces_analyzer: Arc<dyn ISurfacesOrphanProtocol>,
        config: ArchitectureConfig,
    ) -> Self {
        Self {
            resolver,
            taxonomy_analyzer,
            contract_analyzer,
            capabilities_analyzer,
            utility_analyzer,
            agent_analyzer,
            surfaces_analyzer,
            config,
        }
    }

    fn is_ignored(&self, file: &str) -> bool {
        let file = file.replace('\\', "/");

        self.config.ignored_paths.values.iter().any(|pattern| {
            let raw = pattern.value().replace('\\', "/");
            if raw.is_empty() {
                return false;
            }

            if file == raw || file.ends_with(&raw) {
                return true;
            }

            let normalized = raw.trim_start_matches('/');
            if normalized.is_empty() {
                return false;
            }

            file.starts_with(&format!("{normalized}/"))
                || file.contains(&format!("/{normalized}/"))
                || file.contains(&format!("/{normalized}"))
        })
    }
}
```

Replace `check_orphans` inside `impl IOrphanAggregate for ArchOrphanAnalyzer`:

```rust
    fn check_orphans(
        &self,
        layer_detector: &dyn ILayerDetectionAggregate,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult> {
        // Global gate: skip all orphan checks if architecture checker is disabled.
        if !self.config.enabled.value || !layer_detector.config().enabled.value {
            return Vec::new();
        }

        // Apply configured ignored paths.
        let filtered_files: Vec<String> = files
            .iter()
            .filter(|f| !self.is_ignored(f))
            .cloned()
            .collect();

        let files = filtered_files.as_slice();

        let mut results: Vec<LintResult> = Vec::new();

        let file_vo = shared::orphan_detector::OrphanFileListVO::new(files.to_vec());
        let context: GraphAnalysisContext = self
            .resolver
            .build_graph_context(std::slice::from_ref(&file_vo), root_dir);

        let configured = layer_detector.get_orphan_entry_points();
        let configured_vo = shared::orphan_detector::OrphanEntryPatternListVO::new(configured);

        let entry_points = self
            .resolver
            .identify_entry_points(std::slice::from_ref(&file_vo), &[configured_vo]);

        let alive_files_set: Vec<String> =
            self._trace_reachability(&entry_points.values, &context.import_graph);

        for f in files {
            let file_fp = match FilePath::new(f.clone()) {
                Ok(fp) => fp,
                Err(_) => continue,
            };

            let layer_str = match layer_detector.detect_layer(f, root_dir) {
                Some(l) => l,
                None => continue,
            };

            let definition = match layer_detector.get_layer_def(&layer_str) {
                Some(d) => d,
                None => continue,
            };

            let basename = file_fp.basename();
            if definition.exceptions.values.contains(&basename) {
                continue;
            }

            if !definition.orphan.check_orphan.value {
                continue;
            }

            let layer_vo = LayerNameVO::new(&layer_str);
            let res =
                self._evaluate_layer(f, &context, &alive_files_set, &layer_vo, files, root_dir);

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

Replace the utility branch inside `_evaluate_layer`:

```rust
        if layer_str.contains(LAYER_UTILITY) {
            return self.utility_analyzer.is_utility_orphan(
                &fp,
                &root,
                all_files,
                &context.inbound_links,
            );
        }
```

Replace `ILayerDetectionAggregate` implementation:

```rust
impl ILayerDetectionAggregate for ArchOrphanAnalyzer {
    fn config(&self) -> &shared::config_system::taxonomy_config_vo::ArchitectureConfig {
        &self.config
    }

    fn detect_layer(&self, file_path: &str, _root_dir: &str) -> Option<String> {
        let path = std::path::Path::new(file_path);
        let filename = path.file_name()?.to_str()?;
        let stem = std::path::Path::new(filename).file_stem()?.to_str()?;

        const PREFIX_MAP: &[(&str, &str)] = &[
            ("taxonomy_", "taxonomy"),
            ("contract_", "contract"),
            ("utility_", "utility"),
            ("capabilities_", "capabilities"),
            ("agent_", "agent"),
            ("surface_", "surfaces"),
            ("root_", "root"),
        ];

        for (prefix, layer) in PREFIX_MAP {
            if stem.starts_with(prefix) {
                return Some(layer.to_string());
            }
        }

        None
    }

    fn get_layer_def(&self, layer: &str) -> Option<LayerDefinition> {
        let key = LayerNameVO::new(layer);
        if let Some(def) = self.config.layers.get(&key) {
            return Some(def.clone());
        }

        let mut def = LayerDefinition::default();
        def.orphan.check_orphan = shared::common::taxonomy_common_vo::BooleanVO::new(true);
        def.exceptions.values = vec![
            "mod.rs".to_string(),
            "__init__.py".to_string(),
            "index.ts".to_string(),
            "index.js".to_string(),
            "py.typed".to_string(),
        ];

        Some(def)
    }

    fn get_orphan_entry_points(&self) -> Vec<String> {
        vec![
            "_container.rs".to_string(),
            "_container.py".to_string(),
            "_container.ts".to_string(),
            "_container.js".to_string(),
            "_entry.rs".to_string(),
            "_entry.py".to_string(),
            "_entry.ts".to_string(),
            "_entry.js".to_string(),
            "main.rs".to_string(),
            "lib.rs".to_string(),
            "main.py".to_string(),
            "main.ts".to_string(),
            "main.js".to_string(),
            "index.ts".to_string(),
            "index.js".to_string(),
        ]
    }
}
```

---

# 5. Container Fix: Wire Ignored Paths

## File: `crates/orphan-detector/src/root_orphan_detector_container.rs`

Replace with:

```rust
// PURPOSE: OrphanContainer — wiring for orphan-detector feature (root layer, wiring only)

use crate::agent_orphan_orchestrator::ArchOrphanAnalyzer;
use crate::capabilities_orphan_graph_resolver::OrphanGraphResolver;
use shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::orphan_detector::contract_orphan_graph_resolver_protocol::IOrphanGraphResolverProtocol;
use std::sync::Arc;

pub struct OrphanContainer {
    analyzer: Arc<dyn IOrphanAggregate>,
    layer_detector: Arc<dyn ILayerDetectionAggregate>,
}

impl OrphanContainer {
    pub fn new() -> Self {
        Self::new_with_ignored(Vec::new())
    }

    pub fn new_with_ignored(ignored_paths: Vec<String>) -> Self {
        let mut config = ArchitectureConfig::default();

        config.ignored_paths = FilePathList::new(
            ignored_paths
                .into_iter()
                .filter_map(|p| FilePath::new(p).ok())
                .collect(),
        );

        let resolver: Arc<dyn IOrphanGraphResolverProtocol> = Arc::new(OrphanGraphResolver::new());

        let arch = Arc::new(ArchOrphanAnalyzer::new_with_config(
            resolver,
            Arc::new(crate::capabilities_orphan_taxonomy_analyzer::TaxonomyOrphanAnalyzer::new()),
            Arc::new(crate::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer::new()),
            Arc::new(
                crate::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer::new(),
            ),
            Arc::new(crate::capabilities_orphan_utility_analyzer::UtilityOrphanAnalyzer::new()),
            Arc::new(crate::capabilities_orphan_agent_analyzer::AgentOrphanAnalyzer::new()),
            Arc::new(crate::capabilities_orphan_surfaces_analyzer::SurfacesOrphanAnalyzer::new()),
            config,
        ));

        let layer: Arc<dyn ILayerDetectionAggregate> = arch.clone();

        Self {
            analyzer: arch.clone() as Arc<dyn IOrphanAggregate>,
            layer_detector: layer,
        }
    }

    pub fn analyzer(&self) -> Arc<dyn IOrphanAggregate> {
        self.analyzer.clone()
    }

    pub fn layer_detector(&self) -> Arc<dyn ILayerDetectionAggregate> {
        self.layer_detector.clone()
    }
}

impl Default for OrphanContainer {
    fn default() -> Self {
        Self::new()
    }
}
```

---

# 6. Graph Resolver Fixes

This is the most important functional/performance fix.

## File: `crates/orphan-detector/src/capabilities_orphan_graph_resolver.rs`

Add import:

```rust
use std::collections::HashSet;
```

Replace `identify_entry_points` inside `impl IOrphanGraphResolverProtocol for OrphanGraphResolver`:

```rust
    fn identify_entry_points(
        &self,
        files: &[OrphanFileListVO],
        configured: &[OrphanEntryPatternListVO],
    ) -> OrphanFileListVO {
        let file_strs: Vec<String> = files
            .iter()
            .flat_map(|v| v.values.iter().cloned())
            .collect();

        let configured_strs: Vec<String> = configured
            .iter()
            .flat_map(|p| p.values.iter().cloned())
            .collect();

        let matched: Vec<String> = if configured_strs.is_empty() {
            file_strs
                .iter()
                .filter(|f| {
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
                        || basename == "index.ts"
                        || basename == "index.js"
                })
                .cloned()
                .collect()
        } else {
            file_strs
                .iter()
                .filter(|f| {
                    let basename = f.rsplit('/').next().unwrap_or(f);

                    configured_strs.iter().any(|pattern| {
                        basename == pattern
                            || basename.ends_with(pattern)
                            || shared::orphan_detector::utility_orphan_filename::file_stem(basename)
                                .contains(pattern)
                    })
                })
                .cloned()
                .collect()
        };

        let normalized = matched
            .into_iter()
            .map(|f| OrphanGraphResolver::normalize_file_path(&f))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        OrphanFileListVO::new(normalized)
    }
```

Replace the private helper block inside `impl OrphanGraphResolver` with:

```rust
impl OrphanGraphResolver {
    pub fn new() -> Self {
        Self {}
    }

    fn normalize_file_path(value: &str) -> String {
        shared::common::taxonomy_path_vo::FilePath::new(value.to_string())
            .map(|fp| fp.value().to_string())
            .unwrap_or_else(|_| value.to_string())
    }

    fn add_edge(
        import_graph: &mut HashMap<String, Vec<String>>,
        inbound_links: &mut HashMap<String, Vec<String>>,
        from: &str,
        to: &str,
    ) {
        let to = Self::normalize_file_path(to);
        if to == from {
            return;
        }

        let neighbors = import_graph.entry(from.to_string()).or_default();
        if !neighbors.contains(&to) {
            neighbors.push(to.clone());
        }

        let inbound = inbound_links.entry(to).or_default();
        if !inbound.contains(&from.to_string()) {
            inbound.push(from.to_string());
        }
    }

    fn build_crate_module_index(
        crate_src_dirs: &HashMap<String, std::path::PathBuf>,
    ) -> HashMap<String, HashMap<String, String>> {
        let mut unique_dirs: HashMap<std::path::PathBuf, Vec<String>> = HashMap::new();

        for (crate_name, src_dir) in crate_src_dirs {
            unique_dirs
                .entry(src_dir.clone())
                .or_default()
                .push(crate_name.clone());
        }

        let mut index: HashMap<String, HashMap<String, String>> = HashMap::new();

        for (src_dir, crate_names) in unique_dirs {
            let mut map: HashMap<String, String> = HashMap::new();

            let files = utility_orphan_io::scan_directory_recursive(&src_dir);

            for file in files {
                let path = std::path::PathBuf::from(&file);
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or_default();

                if !matches!(ext, "rs" | "py" | "ts" | "js" | "tsx" | "jsx") {
                    continue;
                }

                let rel = match path.strip_prefix(&src_dir) {
                    Ok(r) => r,
                    Err(_) => continue,
                };

                let rel_no_ext = rel.with_extension("");

                let mut comps: Vec<String> = rel_no_ext
                    .components()
                    .filter_map(|c| c.as_os_str().to_str())
                    .map(shared::orphan_detector::utility_orphan::normalize_module_component)
                    .collect();

                if comps.is_empty() {
                    continue;
                }

                if let Some(last) = comps.last() {
                    if last == "mod" || last == "__init__" || last == "index" {
                        comps.pop();
                    }
                }

                if comps.is_empty() {
                    continue;
                }

                let full = comps.join("/");
                map.insert(full.clone(), file.clone());

                if let Some(last) = comps.last() {
                    map.entry(last.clone()).or_insert_with(|| file.clone());
                }
            }

            for crate_name in crate_names {
                index.insert(crate_name, map.clone());
            }
        }

        index
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
            let normalized =
                shared::orphan_detector::utility_orphan::normalize_module_path(&candidate);

            if let Some(path) = map.get(&normalized) {
                let path = Self::normalize_file_path(path);
                if path != current_file {
                    return Some(path);
                }
            }
        }

        None
    }

    /// Cached regexes (Perf 1): compiled once via OnceLock.
    fn pub_mod_path_re() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| {
            Regex::new(r#"#\[path\s*=\s*"([^"]+)"\]\s*(?:pub\s+)?mod\s+([a-zA-Z_]+)"#).ok()
        })
        .as_ref()
    }

    fn plain_mod_re() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"(?:pub\s+)?mod\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*;").ok())
            .as_ref()
    }

    fn import_re() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"(?:use|import|from)\s+([a-zA-Z_][a-zA-Z0-9_\.:]*)").ok())
            .as_ref()
    }

    fn inh_re() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"class\s+\w+\(([^)]+)\)").ok())
            .as_ref()
    }

    fn build_graph_context_inner(&self, files: &[String], root_dir: &str) -> GraphAnalysisContext {
        let files: Vec<String> = files
            .iter()
            .map(|f| Self::normalize_file_path(f))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        let mut import_graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut inbound_links: HashMap<String, Vec<String>> = HashMap::new();
        let mut inheritance_map: HashMap<String, Vec<String>> = HashMap::new();
        let file_definitions: HashMap<String, Vec<String>> = HashMap::new();

        let mut module_to_file: HashMap<String, String> = HashMap::new();

        for f in &files {
            let stem = file_stem(f);
            module_to_file.insert(stem.clone(), f.clone());

            if let Some(parent) = f.rsplit('/').nth(1) {
                let module_path = format!("{parent}/{stem}");
                module_to_file.insert(module_path.clone(), f.clone());

                let normalized_path =
                    shared::orphan_detector::utility_orphan::normalize_module_path(&module_path);

                if normalized_path != module_path {
                    module_to_file.insert(normalized_path, f.clone());
                }
            }

            if stem == "mod" {
                if let Some(parent_dir) = f.rsplit('/').nth(1) {
                    module_to_file.insert(parent_dir.to_string(), f.clone());

                    let normalized =
                        shared::orphan_detector::utility_orphan::normalize_module_component(
                            parent_dir,
                        );

                    if normalized != parent_dir {
                        module_to_file.insert(normalized.clone(), f.clone());
                    }

                    if let Some(grandparent) = f.rsplit('/').nth(2) {
                        let composite = format!("{grandparent}/{parent_dir}");
                        module_to_file.insert(composite.clone(), f.clone());

                        let normalized_composite =
                            shared::orphan_detector::utility_orphan::normalize_module_path(
                                &composite,
                            );

                        if normalized_composite != composite {
                            module_to_file.insert(normalized_composite, f.clone());
                        }
                    }
                }
            }
        }

        let mut workspace_modules: std::collections::HashSet<String> =
            std::collections::HashSet::new();

        let mut crate_src_dirs: HashMap<String, std::path::PathBuf> = HashMap::new();
        let root_path = std::path::Path::new(root_dir);

        for ws_dir in &["crates", "packages", "modules"] {
            let ws_path = root_path.join(ws_dir);

            if shared::orphan_detector::utility_orphan_io::is_dir(&ws_path) {
                let entries = shared::orphan_detector::utility_orphan_io::scan_directory(&ws_path);

                for (name, path_str, is_dir_entry) in entries {
                    if !is_dir_entry {
                        continue;
                    }

                    workspace_modules.insert(name.clone());
                    workspace_modules.insert(name.replace('-', "_"));

                    let src_dir = std::path::PathBuf::from(&path_str).join("src");

                    if shared::orphan_detector::utility_orphan_io::is_dir(&src_dir) {
                        crate_src_dirs.insert(name.clone(), src_dir.clone());
                        crate_src_dirs.insert(name.replace('-', "_"), src_dir);
                    }
                }
            }
        }

        let crate_module_index = Self::build_crate_module_index(&crate_src_dirs);

        for f in &files {
            import_graph.entry(f.clone()).or_default();

            let content = utility_orphan_io::read_file_safe(f);

            if content.is_empty()
                && !shared::orphan_detector::utility_orphan_io::is_file(&std::path::PathBuf::from(f))
            {
                continue;
            }

            // Pass 1: #[path = "..."] pub mod
            if let Some(re) = Self::pub_mod_path_re() {
                for cap in re.captures_iter(&content) {
                    let mod_path = cap[1].to_string();

                    let base_dir = match std::path::Path::new(f).parent() {
                        Some(p) => p.to_string_lossy().to_string(),
                        None => String::from("."),
                    };

                    let resolved = if mod_path.starts_with('/') {
                        mod_path.clone()
                    } else {
                        format!("{base_dir}/{mod_path}")
                    };

                    if shared::orphan_detector::utility_orphan_io::is_file(
                        &std::path::PathBuf::from(&resolved),
                    ) {
                        Self::add_edge(&mut import_graph, &mut inbound_links, f, &resolved);
                    }
                }
            }

            // Pass 2: plain mod
            if let Some(re) = Self::plain_mod_re() {
                for cap in re.captures_iter(&content) {
                    let mod_name = cap[1].to_string();

                    let parent = match std::path::Path::new(f).parent() {
                        Some(p) => p,
                        None => continue,
                    };

                    let candidates = [
                        parent.join(format!("{mod_name}.rs")),
                        parent.join(&mod_name).join("mod.rs"),
                        parent.join(format!("{mod_name}.py")),
                        parent.join(&mod_name).join("__init__.py"),
                    ];

                    for candidate in &candidates {
                        if shared::orphan_detector::utility_orphan_io::is_file(candidate) {
                            if let Some(path_str) = candidate.to_str() {
                                Self::add_edge(
                                    &mut import_graph,
                                    &mut inbound_links,
                                    f,
                                    path_str,
                                );
                                break;
                            }
                        }
                    }
                }
            }

            // Pass 3: use/import/from
            let Some(import_re) = Self::import_re() else {
                continue;
            };

            for cap in import_re.captures_iter(&content) {
                let full_import = cap[1].to_string();

                let full_import =
                    if let Some(stripped) = full_import.strip_prefix("lint_arwaky::") {
                        format!("crate::{stripped}")
                    } else {
                        full_import
                    };

                // crate:: resolution
                if let Some(path_part) = full_import.strip_prefix("crate::") {
                    let segments: Vec<&str> = path_part.split("::").filter(|s| !s.is_empty()).collect();

                    if segments.len() >= 2 {
                        let mut resolved = false;

                        for i in (1..segments.len()).rev() {
                            let composite = segments[..i].join("/");
                            let normalized =
                                shared::orphan_detector::utility_orphan::normalize_module_path(
                                    &composite,
                                );

                            if let Some(file_path) = module_to_file.get(&normalized) {
                                Self::add_edge(
                                    &mut import_graph,
                                    &mut inbound_links,
                                    f,
                                    file_path,
                                );
                                resolved = true;
                                break;
                            }
                        }

                        if resolved {
                            continue;
                        }

                        for seg in segments[..segments.len() - 1].iter().rev() {
                            let normalized =
                                shared::orphan_detector::utility_orphan::normalize_module_component(
                                    seg,
                                );

                            if let Some(file_path) = module_to_file.get(&normalized) {
                                Self::add_edge(
                                    &mut import_graph,
                                    &mut inbound_links,
                                    f,
                                    file_path,
                                );
                                resolved = true;
                                break;
                            }
                        }

                        if resolved {
                            continue;
                        }
                    }

                    if let Some(seg) = segments.first() {
                        let normalized =
                            shared::orphan_detector::utility_orphan::normalize_module_component(seg);

                        if let Some(file_path) = module_to_file.get(&normalized) {
                            Self::add_edge(&mut import_graph, &mut inbound_links, f, file_path);
                        }
                    }

                    continue;
                }

                // super:: resolution
                if let Some(path_part) = full_import.strip_prefix("super::") {
                    let segments: Vec<&str> = path_part.split("::").filter(|s| !s.is_empty()).collect();

                    if segments.len() >= 2 {
                        let mut found = false;

                        for i in (1..segments.len()).rev() {
                            let composite = segments[..i].join("/");
                            let normalized =
                                shared::orphan_detector::utility_orphan::normalize_module_path(
                                    &composite,
                                );

                            if let Some(file_path) = module_to_file.get(&normalized) {
                                Self::add_edge(
                                    &mut import_graph,
                                    &mut inbound_links,
                                    f,
                                    file_path,
                                );
                                found = true;
                                break;
                            }
                        }

                        if found {
                            continue;
                        }

                        for seg in segments[..segments.len() - 1].iter().rev() {
                            let normalized =
                                shared::orphan_detector::utility_orphan::normalize_module_component(
                                    seg,
                                );

                            if let Some(resolved) = module_to_file.get(&normalized) {
                                Self::add_edge(
                                    &mut import_graph,
                                    &mut inbound_links,
                                    f,
                                    resolved,
                                );
                                break;
                            }
                        }
                    } else if let Some(seg) = segments.first() {
                        let normalized =
                            shared::orphan_detector::utility_orphan::normalize_module_component(seg);

                        if let Some(resolved) = module_to_file.get(&normalized) {
                            Self::add_edge(&mut import_graph, &mut inbound_links, f, resolved);
                        }
                    }

                    continue;
                }

                // Workspace-crate Rust imports:
                //   use shared::orphan_detector::utility_orphan_io::read_file_safe;
                if let Some(colon_idx) = full_import.find("::") {
                    let crate_name = &full_import[..colon_idx];
                    let rest = &full_import[colon_idx + 2..];

                    let segments: Vec<&str> = rest.split("::").filter(|s| !s.is_empty()).collect();

                    if workspace_modules.contains(crate_name) {
                        if let Some(resolved) = Self::resolve_workspace_module(
                            &crate_module_index,
                            crate_name,
                            &segments,
                            f,
                        ) {
                            Self::add_edge(&mut import_graph, &mut inbound_links, f, &resolved);
                            continue;
                        }
                    }

                    if !segments.is_empty() {
                        continue;
                    }
                }

                // Python/JS dotted workspace imports:
                //   from shared.orphan_detector.utility_orphan_io import read_file_safe
                if full_import.contains('.') {
                    let segments: Vec<&str> =
                        full_import.split('.').filter(|s| !s.is_empty()).collect();

                    if let Some(crate_name) = segments.first() {
                        if workspace_modules.contains(*crate_name) && segments.len() > 1 {
                            if let Some(resolved) = Self::resolve_workspace_module(
                                &crate_module_index,
                                crate_name,
                                &segments[1..],
                                f,
                            ) {
                                Self::add_edge(
                                    &mut import_graph,
                                    &mut inbound_links,
                                    f,
                                    &resolved,
                                );
                                continue;
                            }
                        }
                    }
                }

                // Fallback local module resolution.
                let mut dep = full_import.clone();

                if let Some(dot) = dep.find('.') {
                    dep = dep[..dot].to_string();
                }

                if let Some(colon) = dep.find("::") {
                    dep = dep[..colon].to_string();
                }

                if matches!(dep.as_str(), "crate" | "self" | "super") {
                    continue;
                }

                let is_known_local =
                    module_to_file.contains_key(&dep) || workspace_modules.contains(&dep);

                if !is_known_local {
                    continue;
                }

                if let Some(resolved) = module_to_file.get(&dep) {
                    Self::add_edge(&mut import_graph, &mut inbound_links, f, resolved);
                } else {
                    Self::add_edge(&mut import_graph, &mut inbound_links, f, &dep);
                }
            }

            // Pass 4: Python class inheritance
            if let Some(re) = Self::inh_re() {
                for cap in re.captures_iter(&content) {
                    for base in cap[1].split(',') {
                        inheritance_map
                            .entry(f.clone())
                            .or_default()
                            .push(base.trim().to_string());
                    }
                }
            }
        }

        GraphAnalysisContext::new(
            ImportGraph::new(import_graph),
            InboundLinkMap::new(inbound_links),
            InheritanceMap::new(inheritance_map),
            FileDefinitionMap::new(file_definitions),
        )
    }
}
```

---

# 7. Utility Analyzer Fix

## File: `crates/orphan-detector/src/capabilities_orphan_utility_analyzer.rs`

Replace the protocol implementation and helpers:

```rust
// PURPOSE: UtilityOrphanAnalyzer — IUtilityOrphanProtocol for detecting orphan utility files

use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IUtilityOrphanProtocol;
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use shared::orphan_detector::utility_file_cache;
use shared::orphan_detector::utility_orphan::{contains_delimited, import_tokens};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct UtilityOrphanAnalyzer {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IUtilityOrphanProtocol for UtilityOrphanAnalyzer {
    fn is_utility_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        all_files: &[String],
        inbound_links: &InboundLinkMap,
    ) -> OrphanIndicatorResult {
        let fp = f.value();

        // Fast path: use the already-built import graph.
        if let Some(importers) = inbound_links.mapping.get(fp) {
            if importers.iter().any(|importer| importer != fp) {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
            }
        }

        let module_name = match std::path::Path::new(fp)
            .file_stem()
            .and_then(|s| s.to_str())
        {
            Some(name) => name.to_string(),
            None => {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
            }
        };

        let tokens = import_tokens(fp);

        // Fallback for imports not captured by graph resolution.
        for other_file in all_files {
            if other_file == fp {
                continue;
            }

            let other_fp = match FilePath::new(other_file.clone()) {
                Ok(fp) => fp,
                Err(_) => continue,
            };

            let content = utility_file_cache::read_cached(&other_fp);
            if content.value().is_empty() {
                continue;
            }

            if self.check_import_patterns(content.value(), &module_name, &tokens) {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
            }
        }

        OrphanIndicatorResult::new(
            true,
            AesOrphanViolation::UtilityOrphan {
                stem: module_name.clone(),
                reason: Some(
                    format!("Utility file '{module_name}' is not imported by any other file.")
                        .into(),
                ),
            }
            .to_string(),
            Severity::HIGH,
        )
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for UtilityOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl UtilityOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    fn check_import_patterns(
        &self,
        content: &str,
        module_name: &str,
        tokens: &[String],
    ) -> bool {
        // Module declaration:
        //   mod utility_orphan_io;
        if content.contains(&format!("mod {module_name};"))
            || content.contains(&format!("mod {module_name} "))
        {
            return true;
        }

        // Path-like imports:
        //   use shared::orphan_detector::utility_orphan_io::read_file_safe;
        //   from shared.orphan_detector.utility_orphan_io import read_file_safe
        for token in tokens {
            if contains_delimited(content, token) {
                return true;
            }
        }

        false
    }
}
```

---

# 8. Contract Analyzer Fix

## File: `crates/orphan-detector/src/capabilities_orphan_contract_analyzer.rs`

Replace struct and helper/implementation sections with:

```rust
// PURPOSE: ContractOrphanAnalyzer — IContractOrphanProtocol for orphan contract detection

use regex::Regex;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use shared::code_analysis::taxonomy_analysis_vo::InheritanceMap;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IContractOrphanProtocol;
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use shared::orphan_detector::utility_file_cache;
use shared::orphan_detector::utility_orphan::{contains_identifier, has_trait_implementation};
use shared::orphan_detector::utility_orphan_filename::{file_basename, file_suffix};
use shared::orphan_detector::utility_workspace::collect_source_files;
use std::sync::{Arc, Mutex, OnceLock};

// ─── Block 1: Struct Definition ───────────────────────────

struct SearchFilesCache {
    root: std::path::PathBuf,
    file_count: usize,
    files: Arc<Vec<String>>,
}

pub struct ContractOrphanAnalyzer {
    search_cache: Mutex<Option<SearchFilesCache>>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IContractOrphanProtocol for ContractOrphanAnalyzer {
    fn is_contract_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        _file_definitions: &FileDefinitionMap,
        _inheritance_map: &InheritanceMap,
        all_files: &[String],
    ) -> OrphanIndicatorResult {
        let fp = f.value();
        let suffix = file_suffix(fp);

        let content = utility_file_cache::read_cached(f);
        if content.value().is_empty() {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        let trait_name = match Self::extract_contract_trait_name(content.value()) {
            Some(t) => t,
            None => return OrphanIndicatorResult::new(false, String::new(), Severity::LOW),
        };

        let search_files = self.search_files(root_dir, all_files);

        // Check 1: contract not implemented by expected layer.
        let target_prefix = match suffix.as_str() {
            "port" | "protocol" => "capabilities",
            "aggregate" => "agent",
            _ => return OrphanIndicatorResult::new(false, String::new(), Severity::LOW),
        };

        let mut has_impl = false;

        for cf in search_files.iter() {
            let cb = file_basename(cf);

            let is_target_layer = cb.starts_with(target_prefix);
            let is_container_impl = cb.starts_with("root_") && cb.ends_with("_container.rs");

            if !is_target_layer && !is_container_impl {
                continue;
            }

            let cf_fp = match FilePath::new(cf.clone()) {
                Ok(fp) => fp,
                Err(_) => continue,
            };

            let c = utility_file_cache::read_cached(&cf_fp);

            if has_trait_implementation(c.value(), &trait_name) {
                has_impl = true;
                break;
            }
        }

        if !has_impl {
            return OrphanIndicatorResult::new(
                true,
                AesOrphanViolation::ContractOrphan {
                    suffix: suffix.clone(),
                    trait_name: trait_name.clone(),
                    target_layer: target_prefix,
                    reason: Some(
                        format!(
                            "Contract {} '{}' not implemented by any {} file.",
                            suffix, trait_name, target_prefix
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::LOW,
            );
        }

        // Check 2: port/protocol not called by any orchestrator, container, capabilities, or surface.
        if suffix == "port" || suffix == "protocol" {
            let mut called_by_impl_or_user = false;

            for cf in search_files.iter() {
                let cb = file_basename(cf);

                let is_orchestrator = cb.starts_with("agent_")
                    && (cb.ends_with("_orchestrator.rs")
                        || cb.ends_with("_orchestrator.py")
                        || cb.ends_with("_orchestrator.ts")
                        || cb.ends_with("_orchestrator.js"));

                let is_container = cb.ends_with("_container.rs")
                    || cb.ends_with("_container.py")
                    || cb.ends_with("_container.ts")
                    || cb.ends_with("_container.js");

                let is_capabilities = cb.starts_with("capabilities_");
                let is_surface = cb.starts_with("surface_");

                if !is_orchestrator && !is_container && !is_capabilities && !is_surface {
                    continue;
                }

                let cf_fp = match FilePath::new(cf.clone()) {
                    Ok(fp) => fp,
                    Err(_) => continue,
                };

                let c = utility_file_cache::read_cached(&cf_fp);

                if contains_identifier(c.value(), &trait_name) {
                    called_by_impl_or_user = true;
                    break;
                }
            }

            if !called_by_impl_or_user {
                return OrphanIndicatorResult::new(
                    true,
                    AesOrphanViolation::ContractOrphan {
                        suffix: suffix.clone(),
                        trait_name: trait_name.clone(),
                        target_layer: target_prefix,
                        reason: Some(
                            format!(
                                "Contract {} '{}' not called by any orchestrator or container.",
                                suffix, trait_name
                            )
                            .into(),
                        ),
                    }
                    .to_string(),
                    Severity::LOW,
                );
            }
        }

        // Check 3: aggregate not called by any surface OR container.
        if suffix == "aggregate" {
            let mut called_by_surface_or_container = false;

            for cf in search_files.iter() {
                let cb = file_basename(cf);

                let is_surface = cb.starts_with("surface_");

                let is_container = cb.ends_with("_container.rs")
                    || cb.ends_with("_container.py")
                    || cb.ends_with("_container.ts")
                    || cb.ends_with("_container.js");

                if !is_surface && !is_container {
                    continue;
                }

                let cf_fp = match FilePath::new(cf.clone()) {
                    Ok(fp) => fp,
                    Err(_) => continue,
                };

                let c = utility_file_cache::read_cached(&cf_fp);

                if contains_identifier(c.value(), &trait_name) {
                    called_by_surface_or_container = true;
                    break;
                }
            }

            if !called_by_surface_or_container {
                return OrphanIndicatorResult::new(
                    true,
                    AesOrphanViolation::ContractOrphan {
                        suffix: suffix.clone(),
                        trait_name: trait_name.clone(),
                        target_layer: target_prefix,
                        reason: Some(
                            format!(
                                "Contract aggregate '{}' not called by any surface or container.",
                                trait_name
                            )
                            .into(),
                        ),
                    }
                    .to_string(),
                    Severity::LOW,
                );
            }
        }

        OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for ContractOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ContractOrphanAnalyzer {
    pub fn new() -> Self {
        Self {
            search_cache: Mutex::new(None),
        }
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

        let mut files: Vec<String> = all_files.to_vec();

        for ws_dir in &["crates", "packages", "modules"] {
            let ws_path = root.join(ws_dir);
            if ws_path.exists() {
                collect_source_files(&ws_path, &mut files);
            }
        }

        files.sort();
        files.dedup();

        let files = Arc::new(files);

        if let Ok(mut guard) = self.search_cache.lock() {
            *guard = Some(SearchFilesCache {
                root,
                file_count: all_files.len(),
                files: files.clone(),
            });
        }

        files
    }

    fn re_contract_rust() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"(?:pub\s+)?trait\s+([A-Za-z0-9_]+)").ok())
            .as_ref()
    }

    fn re_contract_py() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"class\s+([A-Za-z0-9_]+)").ok())
            .as_ref()
    }

    fn re_ts_interface_export() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"export\s+interface\s+([A-Za-z0-9_]+)").ok())
            .as_ref()
    }

    fn re_interface() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"interface\s+([A-Za-z0-9_]+)").ok())
            .as_ref()
    }

    fn extract_contract_trait_name(content: &str) -> Option<String> {
        let code_lines: String = content
            .lines()
            .filter(|l| {
                let t = l.trim();
                !t.starts_with("//") && !t.starts_with("/*") && !t.starts_with('*')
            })
            .collect::<Vec<_>>()
            .join("\n");

        if let Some(re) = Self::re_contract_rust() {
            if let Some(caps) = re.captures(&code_lines) {
                return Some(caps[1].to_string());
            }
        }

        if let Some(re) = Self::re_ts_interface_export() {
            if let Some(caps) = re.captures(&code_lines) {
                return Some(caps[1].to_string());
            }
        }

        if let Some(re) = Self::re_interface() {
            if let Some(caps) = re.captures(&code_lines) {
                return Some(caps[1].to_string());
            }
        }

        Self::re_contract_py()
            .and_then(|re| re.captures(&code_lines))
            .map(|caps| caps[1].to_string())
    }
}
```

---

# 9. Agent Analyzer Fix

## File: `crates/orphan-detector/src/capabilities_orphan_agent_analyzer.rs`

Replace protocol implementation and helpers:

```rust
// PURPOSE: AgentOrphanAnalyzer — IAgentOrphanProtocol for detecting orphan agent files

use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IAgentOrphanProtocol;
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use shared::orphan_detector::utility_file_cache;
use shared::orphan_detector::utility_orphan::contains_identifier;
use regex::Regex;
use std::sync::OnceLock;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct AgentOrphanAnalyzer {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IAgentOrphanProtocol for AgentOrphanAnalyzer {
    fn is_agent_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        all_files: &[String],
    ) -> OrphanIndicatorResult {
        let content = utility_file_cache::read_cached(f);

        if content.value().is_empty() {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        let aggregate_traits = Self::extract_aggregate_traits(content.value());

        if aggregate_traits.is_empty() {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        let mut any_called = false;

        for agg_name in &aggregate_traits {
            for cf in all_files {
                let cb = match cf.split('/').next_back() {
                    Some(b) => b,
                    None => continue,
                };

                if !Self::is_caller_file(cb) {
                    continue;
                }

                let caller_fp = match FilePath::new(cf.clone()) {
                    Ok(fp) => fp,
                    Err(_) => continue,
                };

                let c = utility_file_cache::read_cached(&caller_fp);

                if contains_identifier(c.value(), agg_name) {
                    any_called = true;
                    break;
                }
            }

            if any_called {
                break;
            }
        }

        if !any_called {
            return OrphanIndicatorResult::new(
                true,
                AesOrphanViolation::AgentOrphan {
                    agg_name: aggregate_traits.join(", "),
                    reason: Some(
                        format!(
                            "Agent orphan: aggregates [{}] not called by any surface or entry point.",
                            aggregate_traits.join(", ")
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::HIGH,
            );
        }

        OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for AgentOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

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
            || matches!(
                basename,
                "main.rs"
                    | "lib.rs"
                    | "main.py"
                    | "__main__.py"
                    | "main.ts"
                    | "main.js"
                    | "index.ts"
                    | "index.js"
            )
    }

    fn extract_aggregate_traits(content: &str) -> Vec<String> {
        let mut traits = Vec::new();

        // Rust: impl<T> Trait for Struct
        if let Some(re) = Self::re_impl_generic() {
            for cap in re.captures_iter(content) {
                let name = cap[1].to_string();
                if name.contains("Aggregate") || name.ends_with("Aggregate") {
                    traits.push(name);
                }
            }
        }

        // Rust: Box<dyn Trait> or Arc<dyn Trait>
        if let Some(re) = Self::re_dyn() {
            for cap in re.captures_iter(content) {
                let name = cap[1].to_string();
                if name.contains("Aggregate") || name.ends_with("Aggregate") {
                    traits.push(name);
                }
            }
        }

        // Python: class Struct(ITrait):
        if let Some(re) = Self::re_py_class() {
            for cap in re.captures_iter(content) {
                for part in cap[1].split(',') {
                    let name = part.trim().to_string();
                    if name.contains("Aggregate") || name.ends_with("Aggregate") {
                        traits.push(name);
                    }
                }
            }
        }

        // TS/JS: class Struct implements AggregateA, AggregateB
        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("//")
                || trimmed.starts_with("/*")
                || trimmed.starts_with('*')
                || trimmed.starts_with('#')
            {
                continue;
            }

            if let Some(pos) = trimmed.find(" implements ") {
                let after = &trimmed[pos + " implements ".len()..];
                let before_brace = after.split('{').next().unwrap_or(after);

                for part in before_brace.split(',') {
                    let name = part.trim().split('<').next().unwrap_or("").trim();
                    let name = name.rsplit("::").next().unwrap_or(name).trim();

                    if name.contains("Aggregate") || name.ends_with("Aggregate") {
                        traits.push(name.to_string());
                    }
                }
            }
        }

        traits.sort();
        traits.dedup();
        traits
    }

    fn re_impl_generic() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"impl\s*(?:<[^>]+>)?\s+([A-Za-z0-9_]+)\s+for\s+").ok())
            .as_ref()
    }

    fn re_dyn() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"(?:Box|Arc)<dyn\s+([A-Za-z0-9_]+)>").ok())
            .as_ref()
    }

    fn re_py_class() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"class\s+\w+\(([^)]+)\)").ok())
            .as_ref()
    }
}
```

---

# 10. Surfaces Analyzer Fix

## File: `crates/orphan-detector/src/capabilities_orphan_surfaces_analyzer.rs`

Replace protocol implementation and helpers:

```rust
// PURPOSE: SurfacesOrphanAnalyzer — ISurfacesOrphanProtocol for orphan surface detection

use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::ISurfacesOrphanProtocol;
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use shared::orphan_detector::utility_file_cache;
use shared::orphan_detector::utility_orphan::contains_identifier;
use shared::orphan_detector::utility_orphan_filename::{file_basename, file_stem, file_suffix};
use shared::taxonomy_definition_vo::LayerDefinition;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct SurfacesOrphanAnalyzer {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ISurfacesOrphanProtocol for SurfacesOrphanAnalyzer {
    fn is_surface_orphan(
        &self,
        f: &FilePath,
        alive_files: &ReachabilityResult,
        _definition: Option<&LayerDefinition>,
    ) -> OrphanIndicatorResult {
        // Fast O(1) HashSet lookup instead of cloning alive set into Vec<String>.
        if alive_files.paths.contains(f) {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        let fp_val = f.value();
        let basename = file_basename(fp_val);
        let stem = file_stem(fp_val);

        let content = utility_file_cache::read_cached(f);

        if !content.value().is_empty() {
            let mut identifiers: Vec<String> = Vec::new();

            for line in content.value().lines() {
                let trimmed = line.trim();

                if let Some(name) = trimmed.strip_prefix("pub fn ") {
                    if let Some(name) = name.split('(').next() {
                        identifiers.push(name.trim().to_string());
                    }
                }

                if let Some(name) = trimmed.strip_prefix("pub struct ") {
                    if let Some(name) = name.split('{').next() {
                        identifiers.push(name.trim().to_string());
                    }
                }
            }

            // Derive workspace root from the analyzed file, not CWD.
            let file_path = std::path::Path::new(fp_val);

            if let Ok(workspace_root) =
                shared::orphan_detector::utility_workspace::find_workspace_root(file_path)
            {
                for id in &identifiers {
                    if Self::is_identifier_imported(&workspace_root, id) {
                        return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                    }
                }
            }
        }

        let suffix = Self::get_surface_suffix(&basename);
        let category = Self::surface_category(&suffix);

        OrphanIndicatorResult::new(
            true,
            AesOrphanViolation::SurfaceOrphan {
                category,
                stem,
                reason: Some("Surface is unreachable.".into()),
            }
            .to_string(),
            Severity::HIGH,
        )
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for SurfacesOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SurfacesOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    fn is_identifier_imported(workspace_root: &std::path::Path, id: &str) -> bool {
        for dir_name in &["crates", "packages", "modules"] {
            let dir = workspace_root.join(dir_name);

            if shared::orphan_detector::utility_orphan_io::is_dir(&dir) {
                let files =
                    shared::orphan_detector::utility_orphan_io::scan_directory_recursive(&dir);

                for file_path in &files {
                    let name = match std::path::Path::new(file_path)
                        .file_name()
                        .and_then(|n| n.to_str())
                    {
                        Some(n) => n,
                        None => continue,
                    };

                    let is_importer = name.starts_with("root_")
                        || name.starts_with("cli_")
                        || name.starts_with("mcp_")
                        || name.starts_with("surface_")
                        || name.contains("_entry")
                        || name.contains("_router")
                        || name.contains("_container")
                        || matches!(
                            name,
                            "main.rs"
                                | "lib.rs"
                                | "main.py"
                                | "__main__.py"
                                | "main.ts"
                                | "main.js"
                                | "index.ts"
                                | "index.js"
                        );

                    let is_source = name.ends_with(".rs")
                        || name.ends_with(".py")
                        || name.ends_with(".ts")
                        || name.ends_with(".js");

                    if !is_importer || !is_source {
                        continue;
                    }

                    let fp = match FilePath::new(file_path.clone()) {
                        Ok(fp) => fp,
                        Err(_) => continue,
                    };

                    let content = utility_file_cache::read_cached(&fp);

                    if contains_identifier(content.value(), id) {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn get_surface_suffix(basename: &str) -> String {
        file_suffix(basename)
    }

    fn surface_category(suffix: &str) -> &'static str {
        match suffix {
            "command" | "controller" | "page" => "smart",
            "hook" | "store" | "action" | "screen" | "router" => "utility",
            "component" | "view" | "layout" => "passive",
            _ => "unknown",
        }
    }
}
```

---

# 11. Capabilities Analyzer Fix

## File: `crates/orphan-detector/src/capabilities_orphan_capabilities_analyzer.rs`

Replace with cached container lookup:

```rust
// PURPOSE: CapabilitiesOrphanAnalyzer — ICapabilitiesOrphanProtocol for orphan capability detection

use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::ICapabilitiesOrphanProtocol;
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use shared::orphan_detector::utility_file_cache;
use shared::orphan_detector::utility_orphan::{contains_identifier, extract_struct_names, extract_trait_names};
use shared::orphan_detector::utility_orphan_filename::file_stem;
use shared::orphan_detector::utility_workspace::{collect_source_files, find_workspace_root};
use std::path::PathBuf;
use std::sync::Mutex;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct CapabilitiesOrphanAnalyzer {
    container_cache: Mutex<Option<(std::path::PathBuf, Vec<PathBuf>)>>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ICapabilitiesOrphanProtocol for CapabilitiesOrphanAnalyzer {
    fn is_capabilities_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult {
        if alive_files.paths.contains(f) {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        let fp = f.value();
        let stem = file_stem(fp);

        if !fp.is_empty() {
            let path = FilePath::new(fp).unwrap_or_default();
            let content = utility_file_cache::read_cached(&path);

            let mut identifiers: Vec<String> = Vec::new();

            identifiers.extend(extract_struct_names(content.value()));
            identifiers.extend(extract_trait_names(content.value()));
            identifiers.push(stem.clone());

            let pascal_stem: String = stem
                .split('_')
                .filter(|s| !s.is_empty())
                .map(|s| {
                    let mut c = s.chars();
                    match c.next() {
                        Some(first) => first.to_uppercase().to_string() + c.as_str(),
                        None => String::new(),
                    }
                })
                .collect();

            identifiers.push(pascal_stem);

            if let Some(container_files) = self.container_files(root_dir) {
                for container in container_files {
                    let container_fp = match FilePath::new(container.to_string_lossy().to_string())
                    {
                        Ok(fp) => fp,
                        Err(_) => continue,
                    };

                    let content = utility_file_cache::read_cached(&container_fp);

                    if identifiers
                        .iter()
                        .any(|id| contains_identifier(content.value(), id))
                    {
                        return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                    }
                }
            }
        }

        OrphanIndicatorResult::new(
            true,
            AesOrphanViolation::CapabilitiesOrphan {
                stem,
                reason: Some("Not reachable from any entry point.".into()),
            }
            .to_string(),
            Severity::MEDIUM,
        )
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for CapabilitiesOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilitiesOrphanAnalyzer {
    pub fn new() -> Self {
        Self {
            container_cache: Mutex::new(None),
        }
    }

    fn container_files(&self, root_dir: &FilePath) -> Option<Vec<PathBuf>> {
        let root = std::path::Path::new(root_dir.value()).to_path_buf();

        if let Ok(guard) = self.container_cache.lock() {
            if let Some((cached_root, files)) = guard.as_ref() {
                if *cached_root == root {
                    return Some(files.clone());
                }
            }
        }

        let workspace_root = find_workspace_root(&root).ok()?;
        let mut all_files: Vec<String> = Vec::new();

        for dir_name in &["crates", "packages", "modules"] {
            let dir = workspace_root.join(dir_name);
            if dir.is_dir() {
                collect_source_files(&dir, &mut all_files);
            }
        }

        let container_files: Vec<PathBuf> = all_files
            .into_iter()
            .map(PathBuf::from)
            .filter(|path| {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default();

                name.ends_with("_container.rs")
                    || name.ends_with("_container.py")
                    || name.ends_with("_container.ts")
                    || name.ends_with("_container.js")
                    || name.ends_with("_entry.rs")
                    || name.ends_with("_entry.py")
                    || name.ends_with("_entry.ts")
                    || name.ends_with("_entry.js")
                    || matches!(
                        name,
                        "main.rs"
                            | "lib.rs"
                            | "main.py"
                            | "__main__.py"
                            | "main.ts"
                            | "main.js"
                            | "index.ts"
                            | "index.js"
                    )
            })
            .collect();

        if let Ok(mut guard) = self.container_cache.lock() {
            *guard = Some((root.clone(), container_files.clone()));
        }

        Some(container_files)
    }
}
```

---

# 12. Taxonomy Analyzer Fix

## File: `crates/orphan-detector/src/capabilities_orphan_taxonomy_analyzer.rs`

Replace the protocol implementation body with:

```rust
impl ITaxonomyOrphanProtocol for TaxonomyOrphanAnalyzer {
    fn is_taxonomy_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        _definition: Option<&LayerDefinition>,
        inbound_links: &InboundLinkMap,
    ) -> OrphanIndicatorResult {
        let stem = file_stem(f.value());

        let suffix = match stem.rfind('_') {
            Some(pos) => &stem[pos + 1..],
            None => "",
        };

        let is_utility_or_helper = matches!(suffix, "utility" | "helper");

        let importers = match inbound_links.mapping.get(f.value()) {
            Some(v) => v,
            None => {
                // Fallback for same-crate imports that graph resolution may miss.
                if Self::has_crate_self_import(f.value()) {
                    return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                }

                let category = if is_utility_or_helper {
                    "utility"
                } else {
                    "taxonomy"
                };

                return OrphanIndicatorResult::new(
                    true,
                    AesOrphanViolation::TaxonomyOrphan {
                        stem: stem.clone(),
                        category,
                        reason: Some(
                            format!("Taxonomy '{stem}' is not imported by any file outside taxonomy.")
                                .into(),
                        ),
                    }
                    .to_string(),
                    Severity::LOW,
                );
            }
        };

        let has_outside_taxonomy = importers.iter().any(|importer| {
            importer
                .split('/')
                .next_back()
                .is_some_and(|b| !b.starts_with("taxonomy_"))
        });

        if has_outside_taxonomy {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        let category = if is_utility_or_helper {
            "utility"
        } else {
            "taxonomy"
        };

        OrphanIndicatorResult::new(
            true,
            AesOrphanViolation::TaxonomyOrphan {
                stem: stem.clone(),
                category,
                reason: Some(
                    format!("Taxonomy '{stem}' is not imported by any file outside taxonomy.")
                        .into(),
                ),
            }
            .to_string(),
            Severity::LOW,
        )
    }
}
```

---

# 13. Expected Result After Fixes

After applying the patches:

1. AES501 taxonomy orphan detection uses real inbound links and has a same-crate fallback.
2. AES502 contract orphan detection correctly recognizes Rust, Python, and TypeScript implementations.
3. AES503 capability orphan detection avoids repeated workspace scans by caching container/entry files.
4. AES504 utility orphan detection uses the import graph first and only falls back to bounded token-based scanning.
5. AES505 agent orphan detection recognizes surfaces, containers, and binary entry points.
6. AES506 surface orphan detection no longer depends on CWD and uses O(1) alive-set lookup.
7. Workspace import resolution works for hyphenated crate directories such as:
   - `orphan-detector`
   - `cli-commands`
   - `code-analysis`
   - `config-system`
   - `role-rules`
8. Ignored paths supplied through `OrphanContainer::new_with_ignored` are respected.
9. Repeated directory traversal and uncached file reads are removed from the hottest paths.
