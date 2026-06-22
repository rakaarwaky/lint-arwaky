// PURPOSE: LayerDetectionAnalyzer — layer detection via filename prefix (FRD v1.1)
// This is the central analyzer that implements IAnalyzer. It provides:
//   1. Layer detection per file — exclusively via filename prefix (FRD v1.1).
//      Files without a valid prefix return None and will be reported by AES101 naming enforcement.
//   2. Module layer detection (direct match → prefix match → path match).
//   3. Specialised sub-layer resolution (e.g., "capabilities(command)" from suffix).
//   4. Layer map construction with rule merging (global rules + per-layer rules + specialised rules).
// Used by all AES rule checkers to determine which architectural layer a file belongs to.

use std::collections::HashMap;
use std::path::Path;

use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::config_system::taxonomy_config_vo::ArchitectureRule;
use shared::file_system::contract_system_port::IFileSystemPort;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use shared::import_rules::taxonomy_path_helper;
use shared::source_parsing::contract_parser_port::ISourceParserPort;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::taxonomy_layer_vo::LayerNameVO;
use std::sync::Arc;

/// Central layer detection and rule analysis engine implementing IAnalyzer.
///
/// Capabilities:
///   - `detect_layer(file, root)` → determines which architectural layer a file belongs to,
///     exclusively via filename prefix (FRD v1.1). Returns None for files without a valid
///     prefix — the AES101 naming rule will report those as violations.
///   - `detect_module_layer(module_path)` → determines which layer a module path imports from.
///   - `resolve_specialized_layer(base, file)` → resolves sub-layers (e.g., "capabilities(command)").
///   - `new(config)` → builds a complete layer map by merging global rules, base-layer rules,
///     and specialised sub-layer rules from the rule configuration.
///
/// Constructor workflow:
///   1. Index all config rules by layer scope (both base name and full scoped name).
///   2. For each layer definition: merge global rules + base-layer rules into the definition.
///   3. For each scoped rule (e.g., "agent(container)"): create specialised sub-layer entries
///      by cloning the base definition and overlaying the scoped rule's values.
///   4. Replace config.layers with the enriched layer map for fast lookup.
pub struct LayerDetectionAnalyzer {
    pub config: ArchitectureConfig,
    pub layer_map: LayerMapVO,
    pub fs: Arc<dyn IFileSystemPort>,
    pub parser: Arc<dyn ISourceParserPort>,
}

