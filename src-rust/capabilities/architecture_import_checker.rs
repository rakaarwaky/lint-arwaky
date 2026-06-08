// arch_import_checker — Import-related architectural checks.
// Implements IArchImportProtocol: check_mandatory_imports, check_forbidden_imports, check_legacy_import_rules.

use crate::taxonomy::{
    AdapterName, ArchitectureConfig, ColumnNumber, ErrorCode, FilePath, LayerDefinition,
    LineNumber, LintMessage, LintResult, LocationList, ScopeRef, Severity,
};
use std::fs;
use std::path::Path;

pub struct ArchImportRuleChecker;

impl ArchImportRuleChecker {
    pub fn new() -> Self {
        Self
    }

    /// Resolve a scope value (e.g. "contract(protocol)", "taxonomy(entity,error,event)",
    /// "contract(port|protocol|aggregate)", "taxonomy") into layer + suffix matches.
    /// Returns (layer_name, suffixes) where suffixes is empty if no suffix restriction.
    fn resolve_scope(scope: &str) -> (&str, Vec<&str>) {
        if let Some(paren) = scope.find('(') {
            let layer = scope[..paren].trim();
            let inner = scope[paren + 1..].trim_end_matches(')').trim();
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
            (layer, suffixes)
        } else {
            (scope.trim(), vec![])
        }
    }

    /// Check if an import line satisfies the given scope requirement.
    /// e.g. scope "contract(protocol)" matches "use crate::contract::some_protocol::X"
    fn import_matches_scope(import_line: &str, layer: &str, suffixes: &[&str]) -> bool {
        let lower = import_line.to_lowercase();
        let layer_match =
            lower.contains(&format!("{}::", layer)) || lower.contains(&format!("::{}::", layer));
        if !layer_match {
            return false;
        }
        if suffixes.is_empty() {
            return true;
        }
        suffixes.iter().any(|s| {
            // Snake_case: "service_container_aggregate" contains "_aggregate"
            if lower.contains(&format!("_{}", s)) || lower.contains(&format!("::{}", s)) {
                return true;
            }
            // PascalCase / barrel import: "ServiceContainerAggregate" ends with "Aggregate" (case-insensitive)
            // Split by :: and check each identifier segment using original case for PascalCase detection
            import_line.split("::").any(|seg| {
                let cleaned = seg
                    .trim_end_matches(';')
                    .trim()
                    .trim_start_matches('{')
                    .trim_end_matches('}')
                    .trim();
                cleaned.split(',').any(|t| {
                    let name = t.trim();
                    let name_lower = name.to_lowercase();
                    if let Some(rest) = name_lower.strip_suffix(s) {
                        // Exact match or snake_case (preceded by _)
                        if rest.is_empty() || rest.ends_with('_') {
                            return true;
                        }
                        // PascalCase: suffix starts with uppercase in original name
                        if name.len() >= s.len() {
                            let suffix_in_orig = &name[name.len() - s.len()..];
                            if suffix_in_orig.starts_with(|c: char| c.is_uppercase()) {
                                return true;
                            }
                        }
                    }
                    false
                })
            })
        })
    }

    fn make_result(file: &str, line: i64, code: &str, msg: &str, sev: Severity) -> LintResult {
        LintResult {
            file: FilePath::new(file.to_string())
                .unwrap_or_else(|_| FilePath::new(".").unwrap_or_default()),
            line: LineNumber::new(line),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw(code),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: sev,
            enclosing_scope: Some(ScopeRef {
                name: crate::taxonomy::DescriptionVO::new(String::new()),
                kind: crate::taxonomy::DescriptionVO::new(String::new()),
                file: None,
                start_line: None,
                end_line: None,
            }),
            related_locations: LocationList::new(),
        }
    }

    fn get_basename(file: &str) -> String {
        Path::new(file)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("")
            .to_string()
    }

