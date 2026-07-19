// PURPOSE: LayerDetectionAnalyzer — layer detection via filename prefix (FRD v1.1)
// This is the central analyzer that implements ILayerDetectionProtocol. It provides:
//   1. Layer detection per file — exclusively via filename prefix (FRD v1.1).
//      Files without a valid prefix return None and will be reported by AES101 naming enforcement.
//   2. Module layer detection (direct match → prefix match → path match).
//   3. Specialised sub-layer resolution (e.g., "capabilities(command)" from suffix).
//   4. Layer map construction with rule merging (global rules + per-layer rules + specialised rules).
// Used by all AES rule checkers to determine which architectural layer a file belongs to.

use std::collections::HashMap;
use std::path::Path;

use shared::code_analysis::contract_layer_detection_protocol::ILayerDetectionProtocol;
use shared::common::taxonomy_definition_vo::LayerDefinition;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::config_system::taxonomy_config_vo::ArchitectureRule;
use shared::import_rules::taxonomy_import_constant::LAYER_PREFIXES;
use shared::taxonomy_definition_vo::LayerMapVO;
use shared::taxonomy_layer_vo::LayerNameVO;

/// Central layer detection and rule analysis engine implementing ILayerDetectionProtocol.
///
/// Capabilities:
///   - `detect_layer(file, root)` → determines which architectural layer a file belongs to,
///     exclusively via filename prefix (FRD v1.1). Returns None for files without a valid
///     prefix — the AES101 naming rule will report those as violations.
///   - `get_layer_def(layer)` → looks up a LayerDefinition by layer name string.
///   - `get_orphan_entry_points()` → returns known orphan entry point filenames.
///   - `config()` → returns the merged architecture configuration.
///   - Internal: `extract_layer_from_prefix(filename)` → extracts layer from filename prefix.
///   - Internal: `resolve_specialized_layer(base, file)` → resolves sub-layers (e.g., "capabilities(command").
///   - Internal: `new(config)` → builds a complete layer map by merging global rules, base-layer rules,
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
}

