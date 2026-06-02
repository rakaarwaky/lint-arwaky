// arch_compliance_analyzer — Core layer-detection and compliance orchestration.
// 1:1 Rust port of capabilities/arch_compliance_analyzer.py

use super::arch_import_checker::ArchImportRuleChecker;
use super::arch_metric_checker::ArchMetricChecker;
use crate::taxonomy::{
    AdapterName, ArchitectureConfig, ColumnNumber, ErrorCode, FilePath, LayerDefinition,
    LayerNameVO, LineNumber, LintMessage, LintResult, LocationList, ScopeRef, Severity,
};
use std::path::Path;

pub struct ArchComplianceAnalyzer {
    pub config: ArchitectureConfig,
}

impl ArchComplianceAnalyzer {
    pub fn new(config: ArchitectureConfig) -> Self {
        Self { config }
    }

    /// Orchestrate all compliance checks for the given file list under root_dir.
    /// Mirrors Python `execute()` — skips barrel files, detects layers, runs sub-checkers.
    pub fn execute(&self, files: &[String], root_dir: &str) -> Vec<LintResult> {
        if !self.config.enabled {
            return Vec::new();
        }

        let mut violations: Vec<LintResult> = Vec::new();
        let import_checker = ArchImportRuleChecker::new();
        let metric_checker = ArchMetricChecker::new();

        for file in files {
            let filename = Path::new(file)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");

            // Skip barrel / entry-point files at orchestrator level
            if Self::is_barrel_file(filename) {
                continue;
            }

            let layer = match self.detect_layer(file, root_dir) {
                Some(l) => l,
                None => continue,
            };

            let def = match self.get_layer_def(&layer) {
                Some(d) => d,
                None => continue,
            };

            // Respect per-layer exception list
            if def.exceptions.values.contains(&filename.to_string()) {
                continue;
            }

            import_checker.check_mandatory_imports(file, def, &mut violations);
            import_checker.check_forbidden_imports(file, &layer, def, &mut violations);
            import_checker.check_legacy_import_rules(file, &layer, &self.config, &mut violations);
            metric_checker.check_line_counts(file, Some(def), &mut violations);
            metric_checker.check_mandatory_class_definition(file, Some(def), &mut violations);
        }

        violations
    }

    /// Determine which layer (if any) a file belongs to.
    ///
    /// Layers are tried in order from most-specific path (longest) to least-specific.
    /// Specialized layers (e.g. `capabilities(command)`) are excluded from the base-layer
    /// scan and resolved in a second pass via `resolve_specialized_layer`.
    pub fn detect_layer(&self, file_path: &str, root_dir: &str) -> Option<String> {
        let rel = Self::get_relative_path(file_path, root_dir);

        // Sort by path-length descending so longer (more specific) paths win.
        let mut sorted_layers: Vec<(&LayerNameVO, &LayerDefinition)> =
            self.config.layers.iter().collect();
        sorted_layers.sort_by(|a, b| b.1.path.value.len().cmp(&a.1.path.value.len()));

        for (name, def) in &sorted_layers {
            // Skip already-specialised entries like "capabilities(command)"
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

    /// Determine which layer a dotted module path belongs to.
    ///
    /// Two strategies (Python parity):
    ///   1. Direct segment match against layer names.
    ///   2. Path-based match: module-path-as-filesystem-path contains the layer path.
    pub fn detect_module_layer(&self, module: &str) -> Option<String> {
        let meaningful_parts: Vec<&str> = module.split('.').filter(|p| !p.is_empty()).collect();

        if meaningful_parts.is_empty() {
            return None;
        }

        // 1. Direct match with layer names (ignoring specialisation suffix).
        for (name, _) in &self.config.layers {
            let base_name = name.value.split('(').next().unwrap_or(&name.value);
            if meaningful_parts.contains(&base_name) {
                return Some(self.refine_module_layer(base_name, &meaningful_parts));
            }
        }

        // 2. Match with definition paths (e.g. "src/capabilities" → "capabilities").
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
            Some(p) if p.is_empty() => ".",
            Some(p) => p.trim_end_matches('/'),
            None => ".",
        };

        // Case 1: Standard match (rel is "src/capabilities/foo.py", path_def is "src/capabilities")
        if parent_dir == norm_path_def {
            return true;
        }

        // Case 2: Running analysis from inside the layer directory
        // (rel is "foo.py", parent is ".", layer path is "" or ".")
        if parent_dir == "." && (norm_path_def.is_empty() || norm_path_def == ".") {
            return true;
        }

        // Case 3: rel has no directory but layer path appears as a suffix of rel
        if parent_dir == "." && rel.ends_with(norm_path_def) {
            return true;
        }

        false
    }

    /// Look up a `LayerDefinition` by its layer name string.
    pub fn get_layer_def(&self, layer: &str) -> Option<&LayerDefinition> {
        self.config.layers.get(&LayerNameVO::new(layer))
    }

    /// Returns true for conventional barrel / re-export files.
    fn is_barrel_file(filename: &str) -> bool {
        matches!(filename, "__init__.py" | "mod.rs" | "index.ts" | "index.js")
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

    fn make_result(file: &str, line: i64, code: &str, msg: &str, sev: Severity) -> LintResult {
        LintResult {
            file: FilePath::new(file.to_string()),
            line: LineNumber::new(line),
            column: ColumnNumber::new(0),
            code: ErrorCode::new(code),
            message: LintMessage::new(msg),
            source: AdapterName::new("architecture"),
            severity: sev,
            enclosing_scope: ScopeRef {
                name: "".to_string(),
                kind: "".to_string(),
                file: FilePath::new(""),
                start_line: LineNumber::new(0),
                end_line: LineNumber::new(0),
            },
            related_locations: LocationList::new(Vec::new()),
        }
    }
}