    fn read_import_lines(file: &str) -> Vec<(usize, String)> {
        let Ok(content) = fs::read_to_string(file) else {
            return vec![];
        };
        let mut result: Vec<(usize, String)> = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        while i < lines.len() {
            let trimmed = lines[i].trim();
            if trimmed.starts_with("import ")
                || trimmed.starts_with("from ")
                || trimmed.starts_with("extern crate ")
            {
                result.push((i + 1, lines[i].to_string()));
                i += 1;
                continue;
            }
            if trimmed.starts_with("use ") {
                let mut combined = lines[i].to_string();
                // Handle multi-line `use foo::{ ... }` blocks
                if combined.contains('{') && !combined.contains('}') {
                    let start = i;
                    i += 1;
                    while i < lines.len() {
                        let part = lines[i].trim().to_string();
                        combined.push_str(&format!(" {}", part));
                        if part.contains('}') || combined.ends_with(';') {
                            break;
                        }
                        i += 1;
                    }
                    // Collapse whitespace for matching
                    combined = combined.split_whitespace().collect::<Vec<&str>>().join(" ");
                    result.push((start + 1, combined));
                } else if !combined.ends_with(';') {
                    // Handle line continuation with trailing comma/backslash
                    while i + 1 < lines.len() {
                        let next = lines[i + 1].trim();
                        if next.starts_with("use ") || next.is_empty() {
                            break;
                        }
                        combined.push_str(&format!(" {}", next));
                        if next.ends_with(';') {
                            i += 1;
                            break;
                        }
                        i += 1;
                    }
                    combined = combined.split_whitespace().collect::<Vec<&str>>().join(" ");
                    result.push((i + 1, combined));
                } else {
                    result.push((i + 1, combined));
                }
            }
            i += 1;
        }
        result
    }

    fn extract_module_from_line(line: &str) -> Option<String> {
        let trimmed = line.trim();
        // Python: `from x import y` or `import x`
        if let Some(rest) = trimmed.strip_prefix("from ") {
            let module = rest.split_whitespace().next()?.to_string();
            return Some(module);
        }
        if let Some(rest) = trimmed.strip_prefix("import ") {
            let module = rest.split_whitespace().next()?.to_string();
            return Some(module);
        }
        // Rust: `use x::y;` or `use x::{a, b, c};`
        if let Some(rest) = trimmed.strip_prefix("use ") {
            let module = rest.trim_end_matches(';').trim().to_string();
            // For multi-line imports like `crate::foo::{a, b}`, extract just `crate::foo`
            if let Some(brace_pos) = module.find("::{") {
                return Some(module[..brace_pos].to_string());
            }
            return Some(module);
        }
        None
    }

    /// Check mandatory imports: file must import from the required layers.
    pub fn check_mandatory_imports(
        &self,
        file: &str,
        definition: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        if definition.mandatory_import.values.is_empty() {
            return;
        }

        let basename = Self::get_basename(file);
        if basename == "__init__.py" {
            return;
        }

        if definition.exceptions.values.contains(&basename) {
            return;
        }

        let import_lines = Self::read_import_lines(file);
        let Ok(content) = fs::read_to_string(file) else {
            return;
        };

        for required in &definition.mandatory_import.values {
            let (layer, suffixes) = Self::resolve_scope(required);
            let is_present = if suffixes.is_empty() {
                content.contains(layer) || import_lines.iter().any(|(_, l)| l.contains(layer))
            } else {
                // Check import lines AND verify via barrel (mod.rs/__init__.py) that
                // the imported type actually originates from a file with the required suffix.
                let line_matches = import_lines
                    .iter()
                    .any(|(_, l)| Self::import_matches_scope(l, layer, &suffixes));
                if line_matches {
                    true
                } else {
                    // Fallback: check if any imported type from this layer comes from
                    // a file with the required suffix by inspecting the barrel file.
                    // For flat layers like taxonomy, barrel re-exports tell us the source suffix.
                    let barrel_matches = import_lines.iter().any(|(_, l)| {
                        l.contains(&format!("{}::", layer)) || l.contains(&format!("{}.", layer))
                    }) && {
                        // Extract type names from imports and check barrel for source suffix
                        Self::barrel_has_suffix_match(file, layer, &suffixes, &import_lines)
                    };
                    barrel_matches
                }
            };

            // Skip mandatory import if file genuinely doesn't use ANY
            // types/identifiers from this layer (not even inline qualifiers).
            // Prevents forcing unused imports just to satisfy AES002.
            let genuinely_unreferenced = if suffixes.is_empty() {
                !content.contains(layer) && !import_lines.iter().any(|(_, l)| l.contains(layer))
            } else {
                !content.contains(layer) && !suffixes.iter().any(|s| content.contains(s))
            };

            if genuinely_unreferenced {
                continue;
            }

            if !is_present {
                let msg = if !definition
                    .mandatory_import_violation_message
                    .value
                    .is_empty()
                {
                    definition.mandatory_import_violation_message.value.clone()
                } else {
                    format!(
                        "AES002 MANDATORY_IMPORT: Missing required import: '{}'.",
                        required
                    )
                };
                violations.push(Self::make_result(file, 0, "AES002", &msg, Severity::HIGH));
            }
        }
    }