impl LayerDetectionAnalyzer {
    /// Construct a new LayerDetectionAnalyzer with merged rule configuration.
    pub fn new(mut config: ArchitectureConfig) -> Self {
        // Step 1: Index all rules by layer scope (both base + full scoped)
        let mut rules_by_layer: HashMap<String, Vec<&ArchitectureRule>> = HashMap::new();
        for rule in &config.rules {
            let scope = rule.scope.to_string();
            let base_key = if scope.is_empty() {
                String::new()
            } else {
                match scope.split('(').next() {
                    Some(s) => s.to_string(),
                    None => scope.to_string(),
                }
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
            let base_name = match lstr.split('(').next() {
                Some(s) => s.to_string(),
                None => lstr.to_string(),
            };
            // Apply: global rules (key="") + base-layer rules (key=base_name)
            for key in &[String::new(), base_name.clone()] {
                if let Some(rules) = rules_by_layer.get(key.as_str()) {
                    for rule in rules {
                        // Skip specialised scoped rules (e.g. contract(port)) when processing base layers
                        if key.as_str() == base_name && rule.scope.value.contains('(') {
                            continue;
                        }
                        if !rule.exceptions.values.is_empty() {
                            for val in &rule.exceptions.values {
                                if !ldef.exceptions.values.contains(val) {
                                    ldef.exceptions.values.push(val.clone());
                                }
                            }
                        }
                        if !rule.mandatory.values.is_empty() {
                            for val in &rule.mandatory.values {
                                if !ldef.mandatory.values.contains(val) {
                                    ldef.mandatory.values.push(val.clone());
                                }
                            }
                        }
                        if !rule.forbidden.values.is_empty() {
                            for val in &rule.forbidden.values {
                                if !ldef.forbidden.values.contains(val) {
                                    ldef.forbidden.values.push(val.clone());
                                }
                            }
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
                            for val in &rule.code_analysis.forbidden_inheritance.values {
                                if !ldef
                                    .code_analysis
                                    .forbidden_inheritance
                                    .values
                                    .contains(val)
                                {
                                    ldef.code_analysis
                                        .forbidden_inheritance
                                        .values
                                        .push(val.clone());
                                }
                            }
                        }
                        if !rule.orphan.orphan_entry_points.values.is_empty() {
                            for val in &rule.orphan.orphan_entry_points.values {
                                if !ldef.orphan.orphan_entry_points.values.contains(val) {
                                    ldef.orphan.orphan_entry_points.values.push(val.clone());
                                }
                            }
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
                                    for val in &r.exceptions.values {
                                        if !spec_def.exceptions.values.contains(val) {
                                            spec_def.exceptions.values.push(val.clone());
                                        }
                                    }
                                }
                                if !r.forbidden.values.is_empty() {
                                    for val in &r.forbidden.values {
                                        if !spec_def.forbidden.values.contains(val) {
                                            spec_def.forbidden.values.push(val.clone());
                                        }
                                    }
                                }
                                if !r.mandatory.values.is_empty() {
                                    for val in &r.mandatory.values {
                                        if !spec_def.mandatory.values.contains(val) {
                                            spec_def.mandatory.values.push(val.clone());
                                        }
                                    }
                                }
                                if !r.allowed.values.is_empty() {
                                    for val in &r.allowed.values {
                                        if !spec_def.allowed.values.contains(val) {
                                            spec_def.allowed.values.push(val.clone());
                                        }
                                    }
                                }
                                if !r.code_analysis.forbidden_inheritance.values.is_empty() {
                                    for val in &r.code_analysis.forbidden_inheritance.values {
                                        if !spec_def
                                            .code_analysis
                                            .forbidden_inheritance
                                            .values
                                            .contains(val)
                                        {
                                            spec_def
                                                .code_analysis
                                                .forbidden_inheritance
                                                .values
                                                .push(val.clone());
                                        }
                                    }
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
        Self { config, layer_map }
    }
}

impl ILayerDetectionProtocol for LayerDetectionAnalyzer {
    fn detect_layer(&self, file_path: &FilePath, _root_dir: &FilePath) -> Option<LayerNameVO> {
        let filename = Path::new(file_path.value())
            .file_name()
            .and_then(|s| s.to_str())
            .map_or("", |s| s);

        let base = self.extract_layer_from_prefix_str(filename)?;
        Some(self.resolve_specialized_layer(&LayerNameVO::new(&base), file_path))
    }

    /// Look up a `LayerDefinition` by its layer name string.
    /// Falls back to the base layer definition if the specialised key is not found.
    ///
    /// Steps:
    ///   1. Try direct lookup with the full layer name (including parenthesised suffix).
    ///   2. If not found, extract the base name (before the parenthesis) and try again.
    fn get_layer_def(&self, layer: &LayerNameVO) -> Option<LayerDefinition> {
        self.config.layers.get(layer).cloned().or_else(|| {
            let base = match layer.value.split('(').next() {
                Some(s) => s,
                None => &layer.value,
            };
            self.config.layers.get(&LayerNameVO::new(base)).cloned()
        })
    }

    fn get_orphan_entry_points(&self) -> Vec<FilePath> {
        vec![
            FilePath::new("_container.rs".to_string()).unwrap_or_default(),
            FilePath::new("_container.py".to_string()).unwrap_or_default(),
            FilePath::new("_container.ts".to_string()).unwrap_or_default(),
            FilePath::new("_container.js".to_string()).unwrap_or_default(),
            FilePath::new("_entry.rs".to_string()).unwrap_or_default(),
            FilePath::new("_entry.py".to_string()).unwrap_or_default(),
            FilePath::new("_entry.ts".to_string()).unwrap_or_default(),
            FilePath::new("_entry.js".to_string()).unwrap_or_default(),
            FilePath::new("main.rs".to_string()).unwrap_or_default(),
            FilePath::new("lib.rs".to_string()).unwrap_or_default(),
            FilePath::new("main.py".to_string()).unwrap_or_default(),
            FilePath::new("main.ts".to_string()).unwrap_or_default(),
            FilePath::new("main.js".to_string()).unwrap_or_default(),
            FilePath::new("index.ts".to_string()).unwrap_or_default(),
            FilePath::new("index.js".to_string()).unwrap_or_default(),
        ]
    }

    fn config(&self) -> &ArchitectureConfig {
        &self.config
    }

    fn extract_layer_from_prefix(&self, filename: &FilePath) -> Option<LayerNameVO> {
        self.extract_layer_from_prefix_str(filename.value())
            .map(LayerNameVO::new)
    }

    /// Resolve specialised sub-layer from file suffix (e.g., "capabilities(command").
    ///
    /// E.g., `capabilities_user_command.rs` with base_layer="capabilities":
    ///   → stem = "capabilities_user_command", last suffix = "command"
    ///   → checks if "capabilities(command)" is a defined specialised layer
    ///   → if yes, returns "capabilities(command)", else returns "capabilities".
    fn resolve_specialized_layer(
        &self,
        base_layer: &LayerNameVO,
        file_path: &FilePath,
    ) -> LayerNameVO {
        self.resolve_specialized_layer_str(&base_layer.value, file_path.value())
            .map(LayerNameVO::new)
            .unwrap_or_else(|| base_layer.clone())
    }

    fn detect_module_layer(&self, module: &str) -> Option<LayerNameVO> {
        self.detect_module_layer_str(module).map(LayerNameVO::new)
    }

    fn refine_module_layer(&self, base_name: &LayerNameVO, parts: &[&str]) -> LayerNameVO {
        LayerNameVO::new(self.refine_module_layer_str(&base_name.value, parts))
    }
}

impl LayerDetectionAnalyzer {
    fn extract_layer_from_prefix_str(&self, filename: &str) -> Option<String> {
        let stem = Path::new(filename)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();

        for &(prefix, layer) in LAYER_PREFIXES {
            if stem.starts_with(prefix) {
                return Some(layer.to_string());
            }
        }

        None
    }

    fn resolve_specialized_layer_str(&self, base_layer: &str, file_path: &str) -> Option<String> {
        let basename = Path::new(file_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .map_or("", |s| s);

        if let Some(underscore_pos) = basename.rfind('_') {
            let suffix = &basename[underscore_pos + 1..];
            if !suffix.is_empty() {
                let specialized = format!("{}({})", base_layer, suffix);
                let key = LayerNameVO::new(specialized.as_str());
                if self.config.layers.contains_key(&key) {
                    return Some(specialized);
                }
            }
        }

        None
    }

    fn detect_module_layer_str(&self, module: &str) -> Option<String> {
        let meaningful_parts: Vec<&str> = module
            .split([':', '.', '/', '\\'])
            .filter(|p| !p.is_empty())
            .collect();

        if meaningful_parts.is_empty() {
            return None;
        }

        for name in self.config.layers.keys() {
            let base_name = match name.value.split('(').next() {
                Some(s) => s,
                None => &name.value,
            };
            if meaningful_parts.contains(&base_name) {
                return Some(self.refine_module_layer_str(base_name, &meaningful_parts));
            }
        }

        for part in &meaningful_parts {
            if let Some(layer) = self.extract_layer_from_prefix_str(part) {
                return Some(self.refine_module_layer_str(&layer, &meaningful_parts));
            }
        }

        None
    }

    fn refine_module_layer_str(&self, base_name: &str, parts: &[&str]) -> String {
        if let Some(idx) = parts.iter().position(|&p| p == base_name) {
            if idx + 1 < parts.len() {
                let next_part = parts[idx + 1];
                if let Some(underscore_pos) = next_part.rfind('_') {
                    let suffix = &next_part[underscore_pos + 1..];
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
}

impl shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate
    for LayerDetectionAnalyzer
{
    fn detect_layer(&self, file_path: &str, root_dir: &str) -> Option<String> {
        ILayerDetectionProtocol::detect_layer(
            self,
            &FilePath::new(file_path.to_string()).unwrap_or_default(),
            &FilePath::new(root_dir.to_string()).unwrap_or_default(),
        )
        .map(|l| l.value)
    }

    fn get_layer_def(&self, layer: &str) -> Option<LayerDefinition> {
        ILayerDetectionProtocol::get_layer_def(self, &LayerNameVO::new(layer))
    }

    fn get_orphan_entry_points(&self) -> Vec<String> {
        ILayerDetectionProtocol::get_orphan_entry_points(self)
            .into_iter()
            .map(|fp| fp.value)
            .collect()
    }

    fn config(&self) -> &ArchitectureConfig {
        ILayerDetectionProtocol::config(self)
    }
}