impl LayerDetectionAnalyzer {
    /// Construct a new LayerDetectionAnalyzer with merged rule configuration.
    ///
    /// Steps:
    ///   1. Build a `rules_by_layer` index: for each rule, map by both its base scope
    ///      (e.g., "agent") and its full scoped name (e.g., "agent(container|registry)").
    ///   2. Iterate all layer definitions from config. For each:
    ///      a. Apply global rules (empty scope key).
    ///      b. Apply base-layer rules (e.g., rules scoped to "agent").
    ///      c. Skip specialised scoped rules (e.g., "agent(container)") at this stage.
    ///   3. For each scoped rule "X(Y|Z)":
    ///      a. Parse the base name X and the set of suffixes {Y, Z}.
    ///      b. Clone the base layer definition.
    ///      c. Overlay the scoped rule's values (forbidden, mandatory, allowed, etc.).
    ///      d. Insert as a new sub-layer entry "X(Y)", "X(Z)".
    ///   4. Store the enriched config and build a LayerMapVO for fast lookups.
    pub fn new(
        mut config: ArchitectureConfig,
        fs: Arc<dyn IFileSystemPort>,
        parser: Arc<dyn ISourceParserPort>,
    ) -> Self {
        // Step 1: Index all rules by layer scope (both base + full scoped)
        let mut rules_by_layer: HashMap<String, Vec<&ArchitectureRule>> = HashMap::new();
        for rule in &config.rules {
            let scope = rule.scope.to_string();
            let base_key = if scope.is_empty() {
                String::new()
            } else {
                scope.split('(').next().unwrap_or(&scope).to_string()
            };
            rules_by_layer.entry(base_key).or_default().push(rule);
            // Also index by full scope (e.g. "agent(container|registry|mixin)")
            if scope.contains('(') {
                rules_by_layer.entry(scope.clone()).or_default().push(rule);
            }
        }

        // Step 2: Merge global + base-layer rules into each layer definition
        let mut new_layers: HashMap<LayerNameVO, LayerDefinition> = HashMap::new();
        for (lname, mut ldef) in config.layers {
            let lstr = lname.to_string();
            let base_name = lstr.split('(').next().unwrap_or(&lstr).to_string();
            // Apply: global rules (key="") + base-layer rules (key=base_name)
            for key in &[String::new(), base_name.clone()] {
                if let Some(rules) = rules_by_layer.get(key.as_str()) {
                    for rule in rules {
                        // Skip specialised scoped rules (e.g. contract(port)) when processing base layers
                        if key.as_str() == base_name && rule.scope.value.contains('(') {
                            continue;
                        }
                        if !rule.exceptions.values.is_empty() {
                            ldef.exceptions = rule.exceptions.clone();
                        }
                        if !rule.mandatory.values.is_empty() {
                            ldef.mandatory = rule.mandatory.clone();
                        }
                        if !rule.forbidden.values.is_empty() {
                            ldef.forbidden = rule.forbidden.clone();
                        }
                        if rule.code_analysis.min_lines.value > 0 {
                            ldef.code_analysis.min_lines = rule.code_analysis.min_lines.clone();
                        }
                        if rule.code_analysis.max_lines.value > 0 {
                            ldef.code_analysis.max_lines = rule.code_analysis.max_lines.clone();
                        }
                        if rule.code_analysis.mandatory_class_definition.value {
                            ldef.code_analysis.mandatory_class_definition =
                                rule.code_analysis.mandatory_class_definition.clone();
                        }
                        if !rule.code_analysis.forbidden_inheritance.values.is_empty() {
                            ldef.code_analysis.forbidden_inheritance =
                                rule.code_analysis.forbidden_inheritance.clone();
                        }
                    }
                }
            }
            new_layers.insert(lname, ldef);
        }

        // Step 3: Create specialised sub-layer entries from scoped rules
        // e.g., "agent(container)" → clone agent def + overlay container-specific rules
        for rule in &config.rules {
            let scope = rule.scope.to_string();
            if !scope.contains('(') {
                continue;
            }
            // Extract suffixes from scope: "agent(container|registry|mixin)"
            if let Some(paren_start) = scope.find('(') {
                let base_name = scope[..paren_start].trim();
                let inner = scope[paren_start + 1..].trim_end_matches(')').trim();
                // Check if the base layer exists — clone def first to avoid borrow conflict
                let base_key_str = base_name.to_string();
                let base_def_opt = {
                    let base_key = LayerNameVO::new(&base_key_str);
                    new_layers.get(&base_key).cloned()
                };
                if let Some(base_def) = base_def_opt {
                    // Step 3a: Parse suffixes (separated by | or ,)
                    let suffixes: Vec<&str> = if inner.contains('|') {
                        inner
                            .split('|')
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty())
                            .collect()
                    } else {
                        inner
                            .split(',')
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty())
                            .collect()
                    };
                    // Step 3b-d: Create one sub-layer per suffix
                    for suffix in suffixes {
                        let specialized_key =
                            LayerNameVO::new(format!("{}({})", base_name, suffix));
                        if new_layers.contains_key(&specialized_key) {
                            continue;
                        }
                        let mut spec_def = base_def.clone();
                        // Step 3c: Overlay scoped rule values onto the cloned definition
                        if let Some(rules) = rules_by_layer.get(&scope) {
                            for r in rules {
                                if !r.exceptions.values.is_empty() {
                                    spec_def.exceptions = r.exceptions.clone();
                                }
                                if !r.forbidden.values.is_empty() {
                                    spec_def.forbidden = r.forbidden.clone();
                                }
                                if !r.mandatory.values.is_empty() {
                                    spec_def.mandatory = r.mandatory.clone();
                                }
                                if !r.allowed.values.is_empty() {
                                    spec_def.allowed = r.allowed.clone();
                                }
                                if !r.code_analysis.forbidden_inheritance.values.is_empty() {
                                    spec_def.code_analysis.forbidden_inheritance =
                                        r.code_analysis.forbidden_inheritance.clone();
                                }
                            }
                        }
                        // Step 3d: Insert the new specialised sub-layer
                        new_layers.insert(specialized_key, spec_def);
                    }
                }
            }
        }

        // Step 4: Store enriched config and build LayerMapVO
        config.layers = new_layers;
        let layer_map = LayerMapVO::new(config.layers.clone());
        Self {
            config,
            layer_map,
            fs,
            parser,
        }
    }

    /// Detect layer from filename — exclusively via filename prefix (FRD v1.1).
    ///
    /// Files MUST carry a layer prefix (e.g., `capabilities_foo.rs` → capabilities layer).
    /// Files without a valid prefix return None, and AES101 naming enforcement will report
    /// them as violations — forcing the developer to add the correct prefix.
    ///
    /// After prefix detection, `resolve_specialized_layer` checks whether the file suffix
    /// corresponds to a specialised sub-layer (e.g., `capabilities_command.rs` with a defined
    /// `capabilities(command)` layer → returns `capabilities(command)` instead of `capabilities`).
    pub fn detect_layer(&self, file_path: &str, _root_dir: &str) -> Option<String> {
        let filename = Path::new(file_path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        // PREFIX-BASED DETECTION (FRD v1.1)
        // All valid files must carry a layer prefix — enforced by AES101/AES102 naming rules.
        if let Some(layer) = taxonomy_path_helper::extract_layer_from_prefix(filename) {
            return Some(self.resolve_specialized_layer(&layer, file_path));
        }

        // No valid prefix found — violates AES101 naming convention.
        // AES101/AES102 will report this separately; we return None so the file
        // is not silently assigned to a wrong layer.
        None
    }

    /// Determine which architectural layer a module path (from an import statement) belongs to.
    ///
    /// Three strategies, in priority order:
    ///
    /// Strategy 1 — Direct segment match:
    ///   Compare each segment of the module path against known layer names.
    ///   E.g., "shared::taxonomy::..." → segment "taxonomy" matches → taxonomy layer.
    ///
    /// Strategy 2 — Prefix-based match (FRD v1.1):
    ///   If no direct match, check if any segment starts with a layer prefix.
    ///   E.g., "taxonomy_definition_vo" starts with "taxonomy_" → taxonomy layer.
    ///
    /// Strategy 3 — Path-based match:
    ///   Convert the module path to a filesystem path and check if it contains any
    ///   layer definition's configured path.
    ///   E.g., module "crates/shared/taxonomy" contains path "shared" → taxonomy layer.
    ///
    /// Each match is refined via `refine_module_layer` to detect specialised sub-layers
    /// (e.g., "capabilities(command)" when the segment after the layer name has a suffix).
    pub fn detect_module_layer(&self, module: &str) -> Option<String> {
        // Split module path into meaningful segments (handles ::, ., /, \ separators)
        let meaningful_parts: Vec<&str> = module
            .split([':', '.', '/', '\\'])
            .filter(|p| !p.is_empty())
            .collect();

        if meaningful_parts.is_empty() {
            return None;
        }

        // Strategy 1: Direct match with layer names (ignoring specialisation suffix)
        for name in self.config.layers.keys() {
            let base_name = name.value.split('(').next().unwrap_or(&name.value);
            if meaningful_parts.contains(&base_name) {
                return Some(self.refine_module_layer(base_name, &meaningful_parts));
            }
        }

        // Strategy 2: Prefix-based match (e.g., "taxonomy_definition_vo" → "taxonomy")
        for part in &meaningful_parts {
            if let Some(layer) = taxonomy_path_helper::extract_layer_from_prefix(part) {
                return Some(self.refine_module_layer(&layer, &meaningful_parts));
            }
        }

        None
    }

    /// Try to resolve a specialised sub-layer from the file's suffix.
    ///
    /// E.g., `capabilities_user_command.rs` with base_layer="capabilities":
    ///   → stem = "capabilities_user_command", last suffix = "command"
    ///   → checks if "capabilities(command)" is a defined specialised layer
    ///   → if yes, returns "capabilities(command)", else returns "capabilities".
    ///
    /// Steps:
    ///   1. Extract the file stem (name without extension).
    ///   2. Find the last underscore segment as the suffix hint.
    ///   3. Construct the specialised layer key: "{base_layer}({suffix})".
    ///   4. Check if this key exists in the built layer map (must have been created from scoped rules).
    ///   5. Return the specialised name if found, otherwise the base layer unchanged.
    fn resolve_specialized_layer(&self, base_layer: &str, file_path: &str) -> String {
        // Step 1: Get file stem
        let basename = Path::new(file_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        // Step 2-5: Check if last underscore suffix matches a specialised sub-layer
        if let Some(underscore_pos) = basename.rfind('_') {
            let suffix = &basename[underscore_pos + 1..];
            if !suffix.is_empty() {
                let specialized = format!("{}({})", base_layer, suffix);
                let key = LayerNameVO::new(specialized.as_str());
                // Step 4: Must have been created in new() from scoped rules
                if self.config.layers.contains_key(&key) {
                    return specialized;
                }
            }
        }

        base_layer.to_string()
    }

    /// Refine a base layer to a specialised sub-layer by inspecting the segment
    /// immediately after the layer name in a dotted module path.
    ///
    /// E.g., parts = ["capabilities", "user_command", "UserCommand"], base = "capabilities"
    ///   → next part after "capabilities" is "user_command"
    ///   → last underscore suffix of "user_command" is "command"
    ///   → checks if "capabilities(command)" exists → returns it if yes.
    ///
    /// Steps:
    ///   1. Find the position of the base layer name in the module parts.
    ///   2. Get the next segment after the base layer name.
    ///   3. Extract the last underscore suffix from that segment.
    ///   4. Construct the specialised key and check if it exists.
    ///   5. Return specialised name or fall back to base name.
    fn refine_module_layer(&self, base_name: &str, parts: &[&str]) -> String {
        // Step 1-2: Find base name position and get next segment
        if let Some(idx) = parts.iter().position(|&p| p == base_name) {
            if idx + 1 < parts.len() {
                let next_part = parts[idx + 1];
                // Step 3: Extract suffix from next segment
                if let Some(underscore_pos) = next_part.rfind('_') {
                    let suffix = &next_part[underscore_pos + 1..];
                    // Step 4-5: Check if specialised sub-layer exists
                    let specialized = format!("{}({})", base_name, suffix);
                    let key = LayerNameVO::new(specialized.as_str());
                    if self.config.layers.contains_key(&key) {
                        return specialized;
                    }
                }
            }
        }
        base_name.to_string()
    }

    /// Look up a `LayerDefinition` by its layer name string.
    /// Falls back to the base layer definition if the specialised key is not found.
    ///
    /// Steps:
    ///   1. Try direct lookup with the full layer name (including parenthesised suffix).
    ///   2. If not found, extract the base name (before the parenthesis) and try again.
    pub fn get_layer_def(&self, layer: &str) -> Option<&LayerDefinition> {
        self.config
            .layers
            .get(&LayerNameVO::new(layer))
            .or_else(|| {
                let base = layer.split('(').next().unwrap_or(layer);
                self.config.layers.get(&LayerNameVO::new(base))
            })
    }
}

impl IAnalyzer for LayerDetectionAnalyzer {
    /// Return the merged architecture configuration.
    fn config(&self) -> &ArchitectureConfig {
        &self.config
    }
    /// Return the layer map (layer name → LayerDefinition).
    fn layer_map(&self) -> &LayerMapVO {
        &self.layer_map
    }
    /// Return the filesystem port for file I/O.
    fn fs(&self) -> &dyn IFileSystemPort {
        &*self.fs
    }
    /// Return the source parser port for code analysis.
    fn parser(&self) -> &dyn ISourceParserPort {
        &*self.parser
    }
    /// Adapter: delegates to internal `detect_layer` and wraps result in LayerNameVO.
    fn detect_layer(&self, f: &FilePath, root_dir: &FilePath) -> Option<LayerNameVO> {
        self.detect_layer(&f.value, &root_dir.value)
            .map(|s| LayerNameVO::new(s.as_str()))
    }
    /// Adapter: delegates to internal `detect_module_layer` and wraps result in LayerNameVO.
    fn detect_module_layer(&self, module_path: &FilePath) -> Option<LayerNameVO> {
        self.detect_module_layer(&module_path.value)
            .map(|s| LayerNameVO::new(s.as_str()))
    }
}