    /// Check forbidden imports: file must NOT import from forbidden layers.
    pub fn check_forbidden_imports(
        &self,
        file: &str,
        layer_name: &str,
        definition: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        if definition.forbidden_import.values.is_empty() {
            return;
        }

        let import_lines = Self::read_import_lines(file);

        for (line_num, line) in &import_lines {
            if let Some(module) = Self::extract_module_from_line(line) {
                for forbidden in &definition.forbidden_import.values {
                    let (layer, suffixes) = Self::resolve_scope(forbidden);
                    let is_forbidden = if suffixes.is_empty() {
                        module.contains(forbidden.as_str()) || module.contains(layer)
                    } else {
                        // Barrel-aware suffix check: verify via barrel that the type
                        // actually originates from a file with the forbidden suffix.
                        // E.g., ServiceContainerAggregate ends with "aggregate" not "port",
                        // so contract(port) should NOT flag it.
                        let suffix_match = Self::import_matches_scope(line, layer, &suffixes);
                        if !suffix_match {
                            false
                        } else {
                            // Verify with barrel — if barrel says the type has a DIFFERENT
                            // suffix, it's not actually forbidden.
                            let barrel_confirms =
                                Self::barrel_confirms_forbidden(file, layer, &suffixes, line);
                            // If barrel confirms the type has this suffix → forbidden
                            // If barrel disagrees → not forbidden (use barrel as source of truth)
                            barrel_confirms
                        }
                    };
                    if is_forbidden {
                        let msg = if !definition
                            .forbidden_import_violation_message
                            .value
                            .is_empty()
                        {
                            definition.forbidden_import_violation_message.value.clone()
                        } else {
                            format!(
                                "AES001 FORBIDDEN_IMPORT: Layer '{}' is importing from forbidden module '{}'.",
                                layer_name, module
                            )
                        };
                        violations.push(Self::make_result(
                            file,
                            *line_num as i64,
                            "AES001",
                            &msg,
                            Severity::CRITICAL,
                        ));
                    }
                }
            }
        }
    }

    /// Check legacy governance rules: cross-layer import restrictions.
    pub fn check_legacy_import_rules(
        &self,
        file: &str,
        file_layer: &str,
        config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    ) {
        if config.governance_rules.is_empty() {
            return;
        }

        // Skip agent layer files
        if file_layer == "agent" {
            return;
        }

        let import_lines = Self::read_import_lines(file);

        for (line_num, line) in &import_lines {
            if let Some(module) = Self::extract_module_from_line(line) {
                // Determine target layer from module path
                let target_layer = self.detect_module_layer(&module, config);

                if let Some(target) = target_layer {
                    for rule in config.governance_rules.iter() {
                        let source_matches = rule.source_layer.value == file_layer;
                        let target_matches = rule.forbidden_target.value == target;

                        if source_matches && target_matches {
                            let desc = if !rule.description.value.is_empty() {
                                rule.description.value.clone()
                            } else {
                                "Forbidden layer import detected.".to_string()
                            };
                            let msg = format!(
                                "[AES Layer Violation] {}. File in '{}' imports from '{}' via '{}'.",
                                desc, file_layer, target, module
                            );
                            violations.push(Self::make_result(
                                file,
                                *line_num as i64,
                                "AES001",
                                &msg,
                                Severity::CRITICAL,
                            ));
                            break;
                        }
                    }
                }
            }
        }
    }

    fn detect_module_layer(&self, module: &str, config: &ArchitectureConfig) -> Option<String> {
        let parts: Vec<&str> = module.split('.').collect();
        for part in &parts {
            for (name, def) in &config.layers {
                let path_last = def.path.value.split('/').last().unwrap_or("");
                if *part == name.value.as_str() || *part == path_last {
                    return Some(name.value.clone());
                }
            }
        }
        None
    }

