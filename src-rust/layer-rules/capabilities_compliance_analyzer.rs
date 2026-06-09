// arch_compliance_analyzer — Layer detection and config resolution.
// Provides detect_layer/detect_module_layer for identifying which AES layer a file belongs to.

use std::collections::HashMap;
use std::path::Path;

use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
use crate::shared_common::taxonomy_layer_vo::LayerNameVO;
use crate::shared_common::taxonomy_rule_vo::ArchitectureRule;

pub struct ArchComplianceAnalyzer {
    pub config: ArchitectureConfig,
}

impl ArchComplianceAnalyzer {
    pub fn new(mut config: ArchitectureConfig) -> Self {
        // Group rules by layer name — both base name and full scope
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

        let mut new_layers: HashMap<LayerNameVO, LayerDefinition> = HashMap::new();
        for (lname, mut ldef) in config.layers {
            let lstr = lname.to_string();
            let base_name = lstr.split('(').next().unwrap_or(&lstr).to_string();
            // Apply: global rules + base-layer rules
            for key in &[String::new(), base_name.clone()] {
                if let Some(rules) = rules_by_layer.get(key.as_str()) {
                    for rule in rules {
                        // Skip specialized rules (e.g. contract(port)) when processing base layers
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
                        if rule.min_lines.value > 0 {
                            ldef.min_lines = rule.min_lines.clone();
                        }
                        if rule.max_lines.value > 0 {
                            ldef.max_lines = rule.max_lines.clone();
                        }
                        if rule.mandatory_class_definition.value {
                            ldef.mandatory_class_definition =
                                rule.mandatory_class_definition.clone();
                        }
                        if !rule.forbidden_inheritance.values.is_empty() {
                            ldef.forbidden_inheritance = rule.forbidden_inheritance.clone();
                        }
                    }
                }
            }
            new_layers.insert(lname, ldef);
        }

        // Create specialized sub-layer entries from rules (e.g., "agent(container)")
        // This enables resolve_specialized_layer to find and return these sub-layers
        // and apply per-role forbidden/mandatory import rules correctly.
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
                    for suffix in suffixes {
                        let specialized_key =
                            LayerNameVO::new(format!("{}({})", base_name, suffix));
                        if new_layers.contains_key(&specialized_key) {
                            continue;
                        }
                        let mut spec_def = base_def.clone();
                        // Apply specialized rules
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
                                if !r.forbidden_inheritance.values.is_empty() {
                                    spec_def.forbidden_inheritance =
                                        r.forbidden_inheritance.clone();
                                }
                            }
                        }
                        new_layers.insert(specialized_key, spec_def);
                    }
                }
            }
        }

        config.layers = new_layers;
        Self { config }
    }

    /// Detect layer from filename — prioritize prefix-based detection (FRD v1.1),
    /// fallback ke path-based untuk root layer files.
    pub fn detect_layer(&self, file_path: &str, root_dir: &str) -> Option<String> {
        let filename = Path::new(file_path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        // PREFIX-BASED DETECTION (FRD v1.1)
        if let Some(layer) = Self::extract_layer_from_prefix(filename) {
            return Some(self.resolve_specialized_layer(&layer, file_path));
        }

        // FALLBACK: Path-based detection untuk root entry files (cli_main_entry, mcp_main_entry)
        let rel = Self::get_relative_path(file_path, root_dir);

        // PRIORITY: Files at scan root (no subdirectory) with no prefix → root layer.
        // This MUST be checked BEFORE the path-based loop because parse_config_yaml
        // adds path: \".\" to all layers without an explicit path, causing Case 4 of
        // match_layer_nonrecursive to match the first iterated layer (non-deterministic).
        if Path::new(&rel)
            .parent()
            .map(|p| p.to_string_lossy() == "")
            .unwrap_or(false)
        {
            return Some("root".to_string());
        }

        let mut sorted_layers: Vec<(&LayerNameVO, &LayerDefinition)> =
            self.config.layers.iter().collect();
        sorted_layers.sort_by_key(|b| std::cmp::Reverse(b.1.path.value.len()));

        for (name, def) in &sorted_layers {
            if name.value.contains('(') {
                continue;
            }

            let is_match = if def.recursive.value {
                Self::match_layer_recursive(&rel, &def.path.value)
            } else {
                Self::match_layer_nonrecursive(&rel, &def.path.value)
            };

            if is_match {
                return Some(self.resolve_specialized_layer(&name.value, file_path));
            }
        }

        None
    }

    /// Extract layer name dari filename prefix.
    /// e.g. "capabilities_import_checker.rs" → Some("capabilities")
    ///      "surface_command_handler.rs" → Some("surfaces")
    ///      "cli_main_entry.rs" → None (root)
    fn extract_layer_from_prefix(filename: &str) -> Option<String> {
        let stem = Path::new(filename)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        const PREFIX_MAP: &[(&str, &str)] = &[
            ("taxonomy_", "taxonomy"),
            ("contract_", "contract"),
            ("capabilities_", "capabilities"),
            ("infrastructure_", "infrastructure"),
            ("agent_", "agent"),
            ("surface_", "surfaces"),
        ];

        for (prefix, layer) in PREFIX_MAP {
            if stem.starts_with(prefix) {
                return Some(layer.to_string());
            }
        }

        None
    }

    /// Determine which layer a dotted module path belongs to.
    ///
    /// Three strategies (prefix-based first per FRD v1.1):
    ///   1. Direct segment match against layer names.
    ///   2. Prefix-based match: segment starts with layer prefix (e.g. "taxonomy_definition_vo").
    ///   3. Path-based match: module-path-as-filesystem-path contains the layer path.
    pub fn detect_module_layer(&self, module: &str) -> Option<String> {
        let meaningful_parts: Vec<&str> = if module.contains("::") {
            module.split("::").filter(|p| !p.is_empty()).collect()
        } else {
            module.split('.').filter(|p| !p.is_empty()).collect()
        };

        if meaningful_parts.is_empty() {
            return None;
        }

        // 1. Direct match with layer names (ignoring specialisation suffix).
        for name in self.config.layers.keys() {
            let base_name = name.value.split('(').next().unwrap_or(&name.value);
            if meaningful_parts.contains(&base_name) {
                return Some(self.refine_module_layer(base_name, &meaningful_parts));
            }
        }

        // 2. Prefix-based match: segment starts with layer prefix (e.g. "taxonomy_definition_vo").
        for part in &meaningful_parts {
            if let Some(layer) = Self::extract_layer_from_prefix(part) {
                return Some(self.refine_module_layer(&layer, &meaningful_parts));
            }
        }

        // 3. Match with definition paths (e.g. "src/capabilities" → "capabilities").
        let module_as_path = module.replace('.', "/");
        for (name, def) in &self.config.layers {
            let def_path = def.path.value.trim_matches('/');
            if !def_path.is_empty() && module_as_path.contains(def_path) {
                let base_name = name.value.split('(').next().unwrap_or(&name.value);
                return Some(self.refine_module_layer(base_name, &meaningful_parts));
            }
        }

        None
    }

    /// If the file's stem ends with a suffix that corresponds to a specialised layer
    /// (e.g. `user_command.py` → `capabilities(command)`), return that specialised name.
    /// Otherwise return `base_layer` unchanged.
    fn resolve_specialized_layer(&self, base_layer: &str, file_path: &str) -> String {
        let basename = Path::new(file_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        if let Some(underscore_pos) = basename.rfind('_') {
            let suffix = &basename[underscore_pos + 1..];
            if !suffix.is_empty() {
                let specialized = format!("{}({})", base_layer, suffix);
                let key = LayerNameVO::new(specialized.as_str());
                if self.config.layers.contains_key(&key) {
                    return specialized;
                }
            }
        }

        base_layer.to_string()
    }

    /// Given a known base-layer name and the dotted-module parts, try to find a more
    /// specific specialised layer by inspecting the segment immediately after the base name.
    ///
    /// E.g. parts = ["capabilities", "user_command", "UserCommand"] and base = "capabilities"
    ///      → next part is "user_command", suffix = "command"
    ///      → checks if "capabilities(command)" exists in layers
    fn refine_module_layer(&self, base_name: &str, parts: &[&str]) -> String {
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

    /// Recursive match: the relative path starts with the layer path prefix,
    /// or starts with the last path segment of that prefix.
    fn match_layer_recursive(rel: &str, path_def: &str) -> bool {
        let last_segment = path_def.rsplit('/').next().unwrap_or(path_def);
        rel.starts_with(path_def) || rel.starts_with(last_segment)
    }

    /// Non-recursive match: the *parent directory* of the relative path equals the layer path.
    ///
    /// Also handles the case where the analysis is run from inside the layer directory
    /// (rel is just a filename, parent is ".").
    fn match_layer_nonrecursive(rel: &str, path_def: &str) -> bool {
        let norm_path_def = path_def.trim_end_matches('/');

        let parent_dir = match Path::new(rel).parent().and_then(|p| p.to_str()) {
            Some("") => ".",
            Some(p) => p.trim_end_matches('/'),
            None => ".",
        };

        // Case 1: Standard match (rel is "src/capabilities/foo.py", path_def is "src/capabilities")
        if parent_dir == norm_path_def {
            return true;
        }

        // Case 2: Running analysis from inside the layer directory
        // (rel is "foo.py", parent is ".", layer path has an actual value)
        if parent_dir == "." && !norm_path_def.is_empty() && norm_path_def != "." {
            return true;
        }

        // Case 3: rel has no directory but layer path appears as a suffix of rel
        if parent_dir == "." && rel.ends_with(norm_path_def) {
            return true;
        }

        // Case 4: File at scan root (no subdirectory) matches non-recursive layer.
        // This case is needed because detect_source_dir descends into src-rust/ before
        // scanning, so a root file like "root_violation.rs" has no "src-rust/" prefix
        // in its relative path. Without this, root layer files never match any layer
        // definition and are skipped entirely by the architectural checkers.
        if parent_dir == "." && !norm_path_def.is_empty() {
            return true;
        }

        false
    }

    /// Look up a `LayerDefinition` by its layer name string.
    pub fn get_layer_def(&self, layer: &str) -> Option<&LayerDefinition> {
        self.config
            .layers
            .get(&LayerNameVO::new(layer))
            .or_else(|| {
                let base = layer.split('(').next().unwrap_or(layer);
                self.config.layers.get(&LayerNameVO::new(base))
            })
    }

    /// Compute the path of `file_path` relative to `root_dir`.
    /// Falls back to the normalised absolute path when no prefix match is found.
    fn get_relative_path(file_path: &str, root_dir: &str) -> String {
        let normalized_file = file_path.replace('\\', "/");
        let normalized_root = root_dir.trim_end_matches('/').replace('\\', "/");
        if normalized_file.starts_with(&normalized_root) {
            normalized_file[normalized_root.len()..]
                .trim_start_matches('/')
                .to_string()
        } else {
            normalized_file
        }
    }
}