    /// Check if the barrel file for `layer` re-exports any type from a file
    /// whose suffix matches one of the required `suffixes`.
    /// E.g., for taxonomy(vo), reads taxonomy/mod.rs and finds that ErrorMessage
    /// comes from common_error_vo.rs → suffix "vo" → match!
    fn barrel_has_suffix_match(
        file: &str,
        layer: &str,
        suffixes: &[&str],
        import_lines: &[(usize, String)],
    ) -> bool {
        let type_suffix = Self::build_type_suffix_map(file, layer);
        if type_suffix.is_empty() {
            return false;
        }
        // Check if any imported type from this layer has the required suffix
        for (_, line) in import_lines {
            if !line.contains(&format!("{}::", layer)) && !line.contains(&format!("{}.", layer)) {
                continue;
            }
            if Self::line_imports_suffix_type(line, &type_suffix, suffixes) {
                return true;
            }
        }
        false
    }

    /// Check if a SPECIFIC type in an import line has the forbidden suffix
    /// by resolving through the barrel's type→suffix map.
    fn barrel_confirms_forbidden(file: &str, layer: &str, suffixes: &[&str], line: &str) -> bool {
        let type_suffix = Self::build_type_suffix_map(file, layer);
        if type_suffix.is_empty() {
            // Can't verify through barrel — trust the original suffix match
            return true;
        }
        Self::line_imports_suffix_type(line, &type_suffix, suffixes)
    }

    /// Extract type names from an import line and check if any have the required suffix.
    fn line_imports_suffix_type(
        line: &str,
        type_suffix: &std::collections::HashMap<String, String>,
        suffixes: &[&str],
    ) -> bool {
        let mut rest: &str = line.trim().trim_end_matches(';');
        if let Some(pos) = rest.rfind("::") {
            rest = &rest[pos + 2..].trim();
            let types_str = rest
                .strip_prefix('{')
                .unwrap_or(rest)
                .strip_suffix('}')
                .unwrap_or(rest);
            for type_name in types_str.split(',') {
                let tn = type_name.trim().to_string();
                if tn.is_empty() {
                    continue;
                }
                if let Some(src_suffix) = type_suffix.get(&tn) {
                    if suffixes.contains(&src_suffix.as_str()) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Build type→source_suffix map from barrel file (mod.rs/__init__.py).
    /// E.g., "pub use common_error_vo::ErrorMessage" → ("ErrorMessage", "vo")
    fn build_type_suffix_map(file: &str, layer: &str) -> std::collections::HashMap<String, String> {
        let file_path = std::path::Path::new(file);
        let parent = file_path.parent().unwrap_or(std::path::Path::new("."));
        let layer_root = parent
            .ancestors()
            .find(|a| a.ends_with(layer))
            .unwrap_or(parent);
        let barrel_names = ["mod.rs", "__init__.py", "index.ts", "index.js"];
        let barrel = barrel_names
            .iter()
            .map(|n| layer_root.join(n))
            .find(|p| p.exists());
        let Some(barrel_path) = barrel else {
            return std::collections::HashMap::new();
        };
        let Ok(content) = std::fs::read_to_string(&barrel_path) else {
            return std::collections::HashMap::new();
        };

        let mut type_suffix: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        while i < lines.len() {
            let trimmed = lines[i].trim();
            if trimmed.starts_with("pub use ") {
                let rest = trimmed.trim_start_matches("pub use ");
                if let Some(module_end) = rest.find("::") {
                    let module = &rest[..module_end];
                    let module_suffix = module.rsplit('_').next().unwrap_or("").to_string();
                    let mut types_part = rest[module_end + 2..].to_string();
                    if types_part.ends_with(';') {
                        types_part = types_part[..types_part.len() - 1].to_string();
                    }
                    if types_part.contains('{') && !types_part.contains('}') {
                        let mut block_lines = types_part;
                        i += 1;
                        while i < lines.len() {
                            let cont = lines[i].trim();
                            block_lines.push_str(cont);
                            if cont.contains('}') {
                                break;
                            }
                            i += 1;
                        }
                        types_part = block_lines;
                    }
                    if types_part.starts_with('{') {
                        types_part = types_part[1..].to_string();
                    }
                    if types_part.ends_with('}') {
                        types_part = types_part[..types_part.len() - 1].to_string();
                    }
                    for type_name in types_part.split(',') {
                        let tn = type_name.trim().to_string();
                        if !tn.is_empty() && !module_suffix.is_empty() {
                            type_suffix.insert(tn, module_suffix.clone());
                        }
                    }
                }
            }
            i += 1;
        }
        type_suffix
    }
}
